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
use std::fmt::{self, Display, Formatter};

use derive_more::{Display, Error};

#[derive(Debug, Clone, PartialEq, Error, Display)]
pub enum CliErrors {
    #[display(fmt = "Command not found")]
    CommandNotFound,

    #[display(fmt = "Please set environment variable ${}. It's used to get {} preference" _0.var, _0.preference)]
    EnvVarError(#[error(not(source))] EnvVarError),

    #[display(fmt = "Something went wront, unable to perform IO")]
    IOError,

    #[display(fmt = "Something went wront {}" _0)]
    BoxedError(#[error(not(source))] String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvVarError {
    pub var: String,
    pub preference: String,
}

impl Display for EnvVarError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.var, self.preference)
    }
}

impl From<std::io::Error> for CliErrors {
    fn from(_: std::io::Error) -> Self {
        Self::IOError
    }
}

impl From<Box<dyn std::error::Error>> for CliErrors {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::BoxedError(e.to_string())
    }
}

pub type CliResult<T> = Result<T, CliErrors>;
