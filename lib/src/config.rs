
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

type Exts = Vec<String>;
type ExtsOption = Option<Exts>;
type Vars = HashMap<String, Var>;
type VarsOption = Option<Vars>;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn merge_exts(config: &Config, other: &Config) -> ExtsOption {
    println!("-> merge_exts()");

    if config.exts.is_some() && other.exts.is_some() {
        let mut exts: Exts = config.exts.as_ref().unwrap().clone();
        // dbg!(exts);

        let _exts = other.exts.as_ref().unwrap();
        // dbg!(_exts);

        for _ext in _exts {
            if !exts.contains(_ext) {
                exts.push(_ext.clone());
            }
        }

        Some(exts)
    } else if other.exts.is_some() {
        panic!("merge exts not implemented B");
        None
    } else if config.exts.is_some() {
        panic!("merge exts not implemented C");
        None
    } else {
        None
    }
}

fn merge_vars(config: &Config, other: &Config) -> VarsOption {
    // println!("-> merge_vars()");

    if config.vars.is_some() && other.vars.is_some() {
        let _vars1 = config.vars.as_ref().unwrap();
        let _vars2 = other.vars.as_ref().unwrap();

        // println!("{}-> merge A vars{}", BLUE, NO_COLOR);
        // println!("  -> vars1: {:?}", _vars1);
        // println!("  -> vars2: {:?}", _vars2);

        let mut vars: Vars = _vars1.clone();

        for (name, _var2) in _vars2 {
            // println!("  -> var2: '{}' {:?}", name, _var2);

            if vars.contains_key(name) {
                let _var1 = vars.get_mut(name).unwrap();

                if let Some(_type) = &_var2.r#type {
                    _var1.r#type = Some(_type.clone());
                }
                if let Some(_format) = &_var2.format {
                    _var1.format = Some(_format.clone());
                }
                if let Some(_default) = &_var2.default {
                    _var1.default = Some(_default.clone());
                }
                if let Some(_fns) = &_var2.fns {
                    for _fn in _fns {
                        // println!("  -> fn: {:?}", _fn);
                        _var1.push(_fn.clone());
                    }
                }
            }
            else {
                vars.insert(name.to_string(), Var::new());
            }
        }

        Some(vars)
    } else if other.vars.is_some() {
        println!("{}-> merge B vars{}", BLUE, NO_COLOR);
        panic!("merge vars not implemented B");
        None
    } else if config.vars.is_some() {
        println!("{}-> merge C vars{}", BLUE, NO_COLOR);
        panic!("merge vars not implemented C");
        None
    } else {
        None
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Function {
    r#fn: Option<String>,
    search: Option<String>,
    replace: Option<String>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            r#fn: None,
            search: None,
            replace: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Var {
    r#type: Option<String>,
    format: Option<String>,
    default: Option<u64>,
    fns: Option<Vec<Function>>,
}

impl Var {
    pub fn new() -> Self {
        Self {
            r#type: None,
            format: None,
            default: None,
            fns: None,
        }
    }

    pub fn push(&mut self, r#fn: Function) {
        match &mut self.fns {
            Some(_fns) => {
                _fns.push(r#fn);
            },
            None => {
                self.fns = Some(vec![r#fn]);
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    root: Option<bool>,
    errors: Option<bool>,
    name: Option<String>,
    exts: ExtsOption,
    vars: VarsOption,
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

        let mut config = Config::new();

        // Root
        if let Some(_root) = &other.root {
            println!("{}-> merge root: {:?}{}", BLUE, _root, NO_COLOR);
            // config.root = Some(*_root);
        }

        // Errors
        if let Some(_errors) = &other.errors {
            println!("{}-> merge errors: {:?}{}", BLUE, _errors, NO_COLOR);
            // config.errors = Some(*_errors);
        }

        // Name
        if let Some(_name) = &other.name {
            println!("{}-> merge name: {:?}{}", BLUE, _name, NO_COLOR);
            // config.name = Some(_name.to_string());
        }

        // Exts
        config.exts = merge_exts(&self, &other);

        // Vars
        config.vars = merge_vars(&self, &other);

        // println!("-> new config: {:?}", config);
        // dbg!(&config);

        config
    }
}

#[cfg(test)]
mod tests_vec {
    #[test]
    fn test_merge() {
        let mut v1: Vec<u8> = vec![1, 2, 3, 4];
        let mut v2: Vec<u8> = vec![21, 42];

        v1.append(&mut v2);

        assert_eq!(6, v1.len());
        assert_eq!(0, v2.len());
    }
}

#[cfg(test)]
mod tests_config {
    use std::collections::HashMap;
    use super::Function;
    use super::Var;
    use super::Config;

    // #[test]
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

    // #[test]
    fn test_config_merge_name() {
        let mut source_c1 = Config::new();
        source_c1.name = Some(String::from("c1"));

        let mut source_c2 = Config::new();
        source_c2.name = Some(String::from("c2"));

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!("c2".to_owned(), merged_c3.name.unwrap());
    }

    #[test]
    fn test_config_merge_exts() {
        let mut source_c1 = Config::new();
        source_c1.exts = Some(vec!["ext1".into(), "ext2".into()]);

        let mut source_c2 = Config::new();
        source_c2.exts = Some(vec!["ext3".into(), "ext2".into()]);

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(2, source_c1.exts.as_ref().unwrap().len());
        assert_eq!(2, source_c2.exts.as_ref().unwrap().len());
        assert_eq!(3, merged_c3.exts.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_merge_vars() {
        let mut fn1 = Function::new();
        fn1.r#fn = Some("fn1".into());

        let mut fn2 = Function::new();
        fn2.r#fn = Some("fn2".into());

        let mut var1a = Var::new();
        var1a.r#type = Some("int1".into());
        var1a.format = Some("f1".into());
        var1a.push(fn1);

        let mut var1b = Var::new();
        var1b.r#type = Some("int2".into());
        var1b.format = Some("f2".into());
        var1b.push(fn2);

        let mut var4a = Var::new();
        var4a.r#type = Some("int4".into());

        let mut var4b = Var::new();
        var4b.format = Some("f4".into());

        let mut vars1: HashMap<String, Var> = HashMap::new();
        vars1.insert("var1".into(), var1a);
        vars1.insert("var2".into(), Var::new());
        vars1.insert("var4".into(), var4a);

        let mut vars2: HashMap<String, Var> = HashMap::new();
        vars2.insert("var1".into(), var1b);
        vars2.insert("var3".into(), Var::new());
        vars2.insert("var4".into(), var4b);

        let mut source_c1 = Config::new();
        source_c1.vars = Some(vars1);

        let mut source_c2 = Config::new();
        source_c2.vars = Some(vars2);

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(4, merged_c3.vars.as_ref().unwrap().len());
        assert_eq!(&"int2".to_string(), merged_c3.vars.as_ref().unwrap()["var1"].r#type.as_ref().unwrap());
        assert_eq!(&"f2".to_string(), merged_c3.vars.as_ref().unwrap()["var1"].format.as_ref().unwrap());

        assert_eq!(&"int4".to_string(), merged_c3.vars.as_ref().unwrap()["var4"].r#type.as_ref().unwrap());
        assert_eq!(&"f4".to_string(), merged_c3.vars.as_ref().unwrap()["var4"].format.as_ref().unwrap());
    }

    type TestConfig = bool;

    // #[test]
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
