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
mod test_templates {
    use ebox::templates::{config_decode, config_new, solidity_new, Compiler, Config, Project};

    #[test]
    fn test_config_new() {
        // case 1
        let name = String::from("test_project");
        let license = String::from("MIT");
        let library: Vec<String> = Vec::new();
        let conf = Config {
            project: Project { name, license },
            compiler: Compiler {
                optimize: true,
                runs: 200,
            },
            library,
        };

        let json = config_new(&conf);
        assert_eq!(json.is_ok(), true);
        assert_eq!(json.unwrap().contains("\"library\": []"), true);
    }

    #[test]
    fn test_config_decode() {
        // case 1
        let conf = "{
  \"project\": {
    \"name\": \"ebox\",
    \"license\": \"MIT\"
  },
  \"compiler\": {
    \"optimize\": false,
    \"runs\": 0
  },
  \"library\": [
    \"github.com/pentalabs\"
  ]
}";
        let decode = config_decode(conf);
        assert_eq!(decode.is_ok(), true);
        assert_eq!(decode.unwrap().library.len(), 1);
    }

    #[test]
    fn test_solidity_new() {
        // case 1
        let sol = solidity_new("Test", "MIT");
        assert_eq!(sol.contains("contract Test"), true);
    }
}
