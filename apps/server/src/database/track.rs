use diesel::prelude::*;

use crate::{utils::macros, models::track::{NewTrack, Track, UpdateTrack}};

// basic CRUD operations

macros::crud_database::resource::find_one!(track, Track);
macros::crud_database::resource::find_all!(track, Track);
macros::crud_database::resource::create!(track, Track, NewTrack);
macros::crud_database::resource::update!(track, Track, UpdateTrack);
macros::crud_database::resource::delete!(track);

// custom operations