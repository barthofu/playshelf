use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use rocket_okapi::{JsonSchema};

use crate::schema::{artist, artist_tracks, artist_albums};
use super::{album::Album, track::Track};

#[derive(JsonSchema, Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Identifiable)]
#[table_name = "artist"]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>
}

#[derive(JsonSchema, Insertable, Deserialize)]
#[table_name = "artist"]
pub struct NewArtist {
    pub name: String,
    pub icon: Option<String>
}

#[derive(JsonSchema, AsChangeset, Deserialize)]
#[table_name = "artist"]
pub struct UpdateArtist {
    pub name: Option<String>,
    pub icon: Option<String>
}

/* associations */

#[derive(JsonSchema, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Artist, foreign_key = "artist_id")]
#[belongs_to(Track, foreign_key = "track_id")]
pub struct ArtistTrack {
    pub id: i32,
    pub track_id: i32,
    pub artist_id: i32,
}

#[derive(JsonSchema, Insertable)]
#[table_name = "artist_tracks"]
pub struct NewArtistTrack {
    pub track_id: i32,
    pub artist_id: i32,
}


#[derive(JsonSchema, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Artist, foreign_key = "artist_id")]
#[belongs_to(Album, foreign_key = "album_id")]
pub struct ArtistAlbum {
    pub id: i32,
    pub album_id: i32,
    pub artist_id: i32,
}

#[derive(JsonSchema, Insertable)]
#[table_name = "artist_albums"]
pub struct NewArtistAlbum {
    pub album_id: i32,
    pub artist_id: i32,
}