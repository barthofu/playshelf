use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_sync_db_pools::{diesel::SqliteConnection, database};

#[database("sqlite")]
pub struct Db(SqliteConnection);

impl<'r> OpenApiFromRequest<'r> for Db {

    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}