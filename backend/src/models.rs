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

use serde::{Deserialize, Serialize};

use crate::schema::{actiontakers as action_takers, items, moves, room as rooms, templates};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct ActionTaker {
    pub room_id: i32,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub armor_class: i32,
    pub health: i32,
    pub dead: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Room {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Move {
    pub id: i32,
    pub name: String,
    hit_type: i32,
    pub hit_radius: Option<i32>,
    pub dice_count: i32,
    pub dice_type: i32,
    pub dice_modifier: i32,
    stat_boost: i32,
    saving_throw: i32,
    pub effect: Option<i32>,
    pub effect_severity: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub health: i32,
    pub armor_class: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "templates"]
pub struct NewTemplate {
    pub name: String,
    pub health: i32,
    pub armor_class: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[table_name = "templates"]
pub struct ChangedTemplate {
    pub name: Option<String>,
    pub health: Option<i32>,
    pub armor_class: Option<i32>,
    pub description: Option<String>,
}
