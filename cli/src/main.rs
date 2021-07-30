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
use std::io::{stdin, stdout, BufWriter, Write};

use clap::Clap;
use libathena::{AthenaClientBuilder, Client};

use cli::options::Options;
use cli::Commands;
use cli::Mode;

const DEFAULT_PROMPT: &str = "(athena)";
const SHELL_PROMPT: &str = "(shell)";
const TARGET_ALL: &str = "(all)";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

fn welcome<W: Write>(handle: &mut BufWriter<W>) -> Result<(), Box<dyn Error>> {
    write!(
        handle,
        r#"Athena {} - C2 for Rats
Aravinth Mavnivannan<realaravinth@batsense.net>
{}
Disclaimer: This software is not authorized for use in committing computer fraud.
The authors of this software CAN NOT be held responsible for the program's users' actions

"#,
        VERSION, PKG_HOMEPAGE,
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    AthenaClientBuilder::default()
        .client(Client::builder())?
        .password(options.password)
        .host(options.c2)
        .build()?;

    let mut input = String::new();
    let mut mode = Mode::Default;
    let stdout = stdout(); // get the global stdout entity
    let mut handle = BufWriter::new(stdout);
    welcome(&mut handle)?;

    loop {
        match mode {
            Mode::Default => {
                input.clear();
                write!(handle, "{} => ", DEFAULT_PROMPT)?;
                handle.flush()?;
                stdin().read_line(&mut input)?;
                let cmd = Commands::parse(&input)?;
                cmd.set_mode(&mut mode);
                if mode == Mode::Default {
                    break;
                }
            }
            Mode::Shell => print!("{}{}", DEFAULT_PROMPT, SHELL_PROMPT),
            Mode::TargetAll => print!("{}{}", DEFAULT_PROMPT, TARGET_ALL),
        };
    }

    Ok(())
}
