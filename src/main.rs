// Copyright 2022 The ebox Authors.
// This file is part of ebox.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
//bdistributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::process;

use clap::{AppSettings::ArgRequiredElseHelp, Parser, Subcommand};
use ebox::actions;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(setting(ArgRequiredElseHelp))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(setting(ArgRequiredElseHelp))]
    /// Init new ebox project.
    Init {
        /// path to project directory.
        path: String,

        #[clap(long, default_value = "MIT")]
        /// license of the project. e.g MIT
        license: String,
    },

    #[clap(setting(ArgRequiredElseHelp))]
    /// create new ethereum (solidity) contracts.
    New {
        /// name of the contracts.
        name: Vec<String>,
    },

    /// compile solidity contracts.
    r#Box {},
}

fn main() {
    let app = Cli::parse();

    match &app.command {
        Some(Commands::Init { path, license }) => {
            let init = actions::init(path, license);
            if init.is_err() {
                eprintln!("{:?}", init.err().unwrap());
                process::exit(1);
            }

            println!("Project {} was initiated.", path);
        }

        Some(Commands::New { name }) => {
            let new = actions::new_contracts(name);
            if new.is_err() {
                println!("{:?}", new.err().unwrap());
                process::exit(1);
            }
        }

        Some(Commands::r#Box {}) => {
            let sbox = actions::sbox();
            if sbox.is_err() {
                println!("{:?}", sbox.err().unwrap());
                process::exit(1);
            }
        }

        None => {}
    }
}
