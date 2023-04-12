use crate::schema::{format, media, tag};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Format))]
#[diesel(belongs_to(Tag))]
#[diesel(primary_key(format_id, tag_id))]
#[diesel(table_name = media)]
pub struct Media {
    pub url: String,
    pub format_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = format)]
pub struct Format {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = tag)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}
