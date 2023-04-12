use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::PathBuf;

use diesel;

use crate::schema::*;
use crate::sql_models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::media_config::MediaConfig;

pub enum MediaSource {
    Local(LocalMedia),
    DB(PostgreSQLMedia),
}

/*
    Strategy:

    fn get_media_list<F>(media_source: &MediaSource, config: &MediaConfig, f: F) -> Vec<PathBuf>
    where
        F: Fn(&MediaSource, &MediaConfig) -> Vec<PathBuf>,
    {
        f(media_source, config)
    }

*/

impl MediaSource {
    pub fn get_media_list(&mut self, config: &MediaConfig) -> Vec<PathBuf> {
        match self {
            Self::Local(m) => m.get_media_list(&config),
            Self::DB(m) => m.get_media_list(&config),
        }
    }
}

pub struct LocalMedia {
    media_paths: Vec<PathBuf>,
}

impl LocalMedia {
    fn get_media_paths(folder_path: &PathBuf) -> Vec<PathBuf> {
        fs::read_dir(folder_path)
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|f| f.is_file())
            .collect()
    }

    pub fn new(config: &MediaConfig) -> Self {
        Self {
            media_paths: Self::get_media_paths(&config.data_folder),
        }
    }

    pub fn get_media_list(&mut self, config: &MediaConfig) -> Vec<PathBuf> {
        let mut rng = thread_rng();
        let mut x = self.media_paths.clone();
        x.shuffle(&mut rng);
        x
    }
}

pub struct PostgreSQLMedia {
    connection: PgConnection,
}

impl PostgreSQLMedia {
    fn query_data(&mut self) -> Vec<PathBuf> {
        let wanted_formats = format::table
            .filter(format::name.eq("JPEG"))
            .select(Format::as_select())
            .load(&mut self.connection)
            .expect("Failed request");

        let wanted_tags = tag::table
            .filter(tag::name.eq("TEST"))
            .select(Tag::as_select())
            .load(&mut self.connection)
            .expect("Failed request");

        let media_tags = media::table
            .select(Media::as_select())
            .load(&mut self.connection)
            .expect("Failed request");

        // let medias = Media::belonging_to(&wanted_formats)
        //     .inner_join()
        //     .select(Media::as_select())
        //     .load(&mut self.connection)
        //     .expect("Failed request");

        // let medias: Vec<(Tag, Vec<Media>)> = tags
        //     .into_iter()
        //     .zip(tags)
        //     .map(|(m, t)| (m, t.into_iter().map(|(_, tag)| tag).collect()))
        //     .collect();

        println!("Displaying {} wanted_formats", wanted_formats.len());
        for r in wanted_formats {
            println!("{:?}", r);
        }

        println!("Displaying {} wanted_tags", wanted_tags.len());
        for r in wanted_tags {
            println!("{:?}", r);
        }

        println!("Displaying {} media_tags", media_tags.len());
        for r in media_tags {
            println!("{:?}", r);
        }

        // println!("Displaying {} medias", medias.len());
        // for r in medias {
        //     println!("{:?}", r);
        // }
        vec![]
    }

    pub fn new(config: &MediaConfig) -> Self {
        let connection = PgConnection::establish(&config.database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url));

        Self { connection }
    }

    pub fn get_media_list(&mut self, config: &MediaConfig) -> Vec<PathBuf> {
        self.query_data()
    }
}
