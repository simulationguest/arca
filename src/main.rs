use std::path::PathBuf;
use clap::{Parser, Subcommand};
use sqlar::create_archive;
mod sqlar;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Create{path: String}
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Cmd::Create{path} => {
            create_archive(&path, &format!("{path}.sqlar")).unwrap();
        }
    }

}
