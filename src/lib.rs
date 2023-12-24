#![deny(clippy::all)]
use napi::{bindgen_prelude::Array, CallContext, JsObject, JsString, Result, Env, JsNumber};
use napi_derive::{js_function, napi};
use std::collections::HashSet;


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

#[napi]
fn to_js_obj(env: Env) -> napi::Result<JsObject> {
  let mut arr = env.create_array(0)?;
  arr.insert("a string")?;
  arr.insert(42)?;
  arr.coerce_to_object()
}

// #[napi]
// pub fn loop_value(m: JsValue, keys: &HashSet<String>, result: &mut Option<HashSet<String>>) {
  // let js_value = ctx.get::<JsValue>(0)?;
  // let keys = ctx.get::<HashSet<String>>(1)?;
  // let mut result = ctx.get::<Option<HashSet<String>>>(2)?.unwrap_or_default();
  // let js_value = ctx.get(0)?;
  // 对于 keys，我们假设它是一个 JavaScript 数组，并且每个元素都是一个字符串
  // let js_array = ctx.get::<JsObject>(1)?;
  // let mut keys = HashSet::new();
  // let array_length = js_array.get_array_length()?;
  // for i in 0..array_length {
  //   let js_string = js_array.get_element::<JsString>(i)?;
  //   let key = js_string.into_utf8()?.into_owned()?;
  //   keys.insert(key);
  // }

  // // 对于 result，我们暂时忽略这个参数，因为它的处理方式取决于你的具体需求
  // let mut result = HashSet::new();

  // fn process_string(s: String, result: &mut HashSet<String>) -> String {
  //   result.insert(s.clone());
  //   s
  // }

  // match js_value {
  //   JsValue::Object(obj) => {
  //     let mut collected_strings = Vec::new();
  //     for key in keys.iter() {
  //       if obj.has_own_property(key)? {
  //         if let Some(value) = obj.get_named_property::<JsValue>(key)? {
  //           match value {
  //             JsValue::String(s) => {
  //               let s_str: String = s.into_utf8()?.into_owned()?;
  //               collected_strings.push(process_string(s_str, &mut result));
  //             }
  //             _ => {}
  //           }
  //         }
  //       }
  //     }
  //     Ok(Some(collected_strings))
  //   }
  //   JsValue::String(s) => {
  //     let s_str: String = s.into_utf8()?.into_owned()?;
  //     Ok(Some(vec![process_string(s_str, &mut result)]))
  //   }
  //   JsValue::Array(arr) => {
  //     let arr: Array = arr.try_into()?;
  //     let mut collected_strings = Vec::new();
  //     for i in 0..arr.get_array_length()? {
  //       let value = arr.get_element::<JsValue>(i)?;
  //       if let JsValue::String(s) = value {
  //         let s_str: String = s.into_utf8()?.into_owned()?;
  //         collected_strings.push(process_string(s_str, &mut result));
  //       }
  //     }
  //     Ok(Some(collected_strings))
  //   }
  //   JsValue::Null | JsValue::Undefined => Ok(None),
  //   _ => Ok(None), // Handle other types if needed
  // }
// }

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


#[napi]
// fn factorial(n: JsNumber) -> Result<JsNumber> {
//   // 使用 get_value 将 JsNumber 转换为 f64，然后转为 u64
//   let num: f64 = n.try_into()?;
//   let result = factorial_recursive(num as u64);
//   // 使用 create_uint64 将 u64 结果转换回 JsNumber
//   n.env().create_uint64(result)
// }

fn factorial(ctx: CallContext) -> Result<JsNumber> {
  let n: JsNumber = ctx.get::<JsNumber>(0)?;
  let num: f64 = n.try_into()?;
  let result = factorial_recursive(num as u64);
  // 使用 ctx.env.create_uint64 将 u64 结果转换回 JsNumber
  ctx.env.create_uint64(result)
}


fn factorial_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial_recursive(n - 1),
    }
}