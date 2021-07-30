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
use std::env::var;
use std::io::Write;
use std::process::Command;

use libathena::payload::attack::{PayloadBuilder, PayloadID};
use tempfile::tempdir;
use tokio::fs;

use crate::{errors::*, State};

pub async fn javascript<W: Write>(s: &mut State<W>) -> CliResult<Vec<PayloadID>> {
    const FILE_NAME: &str = "payload.js";

    let editor = var("EDITOR").map_err(|_| {
        CliErrors::EnvVarError(EnvVarError {
            var: "EDITOR".into(),
            preference: "editor".into(),
        })
    })?;

    let dir = tempdir()?;

    let file_path = dir.path().join(FILE_NAME);

    let mut editor_program = Command::new(&editor)
        .arg(&file_path)
        .spawn()
        .unwrap_or_else(|_| panic!("Unable to launch editor {}", &editor));

    editor_program.wait()?;

    let payload_content = fs::read_to_string(&file_path).await?;

    let mut payload = PayloadBuilder::default()
        .victim("".into())
        .payload_type("JAVASCRIPT".into())
        .payload(payload_content.trim().to_owned())
        .password(s.client.get_password().to_owned())
        .build()
        .unwrap();

    let mut payload_ids = Vec::with_capacity(s.victims.len());
    for victim in s.victims.iter() {
        payload.victim = victim.name.to_owned();
        let id = s.client.attack_set_payload(&payload).await?;
        payload_ids.push(id);
    }

    Ok(payload_ids)
}
