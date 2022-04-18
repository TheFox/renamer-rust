
use core::result::Result;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use log::debug;

use serde::Serialize;
use serde::Deserialize;
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
type Finds = HashMap<String, Vec<String>>;
type FindsOption = Option<Finds>;
type RegexFinds = Vec<(Regex, Vec<String>)>;
type RegexFindsOption = Option<RegexFinds>;
pub type ConfigOption = Option<Config>;

#[cfg(debug_assertions)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Deserialize, Clone)]
struct Function {
    #[serde(alias = "fn")]
    name: Option<String>,

    search: Option<String>,
    replace: Option<String>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            name: None,
            search: None,
            replace: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Var {
    #[serde(alias = "type")]
    vtype: Option<String>,

    format: Option<String>,
    default: Option<u64>,
    fns: Option<Vec<Function>>,
}

impl Var {
    pub fn new() -> Self {
        Self {
            vtype: None,
            format: None,
            default: None,
            fns: None,
        }
    }

    pub fn from(vtype: String, format: String) -> Self {
        Self {
            vtype: Some(vtype),
            format: Some(format),
            default: None,
            fns: None,
        }
    }

    fn format(&self, ovalue: String) -> String {
        lazy_static! {
            static ref re: Regex = Regex::new(r"%(\d+)?.").unwrap();
        }

        let vtype: &str = match &self.vtype {
            Some(_vtype) => &_vtype,
            None => panic!("Cannot format variable because no type was povieded"),
        };

        let format = match &self.format {
            Some(_format) => _format,
            None => panic!("No format defined for variable"),
        };

        let caps = re.captures(&format).unwrap();

        let fvalue = match vtype {
            "int" => {
                match &caps[0] {
                    "%02d" => format!("{:0>2}", ovalue),
                    "%03d" => format!("{:0>3}", ovalue),

                    "%2d" => format!("{:>2}", ovalue),
                    "%3d" => format!("{:>3}", ovalue),

                    "%d" => format!("{}", ovalue),

                    _ => panic!("Format not implemented for {}: {}", vtype, format),
                }
            },
            "str" => {
                match &caps[0] {
                    "%02s" => format!("{:0>2}", ovalue),
                    "%03s" => format!("{:0>3}", ovalue),

                    "%2s" => format!("{:>2}", ovalue),
                    "%3s" => format!("{:>3}", ovalue),

                    "%s" => format!("{}", ovalue),

                    _ => panic!("Format not implemented for {}: {}", vtype, format),
                }
            },
            _ => panic!("Type not implemented: {}", vtype),
        };

        format.replace(&caps[0], &fvalue)
    }

    pub fn format_s(&self, value: &str) -> String {
        self.format(value.to_string())
    }

