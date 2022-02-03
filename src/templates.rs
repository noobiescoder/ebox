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

use crate::errors::{self, ResultExt};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

// Config structure for ebox.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub compiler: Compiler,
    pub library: Vec<String>,
}

// config.project structure.
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub license: String,
}

// config.compiler structure.
#[derive(Serialize, Deserialize)]
pub struct Compiler {
    pub optimize: bool,
    pub runs: i64,
}

// create config json.
pub fn config_new(conf: &Config) -> errors::Result<String> {
    serde_json::to_string_pretty(conf).chain_err(|| "error parsing config into jso")
}

// decode config json.
pub fn config_decode(conf: &str) -> errors::Result<Config> {
    serde_json::from_str(conf).chain_err(|| "error decoding config")
}

// solidity templates.
pub fn solidity_new(name: &str, license: &str) -> String {
    format!(
        "// SPDX-License-Identifier: {}

pragma solidity ^0.8.7;

contract {} {{
    constructor() {{}}
}}",
        license, name
    )
    .to_string()
}

// deployment structure.
#[derive(Serialize, Deserialize)]
pub struct Deployment {
    pub contract: String,
    pub constructor: HashMap<String, Input>,
}

// deployment.inputs struct.
#[derive(Serialize, Deserialize)]
pub struct Input {
    pub r#type: String,
    pub value: String,
}

// ABI structure.
#[derive(Deserialize)]
pub struct Abis {
    pub inputs: Vec<AbisInput>,
}

// ABI input structure.
#[derive(Deserialize)]
pub struct AbisInput {
    pub name: String,
    pub r#type: String,
}

// generate template from ABI.
pub fn d_generate(contract: &str, abi: &str) -> errors::Result<String> {
    let decode: Vec<Abis> = serde_json::from_str(abi).chain_err(|| "failed decoding ABI")?;

    let mut constructor: HashMap<String, Input> = HashMap::new();

    for input in &decode[0].inputs {
        let inp = Input {
            r#type: input.r#type.to_string(),
            value: String::new(),
        };
        constructor.insert(input.name.to_string(), inp);
    }

    let dep = Deployment {
        contract: contract.into(),
        constructor,
    };

    serde_json::to_string_pretty(&dep).chain_err(|| "failed parsing deployment details.")
}

// decode deployment config.
pub fn d_decode(contents: &str) -> errors::Result<Deployment> {
    serde_json::from_str(contents).chain_err(|| "failed decode deployment config")
}
