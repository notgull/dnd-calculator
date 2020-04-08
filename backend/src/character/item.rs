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

use crate::{
    models::{ChangedItem, Item, NewItem},
    DbPool,
};
use actix_web::{web, Error, HttpResponse, Result as ActixResult};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

fn add_item_impl(conn: &PgConnection, item: &NewItem) -> Result<i32, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    diesel::insert_into(items)
        .values(item)
        .returning(id)
        .get_result(conn)
}

/// Add item to database.
pub async fn add_item(
    pool: web::Data<DbPool>,
    template: web::Json<NewItem>,
) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let item_id = web::block(move || add_item_impl(&conn, &template))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().json(item_id).await
}

fn list_items_impl(conn: &PgConnection) -> Result<Vec<Item>, diesel::result::Error> {
    use crate::schema::items::dsl::*;

    items.load::<Item>(conn)
}

/// Get a list of all items
pub async fn list_items(pool: web::Data<DbPool>) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let items = web::block(move || list_items_impl(&conn))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().json(items).await
}

fn edit_item_impl(
    conn: &PgConnection,
    item_id: i32,
    item: &ChangedItem,
) -> Result<(), diesel::result::Error> {
    use crate::schema::items::dsl::*;

    diesel::update(items)
        .filter(id.eq(item_id))
        .set(item)
        .execute(conn)?;

    Ok(())
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EditItemReq {
    item_id: i32,
    item: ChangedItem,
}

/// Edit a particular item
pub async fn edit_item(
    pool: web::Data<DbPool>,
    data: web::Json<EditItemReq>,
) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let data = data.into_inner();

    web::block(move || edit_item_impl(&conn, data.item_id, &data.item))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().await
}
