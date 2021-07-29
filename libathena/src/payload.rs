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
use serde::{Deserialize, Serialize};

pub mod victim {
    use super::*;
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Payload {
        pub id: i32,
        pub payload_type: String,
        pub payload: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PayloadCollection {
        pub payloads: Vec<Payload>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PayloadResult {
        pub id: i32,
        pub response: String,
    }
}

pub mod attack {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Victim {
        pub name: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Password {
        pub password: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Payload {
        pub victim: String,
        pub payload_type: String,
        pub payload: String,
        pub password: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PayloadID {
        pub id: i32,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ResponseReq {
        pub id: i32,
        pub password: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PayloadResponse {
        pub response: Option<String>,
    }
}
