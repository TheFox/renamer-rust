
use std::fs::read_to_string;
use std::path::PathBuf;
use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;
use serde_json::from_str;
use serde_json::Value;

use crate::types::ConfigPath;
use crate::colors::NO_COLOR;
use crate::colors::RED;
use crate::colors::BLUE;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn merge_vars(config: &mut Config, other: &Config) {
    println!("-> merge_vars");

    if config.vars.is_some() && other.vars.is_some() {
        let _vars1 = config.vars.as_ref().unwrap();
        let _vars2 = other.vars.as_ref().unwrap();

        println!("{}-> merge A vars{}", BLUE, NO_COLOR);
        println!("  -> vars1: {:?}", _vars1);
        println!("  -> vars2: {:?}", _vars2);

        for (name, _c) in _vars1 {
            println!("  -> var1: '{}' {:?}", name, _c);
            if _vars2.contains_key(name) {}
            else {
                _vars2.insert(name.to_string(), *_c);
            }
        }
    } else if other.vars.is_some() {
        println!("{}-> merge B vars{}", BLUE, NO_COLOR);
        panic!("not implemented B");
    } else if config.vars.is_some() {
        println!("{}-> merge C vars{}", BLUE, NO_COLOR);
        panic!("not implemented C");
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Function {
    r#fn: String,
    search: Option<String>,
    replace: Option<String>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            r#fn: String::from("none"),
            search: None,
            replace: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Var {
    r#type: String,
    format: Option<String>,
    default: Option<u64>,
    fns: Option<Vec<Function>>,
}

impl Var {
    pub fn new() -> Self {
        Self {
            r#type: String::from("none"),
            format: None,
            default: None,
            fns: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    root: Option<bool>,
    errors: Option<bool>,
    name: Option<String>,
    exts: Option<Vec<String>>,
    vars: Option<HashMap<String, Var>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root: None,
            errors: None,
            name: None,
            exts: None,
            vars: None,
        }
    }

    pub fn from_path(path: String) -> Self {
        let data = read_to_string(path).expect("Unable to read file");

        // if cfg!(debug_assertions) {
        //     let json: Value = from_str(&data).expect("JSON does not have correct format.");
        //     dbg!(json);
        // }

        from_str(&data).expect("JSON was not well-formatted")
    }

    pub fn from_config_path(config_path: ConfigPath) -> Self {
        println!("{}-> config path: {:?}{}", BLUE, config_path, NO_COLOR);
        match config_path {
            Some(_config_path) => Self::from_path(_config_path),
            None => Self::new(),
        }
    }

    pub fn from_path_buf(path_buf: PathBuf) -> Self {
        Self::from_path(path_buf.display().to_string())
    }

    pub fn merge(&mut self, other: &Self) -> Self {
        println!("{}-> merge{}", BLUE, NO_COLOR);

        let mut config = self.clone();

        // Root
        if let Some(_root) = &other.root {
            println!("{}-> merge root: {:?}{}", BLUE, _root, NO_COLOR);
            // print_type_of(_root);
            config.root = Some(*_root);
        }

        // Errors
        if let Some(_errors) = &other.errors {
            println!("{}-> merge errors: {:?}{}", BLUE, _errors, NO_COLOR);
            // print_type_of(_errors);
            config.errors = Some(*_errors);
        }

        // Name
        if let Some(_name) = &other.name {
            println!("{}-> merge name: {:?}{}", BLUE, _name, NO_COLOR);
            // print_type_of(&_name);
            config.name = Some(_name.to_string());
        }

        // Vars
        merge_vars(&mut config, &other);

        config
    }
}

#[cfg(test)]
mod tests_config {
    use std::collections::HashMap;
    use super::Function;
    use super::Var;
    use super::Config;

    #[test]
    fn test_config_merge_root() {
        let _data: Vec<(bool, bool, bool)> = vec![
            (false, false, false),
            (true, false, false),
            (false, true, true),
        ];

        for _t in _data {
            let mut source_c1 = Config::new();
            source_c1.root = Some(_t.0);

            let mut source_c2 = Config::new();
            source_c2.root = Some(_t.1);

            let merged_c3 = source_c1.merge(&source_c2);

            assert_eq!(_t.2, merged_c3.root.unwrap());
        }
    }

    #[test]
    fn test_config_merge_name() {
        let mut source_c1 = Config::new();
        source_c1.name = Some(String::from("c1"));

        let mut source_c2 = Config::new();
        source_c2.name = Some(String::from("c2"));

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!("c2".to_owned(), merged_c3.name.unwrap());
    }

    #[test]
    fn test_config_merge_vars() {
        let mut vars1: HashMap<String, Var> = HashMap::new();
        vars1.insert("v1".to_string(), Var::new());
        vars1.insert("v2".to_string(), Var::new());

        let mut vars2: HashMap<String, Var> = HashMap::new();
        vars2.insert("v1".to_string(), Var::new());
        vars2.insert("v3".to_string(), Var::new());

        let mut source_c1 = Config::new();
        source_c1.vars = Some(vars1);

        let mut source_c2 = Config::new();
        source_c2.vars = Some(vars2);

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(2, merged_c3.vars.unwrap().len());
    }

    type TestConfig = bool;

    #[test]
    fn test_config_merge_all() {
        let _data: Vec<(TestConfig, TestConfig, TestConfig)> = vec![
            (
                (false),
                (false),
                (false),
            ),
        ];

        for _t in _data {
            dbg!(_t);
            let mut source_c1 = Config::new();
            source_c1.root = Some(_t.0);

            let mut source_c2 = Config::new();
            source_c2.root = Some(_t.1);

            let merged_c3 = source_c1.merge(&source_c2);

            assert_eq!(_t.2, merged_c3.root.unwrap());
        }
    }
}
