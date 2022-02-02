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

use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::{
    compiler,
    templates::{config_decode, config_new, solidity_new, Compiler, Config, Project},
    utils,
};

// actions for initiating new ebox project.
pub fn init(path: &str, license: &str) -> Result<(), Error> {
    let config_path = format!("{}/{}", path, "ebox.json");
    let dir_exist = utils::dir_exist(path);
    let config_exist = utils::file_exist(&config_path);
    let err_1 = Error::new(ErrorKind::Interrupted, "Directory not empty");

    if !dir_exist {
        let _ = utils::dir_new(path)?;
    }

    if config_exist {
        return Err(err_1);
    }

    let name = fs::canonicalize(PathBuf::from(path))?
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let conf = Config {
        project: Project {
            name,
            license: String::from(license),
        },
        compiler: Compiler {
            optimize: false,
            runs: 0,
        },
        library: Vec::new(),
    };

    let _ = utils::file_new(&config_path, config_new(&conf)?.as_str());

    Ok(())
}

// actions for create new contracts.
pub fn new_contracts(contracts: Vec<String>) -> Result<(), Error> {
    let config_path = "ebox.json";
    let config_exist = utils::file_exist(config_path);
    let contract_dir = "contracts";
    let contract_dir_exist = utils::dir_exist(contract_dir);
    let err_1 = Error::new(
        ErrorKind::Interrupted,
        "`ebox.json` not found, consider init the project first.",
    );

    if !config_exist {
        return Err(err_1);
    }

    if !contract_dir_exist {
        let _ = utils::dir_new(contract_dir)?;
    }

    let conf_str = utils::file_read(config_path)?;
    let conf = config_decode(&conf_str)?;
    let license = conf.project.license.as_str();

    for contract in contracts {
        let contract_name = contract.as_str();
        let sol = format!("{}/{}.sol", contract_dir, contract_name);
        let _ = utils::file_new(&sol, &solidity_new(contract_name, license));
    }

    Ok(())
}

// actions for compile the project.
pub fn sbox() -> Result<(), Error> {
    let config_path = "ebox.json";
    let config_exist = utils::file_exist(config_path);
    let contract_dir = "contracts";
    let contract_dir_exist = utils::dir_exist(contract_dir);
    let err_1 = Error::new(
        ErrorKind::Interrupted,
        "`ebox.json` not found, consider init the project first.",
    );

    let err_2 = Error::new(
        ErrorKind::Interrupted,
        "`contracts` directory not found, consider create new contract inside `contracts` first.",
    );

    if !config_exist {
        return Err(err_1);
    }

    if !contract_dir_exist {
        return Err(err_2);
    }

    let conf_str = utils::file_read(config_path)?;
    let conf = config_decode(&conf_str)?;
    let c_optimize = conf.compiler.optimize;
    let c_runs = conf.compiler.runs.to_string();
    let mut opts: Vec<String> = Vec::new();

    if c_optimize {
        opts.push(String::from("--optimize"));
        opts.push(String::from("--optimize-runs"));
        opts.push(c_runs);
    }

    let contracts = utils::dir_read(contract_dir)?;
    let solc = compiler::new("solc");

    for ele in contracts {
        let path_str = format!("{}/{}", contract_dir, &ele);
        let path = PathBuf::from(&path_str);
        if path.is_file() {
            if path.extension().unwrap() == "sol" {
                let _ = solc.compile(&path_str, "artifacts", &opts)?;
            }
        }
    }

    Ok(())
}
