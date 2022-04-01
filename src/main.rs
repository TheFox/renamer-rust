
include!(concat!(env!("OUT_DIR"), "/config.rs"));

use std::env::args;
use std::io::Result;
use renamer_lib::app::App;
use renamer_lib::renamer::Renamer;

fn print_app_info() {
    println!("{} v{} ({})", APP_NAME, APP_VERSION, APP_BUILD_AT);
    println!("{}", APP_AUTHORS);
    println!("{}", APP_HOMEPAGE);
    println!("");
}

fn print_usage() {
    println!("Usage:");
    println!("  renamer [-h] [--config <path>] [[--path <path>]...] [--limit <count>] [-n|--dryrun] [--print]");
    println!();
    println!("Options:");
    println!("  -h|--help                Show help.");
    println!("  -c|--config <path>       Path to config file.");
    println!("  -p|--path <path>         Path to root directory. Multiple -p arguments possible.");
    println!("  -l|--limit <count>       Limit files to rename.");
    println!("  -n|--dryrun              Do not change anything.");
    println!("     --print               Print config.");
    println!();
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    println!("-> start");

    let args: Vec<String> = args().collect();
    let argc = args.len();

    if cfg!(debug_assertions) {
        println!("-> args: {:?}", args);
        println!("-> argc: {:?}", argc);
    }

    if argc == 1 {
        print_app_info();
        print_usage();
        return Ok(());
    }

    let mut app = App::new();
    let mut skip_next = false;
    for index in 1..argc {
        if skip_next {
            skip_next = false;
            continue;
        }
        let arg = &args[index];
        let next = &args.get(index + 1);

        #[cfg(debug_assertions)]
        println!("-> arg: #{} {:?}", index, arg);

        match arg.as_str() {
            "-h" | "--help" => {
                print_app_info();
                print_usage();
                return Ok(());
            },
            "-V" | "--version" => {
                print_app_info();
                print_usage();
                return Ok(());
            },
            "-c" | "--config" => {
                if let Some(_next) = next {
                    app.config = Some(_next.to_string());
                    skip_next = true;
                }
            },
            "-p" | "--path" => {
                if let Some(_next) = next {
                    println!("-> next: {:?}", _next);
                    if let Some(_paths) = &mut app.paths {
                        _paths.push(_next.to_string());
                    } else {
                        app.paths = Some(vec![_next.to_string()]);
                    }
                    skip_next = true;
                }
            },
            "-l" | "--limit" => {
                if let Some(_next) = next {
                    app.limit = Some(_next.parse::<usize>().unwrap());
                    skip_next = true;
                }
            },
            "-n" | "--dryrun" => {
                if let Some(_next) = next {
                    app.dryrun = true;
                }
            },
            _ => {
                panic!("Unrecognized argument: {}", arg);
            },
        }
    }

    if cfg!(debug_assertions) {
        println!("-> app.config: {:?}", app.config);
        println!("-> app.paths: {:?}", app.paths);
        println!("-> app.limit: {:?}", app.limit);
        println!("-> app.dryrun: {:?}", app.dryrun);
    }

    let renamer = Renamer::new(app.config);
    renamer.rename(app.paths, app.limit, app.dryrun);

    #[cfg(debug_assertions)]
    println!("-> end");

    Ok(())
}

#[cfg(test)]
mod tests_base {
    #[test]
    fn test1() {
        assert!(true);
    }
}
