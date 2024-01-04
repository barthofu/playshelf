use rocket::{launch, catchers, routes};
use rocket_okapi::{openapi_get_routes, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};

use playshelf_server::{routes::{self, errors}, middlewares::cors::Cors};
use playshelf_server::utils::database::Db;

fn get_docs() -> SwaggerUIConfig {

    SwaggerUIConfig {
        url: "../api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {

    println!("Starting server...");
    
    rocket::build()
        .attach(Cors)
        .attach(Db::fairing())
        .register("/", catchers![errors::default])
        .mount("/api", openapi_get_routes![
            
            routes::misc::index,

            routes::tracks::get,
            routes::tracks::get_all, 
            routes::tracks::create,
            routes::tracks::update,
            routes::tracks::delete,

            routes::artists::get,
            routes::artists::get_all, 
            routes::artists::create,
            routes::artists::update,
            routes::artists::delete,
            
            routes::artists::get_tracks,
            routes::artists::associate_track,
            routes::artists::dissociate_track,

        ])
        .mount("/api", routes![
                
                routes::audio::stream,
        ])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}