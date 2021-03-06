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
use std::io::Write;

use crate::errors::*;
use crate::Mode;
use crate::State;

#[derive(Clone, Debug, PartialEq)]
pub enum Commands {
    ListVictims,
    SelectVictim,
    MultipleVictims,
    JavaScript,
    Shell,
    Help,
    Exit,
}

impl Commands {
    /// get matching user input for Self
    pub fn get_val(&self) -> &'static str {
        match self {
            Self::ListVictims => "lsv",
            Self::SelectVictim => "select",
            Self::MultipleVictims => "all",
            Self::JavaScript => "js",
            Self::Shell => "sh",
            Self::Help => "help",
            Self::Exit => "exit",
        }
    }

    /// set mode when a command is received
    pub fn set_mode(&self, mode: &mut Mode) {
        macro_rules! set_mode {
            ($self:expr, $option:expr, $mode:expr, $val:expr) => {
                if *$self == $option {
                    return *$val = $mode;
                };
            };
        }

        set_mode!(self, Self::Shell, Mode::Shell, mode);
        set_mode!(self, Self::MultipleVictims, Mode::TargetAll, mode);
        if *mode != Mode::Default {
            set_mode!(self, Self::Exit, Mode::Default, mode);
        } else {
            set_mode!(self, Self::Exit, Mode::Exit, mode);
        }
    }

    /// Parse user input and to get Self
    pub fn parse(cmd: &str) -> CliResult<Self> {
        macro_rules! derive_parse {
            ($item:expr, $cmd:expr) => {
                if $cmd == $item.get_val() {
                    return Ok($item);
                };
            };
        }

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

    /// runner command that parses input and sets mode
    pub fn parse_and_set_mode<W: Write>(s: &mut State<W>, input: &mut String) -> CliResult<Self> {
        let cmd = Commands::parse(&input)?;
        cmd.set_mode(&mut s.mode);
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_work() {
        macro_rules! get_val_tests {
            ($enum:expr, $val:expr) => {
                assert_eq!($enum.get_val(), $val);
                assert_eq!(Commands::parse($val).unwrap(), $enum);
            };
        }

        get_val_tests!(Commands::ListVictims, "lsv");
        get_val_tests!(Commands::SelectVictim, "select");
        get_val_tests!(Commands::MultipleVictims, "all");
        get_val_tests!(Commands::JavaScript, "js");
        get_val_tests!(Commands::Shell, "sh");
        get_val_tests!(Commands::Help, "help");
        get_val_tests!(Commands::Exit, "exit");

        assert_eq!(
            Commands::parse("commanddoesntexist").err().unwrap(),
            CliErrors::CommandNotFound
        );
    }

    #[test]
    fn set_mode_works() {
        macro_rules! mode_command_conv {
            ($cmd:expr, $mode:expr, $expected_mode:expr) => {
                $cmd.set_mode(&mut $mode);
                assert_eq!($mode, $expected_mode);
            };
        }

        let mut mode = Mode::Default;
        mode_command_conv!(Commands::MultipleVictims, mode, Mode::TargetAll);
        mode_command_conv!(Commands::Shell, mode, Mode::Shell);
        mode_command_conv!(Commands::Exit, mode, Mode::Default);
        mode_command_conv!(Commands::Exit, mode, Mode::Exit);
    }
}
