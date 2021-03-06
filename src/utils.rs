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
use std::{fs, path::PathBuf};

// check if a dir exist.
pub fn dir_exist(target: &str) -> bool {
    let path = PathBuf::from(target);
    path.is_dir()
}

// check if a file exist.
pub fn file_exist(target: &str) -> bool {
    let path = PathBuf::from(target);
    path.exists() && path.is_file()
}

// read all entry inside a dir.
pub fn dir_read(target: &str) -> errors::Result<Vec<String>> {
    let path = PathBuf::from(target);
    let contents = fs::read_dir(path).chain_err(|| "failed reading directory contents")?;

    let mut res: Vec<String> = Vec::new();

    for ele in contents {
        let el = ele.unwrap().file_name().into_string().unwrap();
        res.push(el);
    }

    Ok(res)
}

// read a file.
pub fn file_read(target: &str) -> errors::Result<String> {
    let path = PathBuf::from(target);
    fs::read_to_string(path).chain_err(|| "failed reading file")
}

// create a dir.
pub fn dir_new(target: &str) -> errors::Result<()> {
    let path = PathBuf::from(target);
    fs::create_dir_all(path).chain_err(|| "failed creating directory")
}

// write a file.
pub fn file_new(target: &str, contents: &str) -> errors::Result<()> {
    let path = PathBuf::from(target);
    fs::write(path, contents).chain_err(|| "failed writing new file")
}

// delete directory.
pub fn dir_del(target: &str) -> errors::Result<()> {
    let path = PathBuf::from(target);
    fs::remove_dir_all(path).chain_err(|| "failed deleting directory")
}

// delete files,
pub fn file_del(target: &str) -> errors::Result<()> {
    let path = PathBuf::from(target);
    fs::remove_file(path).chain_err(|| "failed deleting file")
}
