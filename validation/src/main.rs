use profile_validation::validate_repository;
use std::{env, path::PathBuf, process};

fn main() {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let root = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    if arguments.next().is_some() {
        eprintln!("Usage: profile-validation [repository-root]");
        process::exit(2);
    }

    match validate_repository(&root) {
        Ok(summary) => println!(
            "Validated {} records, {} groups, {} sources across {} repository files.",
            summary.records, summary.groups, summary.sources, summary.files
        ),
        Err(findings) => {
            eprintln!("Profile validation failed.");
            for finding in findings {
                eprintln!("- {finding}");
            }
            process::exit(1);
        }
    }
}
