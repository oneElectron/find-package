mod colors;
use colors::*;

mod macros;

use std::cmp::Ordering;

use find_core::{find_installed_package_managers, parse_filters, Repo};

// This is the list of supported package managers
// impl_enum_repo!((homebrew, "brew"), (pacman, "pacman"));

fn main() {
    #[cfg(feature = "benchmark")]
    let start_time = std::time::Instant::now();

    let args = std::env::args().collect::<Vec<String>>();
    let args = parse_options(&args[1..]);

    let installed_package_managers = if args.filter == 0 {
        Some(std::thread::spawn(find_installed_package_managers))
    } else {
        None
    };

    let c = std::fs::read_to_string(path()).unwrap();

    let lines: Vec<&str> = c.lines().collect();

    let mut results: Vec<SearchResult> = vec![];

    #[cfg(feature = "benchmark")]
    let start_time = std::time::Instant::now();

    for query in &args.queries {
        let target = query.to_ascii_lowercase();

        let executable = lines.binary_search_by(|s| {
            let s = &s[..s.find('|').unwrap()];

            if *s > *target {
                return Ordering::Greater;
            } else if *s < *target {
                return Ordering::Less;
            }

            Ordering::Equal
        });

        if executable.is_err() {
            continue;
        }

        let executable = executable.unwrap();

        let mut cur_results = parse_line(lines[executable]);

        for result in cur_results.iter_mut() {
            result.query = Some(query);
        }

        results.append(&mut cur_results);
    }

    #[cfg(feature = "benchmark")]
    {
        let end_time = std::time::Instant::now();
        println!(
            "Searching took: {} ms",
            (end_time - start_time).as_micros() as f64 / 1000.0
        );
    }

    results.sort_by(|a, b| {
        if (a.repo as u64) > (b.repo as u64) {
            Ordering::Greater
        } else if (a.repo as u64) < (b.repo as u64) {
            Ordering::Less
        } else if a.pkg_name > b.pkg_name {
            Ordering::Greater
        } else if a.pkg_name < b.pkg_name {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let filter_package_managers = if args.filter == 0 {
        installed_package_managers.unwrap().join().unwrap_or(0)
    } else {
        args.filter
    };

    for query in args.queries {
        // ? Maybe we should rethink the format of the output
        let r: Vec<&SearchResult<'_>> = results
            .iter()
            .filter(|r| {
                r.query.unwrap() == query && (1 << r.repo as u64) & filter_package_managers != 0
            })
            .collect();
        if !r.is_empty() {
            println!(
                "The program '{}' can be found in the following packages:",
                query
            );
            for result in r {
                println!("  * {}, found in {}", result.pkg_name, result.repo.to_str());
            }
        }
    }

    #[cfg(feature = "benchmark")]
    {
        let end_time = std::time::Instant::now();
        println!(
            "Overall time taken: {} ms",
            (end_time - start_time).as_micros() as f64 / 1000.0
        );
    }
}

fn path() -> std::path::PathBuf {
    if let Ok(p) = std::env::var("XDG_DATA_HOME") {
        return std::path::PathBuf::from(p).join(".local/share/find_package/bin_database.filedb");
    } else {
        return std::path::PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local/share/find_package/bin_database.filedb");
    }
}

fn parse_line(line: &str) -> Vec<SearchResult> {
    let mut output = vec![];
    let mut f = line.find('|').unwrap() + 1;
    let line = &line[f..];

    let mut b = 0usize;

    loop {
        f = b;
        b = f + line[f..].find(':').unwrap();

        let repo = Repo::from(&line[f..b]);

        f = b + 1;
        if let Some(sb) = line[f..].find(',') {
            b = f + sb;
            output.push(SearchResult {
                query: None,
                pkg_name: &line[f..b],
                repo,
            });
        } else {
            output.push(SearchResult {
                query: None,
                pkg_name: &line[f..],
                repo,
            });

            break;
        }
        b += 1;
    }

    output
}

struct SearchResult<'a> {
    query: Option<&'a str>,
    pkg_name: &'a str,
    repo: Repo,
}

struct Arguments<'a> {
    explain: bool,
    verbose: bool,
    queries: Vec<&'a str>,
    filter: u64,
}

fn parse_options(args: &[String]) -> Arguments<'_> {
    let mut arguments = Arguments {
        explain: false,
        verbose: false,
        queries: vec![],
        filter: 0,
    };

    let mut i = args.iter();

    loop {
        if let Some(arg) = i.next() {
            if *arg == "-h" || *arg == "--help" {
                print_help();
                std::process::exit(0);
            } else if *arg == "--explain" {
                arguments.explain = true;
            } else if *arg == "-v" || *arg == "--verbose" {
                arguments.verbose = true;
            } else if *arg == "-f" || *arg == "--filter" {
                if let Some(filters) = i.next() {
                    arguments.filter = parse_filters(&filters);
                } else {
                    println!("{RED}Error{RESET}: filter unspecified");
                }
            } else if !arg.starts_with('-') {
                arguments.queries.push(arg);
            }
        } else {
            break;
        }
    }

    arguments
}

fn print_help() {
    println!("{BOLD}Usage: package-not-found{RESET} [{BOLD}--explain{RESET}] {ULINE}command{RESET} [...]");
    println!();
    println!("Supported package managers: homebrew, pacman");
    println!();
    println!("Prints the packages which provides the given command.");
    println!();
    println!("      --explain                    Output explanation of how to get 'cmd' by");
    println!("                                   installing one of the providing formulae.");
    println!("  -f  --filter <filters>           Comma separated list of package managers to filter for (e.g. homebrew,pacman)");
    println!("  -v, --verbose                    Make some output more verbose.");
    println!("  -h, --help                       Show this message.")
}
