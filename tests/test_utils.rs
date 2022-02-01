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
mod test_utils {
    use std::env::temp_dir;

    use ebox::utils;

    #[test]
    fn test_dir_exist() {
        // case 1
        assert_ne!(utils::dir_exist("lol"), true);
        // case 2
        assert_eq!(utils::dir_exist("."), true);
    }

    #[test]
    fn test_file_exist() {
        // case 1
        assert_ne!(utils::file_exist("lol.toml"), true);
        // case 2
        assert_eq!(utils::file_exist("Cargo.toml"), true);
    }

    #[test]
    fn test_dir_read() {
        // case 1
        let read_1 = utils::dir_read("lol");
        assert_ne!(read_1.is_ok(), true);
        // case 2
        let read_2 = utils::dir_read(".");
        assert_eq!(read_2.is_ok(), true);
        assert!(read_2.unwrap().len() > 2);
    }

    #[test]
    fn test_file_read() {
        // case 1
        let read_1 = utils::file_read("lol.toml");
        assert_ne!(read_1.is_ok(), true);
        // case 2
        let read_2 = utils::file_read("Cargo.toml");
        assert_eq!(read_2.is_ok(), true);
        assert_eq!(read_2.unwrap().contains("ebox"), true);
    }

    #[test]
    fn test_dir_new() {
        // case 1
        let new_2 = utils::dir_new("test_dir_new");
        assert_eq!(new_2.is_ok(), true);
        // clean
        assert_eq!(utils::dir_del("test_dir_new").is_ok(), true);
    }

    #[test]
    fn test_file_new() {
        // case 1
        let mut tmp = temp_dir();
        tmp.push("test_file_new.txt");
        let target = tmp.into_os_string().into_string().unwrap();
        let new_1 = utils::file_new(&target, "test");
        assert_eq!(new_1.is_ok(), true);
        let read_1 = utils::file_read(&target).unwrap();
        assert_eq!(read_1.contains("test"), true);
        // clean
        assert_eq!(utils::file_del(&target).is_ok(), true);
    }

    #[test]
    fn test_dir_del() {
        // case 1
        let mut tmp = temp_dir();
        tmp.push("test_dir_del");
        let target = tmp.into_os_string().into_string().unwrap();
        let del_1 = utils::dir_del(&target);
        assert_ne!(del_1.is_ok(), true);
        // case 2
        let new_2 = utils::dir_new(&target);
        assert_eq!(new_2.is_ok(), true);
        let del_2 = utils::dir_del(&target);
        assert_eq!(del_2.is_ok(), true);
    }

    #[test]
    fn test_file_del() {
        // case 1
        let mut tmp = temp_dir();
        tmp.push("test_file_del.txt");
        let target = tmp.into_os_string().into_string().unwrap();
        let del_1 = utils::file_del(&target);
        assert_ne!(del_1.is_ok(), true);
        // case 2
        let new_2 = utils::file_new(&target, "");
        assert_eq!(new_2.is_ok(), true);
        let del_2 = utils::file_del(&target);
        assert_eq!(del_2.is_ok(), true);
    }
}
