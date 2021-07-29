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
mod attack;
pub mod ships;
mod victim;

use crate::errors::*;

pub use ships::get_name;

pub async fn join_rnner(id: &actix_identity::Identity, data: &crate::AppData) -> ServiceResult<()> {
    if id.identity().is_none() {
        let name = get_name(data).await.to_string();
        sqlx::query!("INSERT INTO cic_victims (name) VALUES ($1);", &name)
            .execute(&data.db)
            .await?;
        id.remember(name);
    }
    Ok(())
}

#[allow(dead_code)]
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

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    victim::services(cfg);
    attack::services(cfg);
}
