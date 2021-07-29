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
use url::Url;

use reqwest::Client;

pub mod payload;
pub mod routes;
pub use routes::V1_ROUTES;

#[derive(Clone, Default)]
pub struct AthenaClientBuilder {
    client: Option<Client>,
    host: Option<String>,
}

impl AthenaClientBuilder {
    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }
    pub fn host(&mut self, host: &str) -> &mut Self {
        if host.ends_with('/') {
            self.host = Some(host[0..host.len() - 1].to_owned())
        } else {
            self.host = Some(host.to_owned());
        }
        self
    }

    pub fn build(&mut self) -> AthenaClient {
        AthenaClient {
            client: self.client.clone().unwrap(),
            host: Url::parse(&self.host.clone().unwrap()).unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct AthenaClient {
    client: Client,
    host: Url,
}

impl AthenaClient {
    pub async fn register(&self) {
        let url = self.host.clone().join(V1_ROUTES.victim.join);
        //self.client.post(url).json();
    }
}
