//! Command‑line interface for the `dusttest` harness.
//!
//! This binary reads a configuration file (defaulting to `dusttest.toml`),
//! determines a sequence of seeds, and executes each defined test command
//! under each seed.  It reports a summary of pass/fail counts and, if
//! requested, prints per‑seed output.

use std::env;
use std::path::Path;

use dusttest::{format_summary, generate_seeds, load_config, run_tests};

fn print_help() {
    println!("Dusttest – deterministic test harness");
    println!("");
    println!("USAGE: dusttest [OPTIONS]\n");
    println!("Options:");
    println!("    --config <FILE>    Path to a custom configuration file (defaults to dusttest.toml)");
    println!("    --seed <INT>       Use a specific seed instead of a random one");
    println!("    --ensemble <N>     Run each test N times with different seeds");
    println!("    --list             List tests without running them");
    println!("    --verbose          Show per‑run output");
    println!("    -h, --help         Display this help");
}

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    // defaults
    let mut config_path: Option<String> = None;
    let mut seed_arg: Option<u64> = None;
    let mut ensemble_size: usize = 1;
    let mut list_only = false;
    let mut verbose = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--config" => {
                if i + 1 >= args.len() {
                    eprintln!("error: --config requires a path argument");
                    return;
                }
                config_path = Some(args[i + 1].clone());
                args.remove(i);
                args.remove(i);
                continue;
            }
            "--seed" => {
                if i + 1 >= args.len() {
                    eprintln!("error: --seed requires an integer argument");
                    return;
                }
                let val = args[i + 1].parse::<u64>();
                match val {
                    Ok(s) => seed_arg = Some(s),
                    Err(_) => {
                        eprintln!("error: --seed requires an integer argument");
                        return;
                    }
                }
                args.remove(i);
                args.remove(i);
                continue;
            }
            "--ensemble" => {
                if i + 1 >= args.len() {
                    eprintln!("error: --ensemble requires a count argument");
                    return;
                }
                let val = args[i + 1].parse::<usize>();
                match val {
                    Ok(n) => ensemble_size = n,
                    Err(_) => {
                        eprintln!("error: --ensemble requires an integer argument");
                        return;
                    }
                }
                args.remove(i);
                args.remove(i);
                continue;
            }
            "--list" => {
                list_only = true;
                args.remove(i);
                continue;
            }
            "--verbose" => {
                verbose = true;
                args.remove(i);
                continue;
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            _ => {
                i += 1;
            }
        }
    }
    let config_file = config_path.unwrap_or_else(|| "dusttest.toml".to_string());
    let tests = match load_config(&config_file) {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("error: unable to read config {}: {}", config_file, e);
            return;
        }
    };
    if tests.is_empty() {
        println!("No tests defined in {}", config_file);
        return;
    }
    if list_only {
        println!("Defined tests:");
        for test in &tests {
            println!("  {}: {}", test.name, test.cmd);
        }
        return;
    }
    let (base_seed, seeds) = generate_seeds(seed_arg, ensemble_size);
    if seeds.is_empty() {
        println!("No seeds generated: nothing to run.");
        return;
    }
    let results = run_tests(&tests, &seeds, verbose);
    // print verbose outputs first, if requested
    if verbose {
        for result in &results {
            println!("=== Test {} ===", result.name);
            for (seed, output) in &result.outputs {
                println!("--- seed {} ---", seed);
                println!("{}", output);
            }
            println!();
        }
    }
    let summary = format_summary(&results, base_seed);
    println!("{}", summary);
}