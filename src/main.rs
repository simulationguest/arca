use crate::sqlar::extract_archive;
use crate::Cmd::Extract;
use clap::{Parser, Subcommand};
use sqlar::create_archive;
use std::path::{Path, PathBuf};

mod sqlar;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Create { path: PathBuf },
    Extract { from: PathBuf, to: PathBuf },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Cmd::Create { path } => {
            let mut out_path = path.as_os_str().to_os_string();
            out_path.push(".sqlite");
            create_archive(&path, Path::new(&out_path)).unwrap();
        }
        Extract { from, to } => {
            extract_archive(&from, &to).unwrap();
        }
    }
}
