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

use std::process::Command;

use crate::errors::{self, ResultExt};

// compiler objects.
pub struct Compiler {
    bin: String,
}

// Compiler methods.
impl Compiler {
    // get compiler version.
    pub fn version(&self) -> errors::Result<String> {
        let cmd = Command::new(self.bin.as_str())
            .arg("--version")
            .output()
            .chain_err(|| "failed getting compiler version")?;

        unsafe {
            let res = std::str::from_utf8_unchecked(&cmd.stdout);
            Ok(res.to_string())
        }
    }

    // compile solidity files.
    pub fn compile(&self, input: &str, output_dir: &str, opts: &[String]) -> errors::Result<()> {
        let cmd = Command::new(self.bin.as_str())
            .args(vec![
                "--bin",
                "--abi",
                "--overwrite",
                "--base-path",
                "contracts",
                "--output-dir",
                output_dir,
            ])
            .args(opts)
            .arg(input)
            .output();

        match cmd {
            Err(err) => Err("failed running command to compile the contracts".into()),
            Ok(res) => {
                if res.stderr.len() > 1 {
                    println!("{}", std::str::from_utf8(&res.stderr).unwrap());
                    return Err("error when compiling contracts".into());
                }

                Ok(())
            }
        }
    }
}

// create new compiler object.
pub fn new(bin: &str) -> Compiler {
    Compiler {
        bin: bin.to_string(),
    }
}
