
include!(concat!(env!("OUT_DIR"), "/config.rs"));

use std::env::args;
use std::io::Result as IoResult;
use std::result::Result as ResResult;

use renamer_lib::renamer::Renamer;
use renamer_lib::types::FileCount;
use renamer_lib::config::Config;
// use renamer_lib::verbose::Verbose;

mod app;
use crate::app::App;

#[cfg(debug_assertions)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print_app_info() {
    println!("{} v{} ({})", APP_NAME, APP_VERSION, APP_BUILD_AT);
    println!("{}", APP_AUTHORS);
    println!("{}", APP_HOMEPAGE);
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("  renamer [-h] [-c|--config <path>] [[-p|--path <path>]...] [-l|--limit <count>] [-n|--dryrun]");
    println!();
    println!("Options:");
    println!("  -h|--help                Show help.");
    println!("  -V|--version             Show version.");
    println!("  -c|--config <path>       Path to config file.");
    println!("  -p|--path <path>         Path to root directory. Multiple `-p`s possible.");
    println!("  -l|--limit <count>       Limit files to rename.");
    println!("  -n|--dryrun              Do not change anything.");
    println!("  -v|--verbose <level>     Verbose Levels: 1,2,3");
    println!("  -v|-vv|-vvv              Verbose");
}

fn main() -> IoResult<()> {
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
                    app.limit = Some(_next.parse::<FileCount>().unwrap());
                    skip_next = true;
                }
            },
            "-d" | "--maxdepth" => {
                if let Some(_next) = next {
                    let md = _next.parse::<u16>().unwrap();
                    if md <= 0 {
                        panic!("Not sure what to do with max depth {}", md);
                    }
                    app.max_depth = Some(md);
                    skip_next = true;
                }
            },
            "-n" | "--dryrun" => {
                app.dryrun = true;
            },
            "--verbose" => {
                if let Some(_next) = next {
                    if let ResResult::Ok(_next) = _next.parse::<u8>() {
                        // app.verbose = _next;
                    }
                    skip_next = true;
                }
            },
            "-v" => {
                // app.verbose = Some(1);

                if let Some(_next) = next {
                    if let ResResult::Ok(_next) = _next.parse::<u8>() {
                        // app.verbose = Some(_next);
                        skip_next = true;
                    }
                }
            },
            // "-vv" => { app.verbose = Some(2); },
            // "-vvv" => { app.verbose = Some(3); },
            _ => panic!("Unrecognized argument: {}", arg),
        }
    }

    if cfg!(debug_assertions) {
        println!("-> app.config: {:?}", app.config);
        println!("-> app.paths: {:?}", app.paths);
        println!("-> app.limit: {:?}", app.limit);
        println!("-> app.max_depth: {:?}", app.max_depth);
        println!("-> app.dryrun: {:?}", app.dryrun);
        println!("-> app.verbose: {:?}", app.verbose);
    }

    // Config
    let config = Config::from_config_path(app.config);

    // Renamer
    let renamer = Renamer::new(config, app.limit, app.max_depth, app.dryrun, app.verbose);

    let stats = renamer.rename(app.paths);
    if app.verbose >= 1 {
        println!("---------------");
        println!("-> dirs:     {}", stats.dirs);
        println!("-> files:    {}", stats.files);
        println!("-> renamed:  {}", stats.renamed);
        println!("-> errors:   {}", stats.errors);
        println!("-> warnings: {}", stats.warnings);
        println!("-> skipped:  {}", stats.skipped);
        println!("-> rest:     {:?}", stats.rest);
        println!("---------------");
    }

    #[cfg(debug_assertions)]
    println!("-> end");

    Ok(())
}
