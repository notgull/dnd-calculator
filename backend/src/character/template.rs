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
    models::{ChangedTemplate, NewTemplate, Template},
    DbPool,
};
use actix_web::{web, Error, HttpResponse, Result as ActixResult};
use diesel::prelude::*;

fn add_template_impl(
    conn: &PgConnection,
    template: &NewTemplate,
) -> Result<i32, diesel::result::Error> {
    use crate::schema::templates::dsl::*;

    println!("Template: {:?}", template);

    let tid = diesel::insert_into(templates)
        .values(template)
        .returning(id)
        .get_result(conn)?;

    Ok(tid)
}

/// Add a template to the setup.
pub async fn add_template(
    pool: web::Data<DbPool>,
    template: web::Json<NewTemplate>,
) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    log::warn!("We've made it here!");

    let returned_id = web::block(move || add_template_impl(&conn, &template))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().json(returned_id).await
}

fn list_templates_impl(
    conn: &PgConnection,
) -> Result<Vec<(i32, String, String)>, diesel::result::Error> {
    use crate::schema::templates::dsl::*;

    let res = templates
        .select((id, name, description))
        .load::<(i32, String, String)>(&*conn)?;
    Ok(res)
}

/// Get a list of all of the needed templates
pub async fn list_templates(pool: web::Data<DbPool>) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let result = web::block(move || list_templates_impl(&conn))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().json(result).await
}

fn get_template_impl(conn: &PgConnection, tid: i32) -> Result<Template, diesel::result::Error> {
    use crate::schema::templates::dsl::*;

    let res = templates.filter(id.eq(tid)).get_result(conn)?;
    Ok(res)
}

/// Get a template by its template ID
pub async fn get_template(
    pool: web::Data<DbPool>,
    tid: web::Json<i32>,
) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let result = web::block(move || get_template_impl(&conn, *tid))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().json(result).await
}

fn edit_template_impl(
    conn: &PgConnection,
    tid: i32,
    template: &ChangedTemplate,
) -> Result<(), diesel::result::Error> {
    use crate::schema::templates::dsl::*;

    diesel::update(templates)
        .filter(id.eq(tid))
        .set(template)
        .execute(conn)?;

    Ok(())
}

/// Edit a template.
pub async fn edit_template(
    pool: web::Data<DbPool>,
    tid: web::Json<i32>,
    template: web::Json<ChangedTemplate>,
) -> ActixResult<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    web::block(move || edit_template_impl(&conn, *tid, &template))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    HttpResponse::Ok().await
}
