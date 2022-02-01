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

#[cfg(test)]
mod test_compiler {
    use ebox::{compiler::new, utils};

    #[test]
    fn test_version() {
        // case 1
        let solc_1 = new("sol");
        let ver_1 = solc_1.version();
        assert_ne!(ver_1.is_ok(), true);
        // case 2
        let solc_2 = new("solc");
        let ver_2 = solc_2.version().unwrap();
        assert_eq!(ver_2.contains("commit"), true);
    }

    #[test]
    fn test_compile() {
        // case 1
        let contract_dir = utils::dir_new("contracts");
        assert_eq!(contract_dir.is_ok(), true);
        let solc_1 = new("sol");
        let opt_1: Vec<String> = Vec::new();
        let compile_1 = solc_1.compile("Test.sol", "out1", &opt_1);
        assert_ne!(compile_1.is_ok(), true);
        // case 2
        let c_dir_2 = utils::dir_new("test_compile");
        assert_eq!(c_dir_2.is_ok(), true);
        let f_2 = utils::file_new("test_compile/Test.sol", "test");
        assert_eq!(f_2.is_ok(), true);
        let solc_2 = new("solc");
        let compile_2 = solc_2.compile("test_compile/Test.sol", "out2", &opt_1);
        assert_ne!(compile_2.is_ok(), true);
        // case 3
        let f_3 = utils::file_new(
            "test_compile/Test.sol",
            "// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract Test {
    constructor() {}
}",
        );
        assert_eq!(f_3.is_ok(), true);
        let compile_3 = solc_2.compile("test_compile/Test.sol", "out3", &opt_1);
        assert_eq!(compile_3.is_ok(), true);
        let out_3 = utils::dir_read("out3").unwrap();
        assert_eq!(out_3.len(), 2);
        // clean
        let clean_1 = utils::dir_del("test_compile");
        assert_eq!(clean_1.is_ok(), true);
        let clean_2 = utils::dir_del("out3");
        assert_eq!(clean_2.is_ok(), true);
        let clean_3 = utils::dir_del("contracts");
        assert_eq!(clean_3.is_ok(), true);
    }
}
