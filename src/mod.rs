fn resolve_exports_or_imports(
  &self,
  package_json_info: &PackageJsonInfo,
  source: &str,
  field_type: &str,
  kind: &ResolveKind,
  context: &Arc<CompilationContext>,
  is_matched: bool,
) -> Option<Vec<String>> {
  farm_profile_function!("resolve_exports_or_imports".to_string());
  let mut additional_conditions: HashSet<String> = vec![
    String::from("development"),
    String::from("production"),
    String::from("module"),
  ]
  .into_iter()
  .collect();

  let resolve_conditions: HashSet<String> = context
    .config
    .resolve
    .conditions
    .clone()
    .into_iter()
    .collect();
  additional_conditions.extend(resolve_conditions);

  let filtered_conditions: Vec<String> = additional_conditions
    .clone()
    .into_iter()
    .filter(|condition| match condition.as_str() {
      "production" => {
        let mode = &context.config.mode;
        matches!(mode, Mode::Production)
      }
      "development" => {
        let mode = &context.config.mode;
        matches!(mode, Mode::Development)
      }
      _ => true,
    })
    .collect();

  // resolve exports field
  let is_browser = TargetEnv::Browser == context.config.output.target_env;
  let is_require = match kind {
    ResolveKind::Require => true,
    _ => false,
  };
  let condition_config = ConditionOptions {
    browser: is_browser && !additional_conditions.contains("node"),
    require: is_require && !additional_conditions.contains("import"),
    conditions: filtered_conditions,
    // set default unsafe_flag to insert require & import field
    unsafe_flag: false,
  };
  let id: &str = if is_matched { source } else { "." };
  let result: Option<Vec<String>> = if field_type == "imports" {
    self.imports(package_json_info, source, &condition_config)
  } else {
    self.exports(package_json_info, id, &condition_config)
  };
  return result;
}

fn conditions(self: &Self, options: &ConditionOptions) -> HashSet<Condition> {
  let mut out: HashSet<Condition> = HashSet::new();
  out.insert(Condition::Default);

  for condition_str in &options.conditions {
    match Condition::from_str(condition_str) {
      Ok(condition_enum) => {
        out.insert(condition_enum);
      }
      Err(error) => {
        // TODO resolve error
        eprintln!("Error: {}", error);
      }
    }
  }
  if !options.unsafe_flag {
    if options.require {
      out.insert(Condition::Require);
    } else {
      out.insert(Condition::Import);
    }

    if options.browser {
      out.insert(Condition::Browser);
    } else {
      out.insert(Condition::Node);
    }
  }
  out
}

fn injects(self: &Self, items: &mut Vec<String>, value: &str) -> Option<Vec<String>> {
  let rgx1: regex::Regex = regex::Regex::new(r#"\*"#).unwrap();
  let rgx2: regex::Regex = regex::Regex::new(r#"/$"#).unwrap();

  for item in items.iter_mut() {
    let tmp = item.clone();
    if rgx1.is_match(&tmp) {
      *item = rgx1.replace(&tmp, value).to_string();
    } else if rgx2.is_match(&tmp) {
      *item += value;
    }
  }

  return items.clone().into_iter().map(|s| Some(s)).collect();
}

fn loop_value(
  self: &Self,
  m: Value,
  keys: &HashSet<Condition>,
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
        if let Some(item_result) = self.loop_value(item, keys, &mut Some(arr_result.clone())) {
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
          if let Ok(condition) = Condition::from_str(&key) {
            if keys.contains(&condition) {
              return self.loop_value(value.clone(), keys, result);
            }
          }
        }
      }
      None
    }
    Value::Null => None,
    _ => None,
  }
}

fn to_entry(
  self: &Self,
  name: &str,
  ident: &str,
  externals: Option<bool>,
) -> Result<String, String> {
  if name == ident || ident == "." {
    return Ok(".".to_string());
  }

  let root = format!("{}/", name);
  let len = root.len();
  let bool = ident.starts_with(&root);
  let output = if bool {
    ident[len..].to_string()
  } else {
    ident.to_string()
  };

  if output.starts_with('#') {
    return Ok(output);
  }

  if bool || externals.unwrap_or(false) {
    if output.starts_with("./") {
      Ok(output)
    } else {
      Ok(format!("./{}", output))
    }
  } else {
    Err(output)
  }
}

fn throws(self: &Self, name: &str, entry: &str, condition: Option<i32>) {
  let message = if let Some(cond) = condition {
    if cond != 0 {
      format!(
        "No known conditions for \"{}\" specifier in \"{}\" package",
        entry, name
      )
    } else {
      format!("Missing \"{}\" specifier in \"{}\" package", entry, name)
    }
  } else {
    format!("Missing \"{}\" specifier in \"{}\" package", entry, name)
  };
  eprintln!("{}", message);
}

fn walk(
  self: &Self,
  name: &str,
  mapping: &HashMap<String, Value>,
  input: &str,
  options: &ConditionOptions,
) -> Vec<String> {
  let entry_result: Result<String, String> = self.to_entry(name, input, Some(true));
  let entry: String = match entry_result {
    Ok(entry) => entry.to_string(),
    Err(error) => {
      eprintln!("Error resolve {} package error: {}", name, error);
      String::from(name)
    }
  };
  let c: HashSet<Condition> = self.conditions(options);
  let mut m: Option<&Value> = mapping.get(&entry);
  let mut replace: Option<String> = None;
  if m.is_none() {
    let mut longest: Option<&str> = None;

    for (key, _value) in mapping.iter() {
      if let Some(cur_longest) = &longest {
        if key.len() < cur_longest.len() {
          // do not allow "./" to match if already matched "./foo*" key
          continue;
        }
      }

      if key.ends_with('/') && entry.starts_with(key) {
        replace = Some(entry[key.len()..].to_string());
        longest = Some(key.as_str());
      } else if key.len() > 1 {
        if let Some(tmp) = key.find('*') {
          let pattern = format!("^{}(.*){}", &key[..tmp], &key[tmp + 1..]);
          let regex = regex::Regex::new(&pattern).unwrap();

          if let Some(captures) = regex.captures(&entry) {
            if let Some(match_group) = captures.get(1) {
              replace = Some(match_group.as_str().to_string());
              longest = Some(key.as_str());
            }
          }
        }
      }
    }

    if let Some(longest_key) = longest {
      m = mapping.get(&longest_key.to_string());
    }
  }
  if m.is_none() {
    self.throws(name, &entry, None);
    return Vec::new();
  }
  let v = self.loop_value(m.unwrap().clone(), &c, &mut None);
  if v.is_none() {
    self.throws(name, &entry, Some(1));
    return Vec::new();
  }
  let cloned_v = v.clone();
  if let Some(replace) = replace {
    if let Some(v1) = self.injects(&mut cloned_v.unwrap(), &replace) {
      return v1;
    }
  }
  v.unwrap()
}
