use rocket_okapi::openapi;
use rocket::{get, post, delete, patch};
use rocket::serde::json::Json;

use crate::models::track::{NewTrack, Track, UpdateTrack};
use crate::utils::database::{Db};
use crate::utils::{response, macros};

// crud handlers

macros::crud_handlers::resource::get!("/tracks/<track_id>", track_id, track, Track);
macros::crud_handlers::resource::get_all!("/tracks", track, Track);
macros::crud_handlers::resource::create!("/tracks", track, Track, NewTrack);
macros::crud_handlers::resource::update!("/tracks/<track_id>", track_id, track, Track, UpdateTrack);
macros::crud_handlers::resource::delete!("/tracks/<track_id>", track_id, track);
