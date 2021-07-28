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
use std::env;
use std::sync::Arc;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    error::InternalError, http::StatusCode, middleware as actix_middleware, web::JsonConfig, App,
    HttpServer,
};
use lazy_static::lazy_static;
use log::info;

mod api;
mod data;
mod settings;

pub use crate::data::Data;
pub use api::v1::ROUTES as V1_API_ROUTES;
pub use settings::Settings;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}

pub const COMPILED_DATE: &str = env!("COMPILED_DATE");
pub const GIT_COMMIT_HASH: &str = env!("GIT_HASH");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

pub type AppData = actix_web::web::Data<Arc<crate::data::Data>>;

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");

    pretty_env_logger::init();
    info!(
        "{}: {}.\nFor more information, see: {}\nBuild info:\nVersion: {} commit: {}",
        PKG_NAME, PKG_DESCRIPTION, PKG_HOMEPAGE, VERSION, GIT_COMMIT_HASH
    );

    let data = Data::new().await;
    sqlx::migrate!("./migrations/").run(&data.db).await.unwrap();
    let data = actix_web::web::Data::new(data);

    println!("Starting server on: http://{}", SETTINGS.server.get_ip());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .header("Permissions-Policy", "interest-cohort=()"),
            )
            .wrap(get_identity_service())
            .wrap(actix_middleware::Compress::default())
            .app_data(data.clone())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::TrailingSlash::Trim,
            ))
            .configure(api::v1::services)
            .app_data(get_json_err())
    })
    .bind(SETTINGS.server.get_ip())
    .unwrap()
    .run()
    .await
}

#[cfg(not(tarpaulin_include))]
pub fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

#[cfg(not(tarpaulin_include))]
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    let cookie_secret = &SETTINGS.server.cookie_secret;
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name("Authorization")
            //TODO change cookie age
            .max_age_secs(216000)
            .domain(&SETTINGS.server.domain)
            .secure(false),
    )
}
