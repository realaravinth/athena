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

use clap::Clap;
use derive_more::{Display, Error};
use libathena::{AthenaClientBuilder, Client};
use serde::{Deserialize, Serialize};

mod options {
    use clap::*;

    /// Athena command and control CLI tool
    #[derive(Clap, Clone, Debug)]
    #[clap(
        name = "anthena-cli",
        author = "Aravinth Manivannan <realaravinth@batsense.net>",
        version = "0.1.0"
    )]
    pub struct Options {
        /// Password to login to C2 server
        #[clap(short, long)]
        pub password: String,

        /// C2 server URL
        #[clap(short, long)]
        pub c2: String,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum Commands {
    ListVictims,
    SelectVictim,
    MultipleVictims,
    JavaScript,
    Shell,
    Help,
    Exit,
}

macro_rules! derive_parse {
    ($item:expr, $cmd:expr) => {
        if $cmd == $item.get_val() {
            return Ok($item);
        };
    };
}

impl Commands {
    fn get_val(&self) -> &'static str {
        match self {
            Self::ListVictims => "lsv",
            Self::SelectVictim => "select",
            Self::MultipleVictims => "multi",
            Self::JavaScript => "js",
            Self::Shell => "sh",
            Self::Help => "help",
            Self::Exit => "exit",
        }
    }

    fn parse(cmd: &str) -> CliResult<Self> {
        let cmd = cmd.trim();

        derive_parse!(Self::ListVictims, cmd);
        derive_parse!(Self::SelectVictim, cmd);
        derive_parse!(Self::MultipleVictims, cmd);
        derive_parse!(Self::JavaScript, cmd);
        derive_parse!(Self::Shell, cmd);
        derive_parse!(Self::Help, cmd);
        derive_parse!(Self::Exit, cmd);

        Err(CliErrors::CommandNotFound)
    }
}

#[derive(Debug, Clone, PartialEq, Error, Display)]
enum CliErrors {
    #[display(fmt = "Command not found")]
    CommandNotFound,
}

type CliResult<T> = Result<T, CliErrors>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = options::Options::parse();

    AthenaClientBuilder::default()
        .client(Client::builder())?
        .password(options.password)
        .host(options.c2)
        .build()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_val_tests {
        ($enum:expr, $val:expr) => {
            assert_eq!($enum.get_val(), $val);
            assert_eq!(Commands::parse($val).unwrap(), $enum);
        };
    }

    #[test]
    fn commands_work() {
        get_val_tests!(Commands::ListVictims, "lsv");
        get_val_tests!(Commands::SelectVictim, "select");
        get_val_tests!(Commands::MultipleVictims, "multi");
        get_val_tests!(Commands::JavaScript, "js");
        get_val_tests!(Commands::Shell, "sh");
        get_val_tests!(Commands::Help, "help");
        get_val_tests!(Commands::Exit, "exit");

        assert_eq!(
            Commands::parse("commanddoesntexist").err().unwrap(),
            CliErrors::CommandNotFound
        );
    }
}
