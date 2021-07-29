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
use std::sync::mpsc;
use std::thread;
//use std::time::Duration;

use actix_web::dev::Server;
use libathena::{payload::*, AthenaClientBuilder, Client, ClientBuilder};

use crate::*;

async fn run_app(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let data = Data::new().await;
    let data = actix_web::web::Data::new(data);

    println!("Starting server on: http://{}", SETTINGS.server.get_ip());

    let srv = HttpServer::new(move || {
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
    .bind(SETTINGS.server.get_ip())?
    .run();
    // send server controller to main thread
    let _ = tx.send(srv.clone());

    // run future
    srv.await
}

#[actix_rt::test]
async fn everything_works() {
    const PAYLOAD_TYPE: &str = "SHELL";
    const PAYLOAD: &str = "echo f";

    {
        let data = crate::Data::new().await;
        let _ = sqlx::query!("DELETE FROM cic_victims")
            .execute(&data.db)
            .await;
    };

    let (tx, rx) = mpsc::channel();

    println!("START SERVER");
    thread::spawn(move || {
        actix_rt::System::new().block_on(run_app(tx)).unwrap();
    });

    let srv = rx.recv().unwrap();

    let client = Client::builder().cookie_store(true);
    let athena = AthenaClientBuilder::default()
        .client(client)
        .unwrap()
        .host(format!("http://localhost:{}", SETTINGS.server.port))
        .password(SETTINGS.password.clone())
        .build()
        .unwrap();

    // register victim
    athena.victim_register().await.unwrap();

    // list victim
    let victims = athena.attack_list_victims().await.unwrap();
    assert!(victims.iter().any(|v| crate::api::v1::ships::SHIPS
        .iter()
        .any(|ship| ship == &v.name)));

    //attacker sets payload
    let mut payload_ids = Vec::with_capacity(victims.len());
    for victim in victims.iter() {
        let payload = attack::PayloadBuilder::default()
            .victim(victim.name.clone())
            .payload_type(PAYLOAD_TYPE.into())
            .payload(PAYLOAD.into())
            .password(athena.get_password().to_owned())
            .build()
            .unwrap();

        payload_ids.push(athena.attack_set_payload(&payload).await.unwrap());
    }

    // victim gets payload
    let victim_payload = athena.victim_get_paylod().await.unwrap();
    println!("{:?}", victim_payload.payloads);
    assert!(victim_payload
        .payloads
        .iter()
        .any(|p| p.payload_type == PAYLOAD_TYPE && p.payload == PAYLOAD));

    // stop server
    srv.stop(true).await;
}
