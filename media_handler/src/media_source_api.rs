use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

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
    pub fn get_media_list(&self) -> Vec<PathBuf> {
        match self {
            Self::Local(m) => m.get_media_list(),
            Self::DB(m) => m.get_media_list(),
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

    pub fn get_media_list(&self) -> Vec<PathBuf> {
        let mut rng = thread_rng();
        let mut x = self.media_paths.clone();
        x.shuffle(&mut rng);
        x
        // vec![
        //     PathBuf::from("data/init/loading.jpeg"),
        //     PathBuf::from("data/init/loading.jpeg"),
        //     PathBuf::from("data/init/loading.jpeg"),
        // ]
    }
}

pub struct PostgreSQLMedia {
    connection: Arc<Mutex<PgConnection>>,
}

impl PostgreSQLMedia {
    fn query_data(&self) -> Vec<PathBuf> {
        let mut conn = self.connection.lock().unwrap();

        let formats = vec!["PNG", "JPEG"];
        let wanted_formats = format::table
            .filter(format::name.eq_any(formats))
            .select(Format::as_select())
            .load(&mut *conn)
            .expect("Failed request");

        let tags = vec!["TEST", "oUI"];
        // media with a specific format AND a specific tag
        let medias_queue: Vec<PathBuf> = Media::belonging_to(&wanted_formats)
            .inner_join(tag::table.on(tag::name.eq_any(tags)))
            .select(Media::as_select())
            .load(&mut *conn)
            .expect("Failed request")
            .into_iter()
            .map(|m| PathBuf::from(m.url))
            .collect();

        medias_queue
    }

    pub fn new(config: MediaConfig) -> Self {
        let connection = PgConnection::establish(&config.database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url));

        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn get_media_list(&self) -> Vec<PathBuf> {
        self.query_data()
    }
}
