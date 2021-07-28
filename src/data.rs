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
//! App data: database connections, etc.
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::SETTINGS;

/// App data
pub struct Data {
    /// databse pool
    pub db: PgPool,
}

impl Data {
    #[cfg(not(tarpaulin_include))]
    /// create new instance of app data
    pub async fn new() -> Arc<Self> {
        let db = PgPoolOptions::new()
            .max_connections(SETTINGS.database.pool)
            .connect(&SETTINGS.database.url)
            .await
            .expect("Unable to form database pool");

        let data = Data { db };

        Arc::new(data)
    }
}
