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

#[macro_use]
extern crate serial_test;

#[cfg(test)]
mod test_actions {
    use ebox::*;

    #[test]
    #[serial]
    fn test_init() {
        // case 1
        let dir_1 = "test_init";
        let init_1 = actions::init(dir_1, "MIT");
        assert_eq!(init_1.is_ok(), true);
        assert_eq!(utils::dir_exist(dir_1), true);
        assert_eq!(utils::file_exist("test_init/ebox.json"), true);
        // case 2
        let init_2 = actions::init(dir_1, "MIT");
        assert_ne!(init_2.is_ok(), true);
        // clean
        let clean_1 = utils::dir_del(dir_1);
        assert_eq!(clean_1.is_ok(), true);
    }

    #[test]
    #[serial]
    fn test_new_contracts() {
        // case 1
        let new_1 = actions::new_contracts(vec!["Test".into(), "Debug".into()]);
        assert_ne!(new_1.is_ok(), true);
        // case 2
        let init_2 = actions::init(".", "MIT");
        assert_eq!(init_2.is_ok(), true);
        let new_2 = actions::new_contracts(vec!["Test".into(), "Debug".into()]);
        assert_eq!(new_2.is_ok(), true);
        let c_dir = utils::dir_read("contracts");
        assert_eq!(c_dir.is_ok(), true);
        assert_eq!(c_dir.ok().unwrap().len(), 2);
        // clean
        let clean_1 = utils::file_del("ebox.json");
        assert_eq!(clean_1.is_ok(), true);
        let clean_2 = utils::dir_del("contracts");
        assert_eq!(clean_2.is_ok(), true);
    }

    #[test]
    #[serial]
    fn test_sbox() {
        // case 1
        let sbox_1 = actions::sbox();
        assert_ne!(sbox_1.is_ok(), true);
        // case 2
        let init_2 = actions::init(".", "MIT");
        assert_eq!(init_2.is_ok(), true);
        let sbox_2 = actions::sbox();
        assert_ne!(sbox_2.is_ok(), true);
        // case 3
        let new_3 = actions::new_contracts(vec!["Test".into(), "Debug".into()]);
        println!("{:?}", new_3);
        assert_eq!(new_3.is_ok(), true);
        let sbox_3 = actions::sbox();
        println!("{:?}", sbox_3);
        assert_eq!(sbox_3.is_ok(), true);
        let out = utils::dir_read("artifacts").unwrap();
        assert_eq!(out.len(), 4);
        // clean
        for d in vec!["artifacts", "contracts"] {
            let clean_d = utils::dir_del(d);
            assert_eq!(clean_d.is_ok(), true);
        }
        let clean_2 = utils::file_del("ebox.json");
        assert_eq!(clean_2.is_ok(), true);
    }
}
