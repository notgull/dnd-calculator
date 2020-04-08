/*
 * Copyright 2020 not_a_seagull
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the Software without restriction, including without
 * limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the
 * Software, and to permit persons to whom the Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
 * CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#[macro_use]
extern crate diesel;

#[macro_use]
mod macros;

pub(crate) mod models;
pub(crate) mod schema;

mod character;

use actix_files::NamedFile;
use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result as ActixResult,
};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use log::info;
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};

async fn get_file(req: HttpRequest) -> ActixResult<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path: PathBuf = ["../dist/", path.to_str().unwrap()].iter().collect();
    Ok(NamedFile::open(path)?)
}

async fn get_root() -> ActixResult<NamedFile> {
    Ok(NamedFile::open("../dist/index.html")?)
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // initialize specific info
    env::set_var("RUST_LOG", "actix_web=debug,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up diesel connection
    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    // the server's url
    let bind = "127.0.0.1:8450";
    info!("Starting dnd_calculator at {}", &bind);

    // start actix server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route(
                "/api/create_template",
                web::post().to(character::add_template),
            )
            .route(
                "/api/list_templates",
                web::post().to(character::list_templates),
            )
            .route("/api/get_template", web::post().to(character::get_template))
            .route(
                "/api/edit_template",
                web::post().to(character::edit_template),
            )
            .route("/api/create_item", web::post().to(character::add_item))
            .route("/api/list_items", web::post().to(character::list_items))
            .route("/api/edit_item", web::post().to(character::edit_item))
            .route("/", web::get().to(get_root))
            .route("/{filename:.*}", web::get().to(get_file))
    })
    .bind(&bind)?
    .run()
    .await
}
