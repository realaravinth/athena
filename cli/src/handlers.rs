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

use libathena::payload::attack::PayloadBuilder;
use tempfile::tempdir;
use tokio::fs;

use crate::{errors::*, State};

pub async fn javascript<W: Write>(s: &mut State<W>) -> CliResult<()> {
    const FILE_NAME: &str = "payload.js";

    let editor = if s.editor.is_some() {
        s.editor.clone().unwrap()
    } else {
        var("EDITOR").map_err(|_| {
            CliErrors::EnvVarError(EnvVarError {
                var: "EDITOR".into(),
                preference: "editor".into(),
            })
        })?
    };

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

    s.upload_payload(&mut payload).await?;
    Ok(())
}

pub async fn shell<W: Write>(s: &mut State<W>, input: &mut String) -> CliResult<()> {
    let mut payload = PayloadBuilder::default()
        .victim("".into())
        .payload_type("SHELL".into())
        .payload("".into())
        .password(s.client.get_password().to_owned())
        .build()
        .unwrap();

    loop {
        s.mode.clone().print_and_read(s, input)?;

        payload.payload = input.trim().to_string();
        s.upload_payload(&mut payload).await?;
        s.read_responses().await?;
    }

    Ok(())
}
