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

use crate::{models::Template, DbPool};
use actix_web::{web, Error, HttpResponse, Result as ActixResult};
use diesel::prelude::*;

fn add_template_impl(
    conn: &PgConnection,
    template: &Template,
) -> Result<(), diesel::result::Error> {
    use crate::schema::templates::dsl::*;

    diesel::insert_into(templates)
        .values(template)
        .execute(conn)?;

    Ok(())
}

pub async fn add_template(
    pool: web::Data<DbPool>,
    template: web::Json<Template>,
) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    web::block(move || add_template_impl(&conn, &template)).await.map_err(|e| {
        log::error!("{:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    HttpResponse::Ok().await
}
