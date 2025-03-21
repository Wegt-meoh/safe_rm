use std::io::Write;

use clap::{Arg, ArgAction, Command};
use glob::glob;

fn main() {
    // Define the command-line arguments and options
    let matches = Command::new("MyCLI")
        .version("1.0")
        .author("Wegt-meoh <wegt-meoh@outlook.com>")
        .about("safe remove your files into system trash")
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force removal of files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Prompt before every removal")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .help("All positional arguments")
                .num_args(1..)
                .index(1), // This is the first positional argument
        )
        .get_matches();

    let patterns: Vec<_> = matches
        .get_many::<String>("input")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    let is_verbose = matches.get_flag("verbose");
    let is_force = matches.get_flag("force");
    let is_interactive = {
        if is_force {
            false
        } else {
            matches.get_flag("interactive")
        }
    };

    let log = |text: String| {
        if is_verbose {
            println!("{text}")
        }
    };

    let err_log = |text: String| {
        if is_verbose {
            eprintln!("{text}")
        }
    };
    for pattern in patterns {
        // `glob` returns an iterator, so we need to handle errors here.

        let entries: Vec<_> = glob(pattern)
            .expect("Can not read glob pattern")
            .filter_map(Result::ok)
            .collect();

        for path_buf in entries {
            let path_str = path_buf.to_string_lossy().into_owned();
            if is_interactive && !get_confirmation(&format!("confirm to remove {}", path_str)) {
                continue;
            }
            match trash::delete(path_buf) {
                Ok(()) => log(format!(
                    "safe_rm: Successfully moved {:?} to trash",
                    path_str
                )),
                Err(e) => err_log(format!("Error deleting {:?}: {}", path_str, e)),
            }
        }
    }
}

fn get_confirmation(q: &str) -> bool {
    loop {
        let mut input = String::new();
        print!("{q}? (y/n): ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}
