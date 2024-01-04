use rocket_okapi::openapi;
use rocket::{get, post, delete, patch};
use rocket::serde::json::Json;

use crate::models::artist::{NewArtist, Artist, UpdateArtist, ArtistTrack};
use crate::models::track::Track;
use crate::utils::database::{Db};
use crate::utils::{response, macros};

// crud handlers

macros::crud_handlers::resource::get!("/artists/<artist_id>", artist_id, artist, Artist);
macros::crud_handlers::resource::get_all!("/artists", artist, Artist);
macros::crud_handlers::resource::create!("/artists", artist, Artist, NewArtist);
macros::crud_handlers::resource::update!("/artists/<artist_id>", artist_id, artist, Artist, UpdateArtist);
macros::crud_handlers::resource::delete!("/artists/<artist_id>", artist_id, artist);

// associations handlers

macros::crud_handlers::association::get!(
    get_tracks, "/artists/<artist_id>/tracks", artist_id, artist, Track
);

macros::crud_handlers::association::associate!(
    associate_track, "/artists/<artist_id>/tracks/<track_id>", artist_id, track_id, artist, track, ArtistTrack
);

macros::crud_handlers::association::dissociate!(
    dissociate_track, "/artists/<artist_id>/tracks/<track_id>", artist_id, track_id, artist, track
);