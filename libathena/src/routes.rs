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
//! Routes available on the C2 server

/// Routes constant
pub const V1_ROUTES: v1::V1 = v1::V1::new();

pub mod v1 {
    //! Routes used in version 1
    pub struct V1 {
        pub victim: Victim,
        pub attack: Attack,
    }

    impl V1 {
        pub const fn new() -> V1 {
            V1 {
                victim: Victim::new(),
                attack: Attack::new(),
            }
        }
    }

    pub struct Victim {
        pub join: &'static str,
        pub get_payload: &'static str,
        pub payload_response: &'static str,
        pub scope: &'static str,
    }

    impl Victim {
        pub const fn new() -> Victim {
            Victim {
                join: "/api/v1/victim/join",
                get_payload: "/api/v1/victim/payload/get",
                payload_response: "/api/v1/victim/payload/response",
                scope: "/api/v1/victim/",
            }
        }
    }

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
