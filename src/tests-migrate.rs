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

use lazy_static::lazy_static;

mod data;
mod settings;

pub use data::Data;
pub use settings::Settings;

#[cfg(not(tarpaulin_include))]
lazy_static! {
    #[cfg(not(tarpaulin_include))]
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}

#[cfg(not(tarpaulin_include))]
#[actix_rt::main]
async fn main() {
    let data = Data::new().await;

    for arg in env::args() {
        if arg == "--build" {
            println!("Building cache buster config");
            build();
        }
    }

    sqlx::migrate!("./migrations/").run(&data.db).await.unwrap();
}

fn build() {
    use std::process::Command;

    // note: add error checking yourself.
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