    /// Push Function
    pub fn push(&mut self, _fn: Function) {
        match &mut self.fns {
            Some(_fns) => {
                _fns.push(_fn);
            },
            None => {
                self.fns = Some(vec![_fn]);
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(skip)]
    is_initialized: bool,

    #[serde(alias = "root")]
    is_root: Option<bool>,

    #[serde(skip)]
    errors: Option<bool>,

    name: Option<String>,
    exts: ExtsOption,
    vars: VarsOption,

    #[serde(alias = "find")]
    finds: FindsOption,

    #[serde(skip)]
    regex_finds: RegexFindsOption,
}

impl Config {
    pub fn new() -> Self {
        // println!("{}-> Config::new(){}", BLUE, NO_COLOR);
        Self {
            is_initialized: false,
            is_root: None,
            errors: None,
            name: None,
            exts: None,
            vars: None,
            finds: None,
            regex_finds: None,
        }
    }

    pub fn from_str(s: &String) -> Self {
        // println!("{}-> Config::from_str(){}", BLUE, NO_COLOR);
        let mut c: Self = from_str(s).expect("JSON was not well-formatted");
        c.setup();
        c
    }

    pub fn from_path(path: String) -> Self {
        let data: String = read_to_string(path).expect("Unable to read file");
        Self::from_str(&data)
    }

    pub fn from_config_path(config_path: ConfigPath) -> Self {
        // println!("{}-> config path: {:?}{}", BLUE, config_path, NO_COLOR);
        match config_path {
            Some(_config_path) => Self::from_path(_config_path),
            None => Self::new(),
        }
    }

    pub fn from_path_buf(path_buf: PathBuf) -> Self {
        Self::from_path(path_buf.display().to_string())
    }

    fn setup(&mut self) {
        // println!("{}-> Config::setup(){}", BLUE, NO_COLOR);

        self.is_initialized = true;
        self.setup_regex_finds();

        // match &self.name {
        //     Some(name) => {},
        //     None => panic!("Config doesn't have a name"),
        // }
    }

    fn setup_regex_finds(&mut self) {
        if let Some(_finds) = &self.finds {
            let mut regex_finds = RegexFinds::new();
            for (regex_s, vars_a) in _finds {

                match Regex::new(regex_s) {
                    Result::Ok(_r) => {
                        let find = (_r, vars_a.clone());
                        regex_finds.push(find);
                    },
                    Result::Err(_e) => {
                        panic!("Cannot parse 'finds' regexp: {}", regex_s);
                    },
                }
            }
            self.regex_finds = Some(regex_finds);
        }
    }

    fn merge_exts(&self, other: &Config) -> ExtsOption {
        if self.exts.is_some() && other.exts.is_some() {
            let mut exts: Exts = self.exts.as_ref().unwrap().clone();
            let _exts = other.exts.as_ref().unwrap();

            for _ext in _exts {
                if !exts.contains(_ext) {
                    exts.push(_ext.clone());
                }
            }

            Some(exts)
        } else if other.exts.is_some() {
            Some(other.exts.as_ref().unwrap().clone())
        } else if self.exts.is_some() {
            Some(self.exts.as_ref().unwrap().clone())
        } else {
            None
        }
    }

    fn merge_vars(&self, other: &Config) -> VarsOption {
        if self.vars.is_some() && other.vars.is_some() {
            let _vars1 = self.vars.as_ref().unwrap();
            let _vars2 = other.vars.as_ref().unwrap();

            let mut vars: Vars = _vars1.clone();

            for (name, _var2) in _vars2 {
                if vars.contains_key(name) {
                    let _var1 = vars.get_mut(name).unwrap();

                    if let Some(_type) = &_var2.vtype {
                        _var1.vtype = Some(_type.clone());
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
                    vars.insert(name.into(), Var::new());
                }
            }

            Some(vars)
        } else if other.vars.is_some() {
            Some(other.vars.as_ref().unwrap().clone())
        } else if self.vars.is_some() {
            Some(self.vars.as_ref().unwrap().clone())
        } else {
            None
        }
    }

    fn merge_finds(&self, other: &Config) -> FindsOption {
        if self.finds.is_some() && other.finds.is_some() {
            let mut finds: Finds = self.finds.as_ref().unwrap().clone();

            let _finds = other.finds.as_ref().unwrap();
            for (name, _find) in _finds {
                if finds.contains_key(name) {
                    *finds.get_mut(name).unwrap() = _find.clone();
                } else {
                    finds.insert(name.into(), _find.clone());
                }
            }

            Some(finds)
        } else if other.finds.is_some() {
            Some(other.finds.as_ref().unwrap().clone())
        } else if self.finds.is_some() {
            Some(self.finds.as_ref().unwrap().clone())
        } else {
            None
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        // println!("{}-> Config::merge(){}", BLUE, NO_COLOR);

        let mut config = Config::new();

        // Root
        // if let Some(_is_root) = &other.is_root {
        //     panic!("not implemented: merge root field");
        // }

        // Errors
        // if let Some(_errors) = &other.errors {
        //     panic!("not implemented: merge errors field");
        // }

        // Name
        if let Some(_name) = &other.name {
            config.name = Some(_name.clone());
        }

        // Exts
        config.exts = self.merge_exts(&other);

        // Vars
        config.vars = self.merge_vars(&other);

        // Finds
        config.finds = self.merge_finds(&other);

        // Setup
        config.setup();

        config
    }

    pub fn has_ext(&self, ext: &String) -> bool {
        match &self.exts {
            Some(_exts) => _exts.contains(ext),
            None => false,
        }
    }

    pub fn is_root(&self) -> bool {
        match &self.is_root {
            Some(is_root) => *is_root,
            None => false,
        }
    }

    pub fn has_name(&self) -> bool {
        match &self.name {
            Some(name) => name.len() > 0,
            None => false,
        }
    }

    pub fn name(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => {
                dbg!(self); // TODO
                panic!("Config doesn't have a name")
            },
        }
    }

    pub fn regex_finds(&self) -> RegexFinds {
        match &self.regex_finds {
            Some(_x) => _x.to_vec(),
            None => {
                // let _x = vec![];
                // &_x
                RegexFinds::new()
            },
        }
    }

    pub fn format_var(&self, name: String, value: String) -> String {
        match &self.vars {
            Some(_vars) => {
                match &_vars.get(&name) {
                    Some(_var) => {
                        _var.format(value)
                    },
                    None => panic!("Cariable not found: {}", name),
                }
            },
            None => panic!("No variables defined in Config"),
        }

    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn path_has_var(&self, path: &Path) -> bool {
        // println!("{}-> Config::path_has_var({:?}){}", BLUE, path, NO_COLOR);

        let path_s: String = path.display().to_string();

        match &self.vars {
            Some(_vars) => {
                // println!("-> vars: {:?}", _vars);
                // println!("-> keys: {:?}", _vars.keys());
                for var_name in _vars.keys() {
                    // println!("-> var: {:?}", var_name);
                    if path_s.contains(var_name) {
                        return true;
                    }
                }
            },
            None => {},
        }

        false
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
mod tests_var {
    use super::Var;

    #[test]
    fn test_var() {
        let v1 = Var::from("v1".into(), "S%02d".into());

        assert_eq!("v1".to_string(), v1.vtype.unwrap());
        assert_eq!("S%02d".to_string(), v1.format.unwrap());
    }

    #[test]
    fn test_var_format() {
        let v1 = Var::from("str".into(), "S%2s".into());
        assert_eq!("S 1".to_string(), v1.format_s("1"));
        assert_eq!("S10".to_string(), v1.format_s("10"));
        assert_eq!("S100".to_string(), v1.format_s("100"));
    }

    #[test]
    fn test_var_format1() {
        let v1 = Var::from("str".into(), "S%02s".into());
        assert_eq!("S01".to_string(), v1.format_s("1"));
        assert_eq!("S10".to_string(), v1.format_s("10"));
        assert_eq!("S100".to_string(), v1.format_s("100"));
    }

    #[test]
    fn test_var_format2() {
        let v1 = Var::from("str".into(), "S%03s".into());
        assert_eq!("S001".to_string(), v1.format_s("1"));
        assert_eq!("S010".to_string(), v1.format_s("10"));
        assert_eq!("S100".to_string(), v1.format_s("100"));
    }
}

#[cfg(test)]
mod tests_config {
    use std::collections::HashMap;
    use super::Finds;
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
            source_c1.is_root = Some(_t.0);

            let mut source_c2 = Config::new();
            source_c2.is_root = Some(_t.1);

            let merged_c3 = source_c1.merge(&source_c2);

            assert_eq!(_t.2, merged_c3.is_root.unwrap());
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
    fn test_config_merge_exts1() {
        let mut source_c1 = Config::new();
        source_c1.exts = Some(vec!["ext1".into(), "ext2".into()]);

        let source_c2 = Config::new();

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(2, source_c1.exts.as_ref().unwrap().len());
        assert!(source_c2.exts.as_ref().is_none());
        assert_eq!(2, merged_c3.exts.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_merge_exts2() {
        let source_c1 = Config::new();

        let mut source_c2 = Config::new();
        source_c2.exts = Some(vec!["ext3".into(), "ext2".into()]);

        let merged_c3 = source_c1.merge(&source_c2);

        assert!(source_c1.exts.as_ref().is_none());
        assert_eq!(2, source_c2.exts.as_ref().unwrap().len());
        assert_eq!(2, merged_c3.exts.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_merge_vars() {
        let mut fn1 = Function::new();
        fn1.name = Some("fn1".into());

        let mut fn2 = Function::new();
        fn2.name = Some("fn2".into());

        let mut var1a = Var::new();
        var1a.vtype = Some("int1".into());
        var1a.format = Some("f1".into());
        var1a.push(fn1);

        let mut var1b = Var::new();
        var1b.vtype = Some("int2".into());
        var1b.format = Some("f2".into());
        var1b.push(fn2);

        let mut var4a = Var::new();
        var4a.vtype = Some("int4".into());

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
        assert_eq!("int2", merged_c3.vars.as_ref().unwrap()["var1"].vtype.as_ref().unwrap());
        assert_eq!("f2", merged_c3.vars.as_ref().unwrap()["var1"].format.as_ref().unwrap());

        assert_eq!("int4", merged_c3.vars.as_ref().unwrap()["var4"].vtype.as_ref().unwrap());
        assert_eq!("f4", merged_c3.vars.as_ref().unwrap()["var4"].format.as_ref().unwrap());
        assert_eq!("f4", merged_c3.vars.as_ref().unwrap()["var4"].format.as_ref().unwrap());
    }

    #[test]
    fn test_config_merge_vars1() {
        let mut vars1: HashMap<String, Var> = HashMap::new();
        vars1.insert("var1".into(), Var::new());
        vars1.insert("var2".into(), Var::new());

        let mut source_c1 = Config::new();
        source_c1.vars = Some(vars1);

        let mut source_c2 = Config::new();

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(2, source_c1.vars.as_ref().unwrap().len());
        assert!(source_c2.vars.is_none());
        assert_eq!(2, merged_c3.vars.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_merge_vars2() {
        let mut vars2: HashMap<String, Var> = HashMap::new();
        vars2.insert("var1".into(), Var::new());
        vars2.insert("var2".into(), Var::new());

        let mut source_c1 = Config::new();

        let mut source_c2 = Config::new();
        source_c2.vars = Some(vars2);

        let merged_c3 = source_c1.merge(&source_c2);

        assert!(source_c1.vars.is_none());
        assert_eq!(2, source_c2.vars.as_ref().unwrap().len());
        assert_eq!(2, merged_c3.vars.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_merge_finds() {
        let mut finds1 = Finds::new();
        finds1.insert("regex1".into(), vec!["var1".into(), "var2".into(), "var3".into()]);
        finds1.insert("regex2".into(), vec!["var1".into(), "var2".into()]);

        let mut finds2 = Finds::new();
        finds2.insert("regex1".into(), vec!["var5".into(), "var6".into()]);
        finds2.insert("regex3".into(), vec!["var3".into(), "var4".into()]);

        let mut source_c1 = Config::new();
        source_c1.finds = Some(finds1);

        let mut source_c2 = Config::new();
        source_c2.finds = Some(finds2);

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(3, merged_c3.finds.as_ref().unwrap().len());

        assert_eq!(2, merged_c3.finds.as_ref().unwrap()["regex1"].len());
        assert_eq!("var5", merged_c3.finds.as_ref().unwrap()["regex1"][0]);
        assert_eq!("var6", merged_c3.finds.as_ref().unwrap()["regex1"][1]);

        assert_eq!(2, merged_c3.finds.as_ref().unwrap()["regex2"].len());
        assert_eq!("var1", merged_c3.finds.as_ref().unwrap()["regex2"][0]);
        assert_eq!("var2", merged_c3.finds.as_ref().unwrap()["regex2"][1]);

        assert_eq!(2, merged_c3.finds.as_ref().unwrap()["regex3"].len());
        assert_eq!("var3", merged_c3.finds.as_ref().unwrap()["regex3"][0]);
        assert_eq!("var4", merged_c3.finds.as_ref().unwrap()["regex3"][1]);
    }

    #[test]
    fn test_config_merge_finds1() {
        let mut finds1 = Finds::new();
        finds1.insert("regex1".into(), vec![]);

        let mut source_c1 = Config::new();
        source_c1.finds = Some(finds1);

        let source_c2 = Config::new();

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(1, merged_c3.finds.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_merge_finds2() {
        let mut finds2 = Finds::new();
        finds2.insert("regex1".into(), vec![]);

        let mut source_c1 = Config::new();

        let mut source_c2 = Config::new();
        source_c2.finds = Some(finds2);

        let merged_c3 = source_c1.merge(&source_c2);

        assert_eq!(1, merged_c3.finds.as_ref().unwrap().len());
    }

    #[test]
    fn test_config_regex_finds1() {
        let mut source_c1 = Config::new();
        assert!(source_c1.regex_finds.is_none());
    }

    #[test]
    fn test_config_regex_finds2() {
        let data: String = r#"{"finds":{"^a":["%var1%"]}}"#.into();
        let mut source_c1 = Config::from_str(&data);
        assert_eq!(1, source_c1.finds.as_ref().unwrap().len());
        assert_eq!(1, source_c1.regex_finds.as_ref().unwrap().len());

        let mut source_c2 = source_c1.clone();
        assert_eq!(1, source_c2.finds.as_ref().unwrap().len());
        assert_eq!(1, source_c2.regex_finds.as_ref().unwrap().len());
    }
}
