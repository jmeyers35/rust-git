use clap::{App, Arg, SubCommand};
use std::env::current_dir;
use std::path::PathBuf;
use std::str::FromStr;

mod librustgit;
use librustgit::create_repository;

fn main() -> std::io::Result<()> {
    let matches = App::new("Rust Git")
        .subcommand(SubCommand::with_name("init").arg(Arg::with_name("path").required(false)))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_matches)) => {
            let path = sub_matches.value_of("path").unwrap();
            create_repository(PathBuf::from_str(path).unwrap())?;
            println!("Successfully created repository at {}", path);
        }
        ("init", None) => create_repository(current_dir().unwrap())?,
        _ => (),
    }
    Ok(())
}
