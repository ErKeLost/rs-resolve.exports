#![deny(clippy::all)]

use std::collections::HashSet;

use napi::Result;
use napi_derive::napi;
#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn plus_1000(input: u32) -> u32 {
  input + 1000
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Condition {
  Default,
  Require,
  Import,
  Browser,
  Node,
  Development,
  Module,
  Production,
  Custom(String),
}

// impl FromStr for Condition {
//   type Err = String;

//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     match s {
//       "default" => Ok(Condition::Default),
//       "require" => Ok(Condition::Require),
//       "import" => Ok(Condition::Import),
//       "browser" => Ok(Condition::Browser),
//       "node" => Ok(Condition::Node),
//       "development" => Ok(Condition::Development),
//       "production" => Ok(Condition::Production),
//       "module" => Ok(Condition::Module),
//       c => Ok(Condition::Custom(c.to_string())),
//       // _ => {}
//     }
//   }
// }

use serde_json::Value;

#[napi]
pub fn loop_value(
  m: Value,
  keys: &HashSet<String>,
  result: &mut Option<HashSet<String>>,
) -> Option<Vec<String>> {
  match m {
    Value::String(s) => {
      if let Some(result_set) = result {
        result_set.insert(s.clone());
      }
      Some(vec![s])
    }
    Value::Array(values) => {
      let arr_result = result.clone().unwrap_or_else(|| HashSet::new());
      for item in values {
        if let Some(item_result) = loop_value(item, keys, &mut Some(arr_result.clone())) {
          return Some(item_result);
        }
      }

      if result.is_none() && !arr_result.is_empty() {
        return Some(arr_result.into_iter().collect());
      } else {
        None
      }
    }
    Value::Object(map) => {
      // TODO Temporarily define the order problem
      let property_order: Vec<String> = vec![
        String::from("browser"),
        String::from("development"),
        String::from("module"),
        String::from("import"),
        String::from("require"),
        String::from("default"),
      ];

      for key in &property_order {
        if let Some(value) = map.get(key) {
          // if let Ok(condition) = Condition::from_str(&key) {
          if keys.contains(key.as_str()) {
            return loop_value(value.clone(), keys, result);
          }
          // }
        }
      }
      None
    }
    Value::Null => None,
    _ => None,
  }
}
