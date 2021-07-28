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
mod join;
mod ships;

pub use ships::get_name;

pub const ROUTES: routes::V1 = routes::V1::new();

pub mod routes {
    pub struct V1 {
        pub victim_join: &'static str,
    }

    impl V1 {
        pub const fn new() -> V1 {
            V1 {
                victim_join: "/api/v1/victim/join",
            }
        }
    }
}

pub async fn join_rnner(id: &actix_identity::Identity, data: &crate::AppData) {
    if let Some(_) = id.identity() {
        ()
    } else {
        id.remember(get_name(data).await.to_string());
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(join::join);
}

pub fn get_random(len: usize) -> String {
    use std::iter;

    use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

    let mut rng: ThreadRng = thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect::<String>()
}
