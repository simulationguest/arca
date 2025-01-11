use crate::sqlar::{extract_archive, Error};
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

    #[clap(flatten)]
    opts: GlobalOpts
}

#[derive(Debug, Parser)]
struct GlobalOpts {

    #[clap(short, long, default_value = "false")]
    verbose: bool,

}

#[derive(Debug, Subcommand)]
enum Cmd {
    Create { path: PathBuf },
    Extract { from: PathBuf, to: PathBuf },
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.cmd {
        Cmd::Create { path } => {
            let mut out_path = path.as_os_str().to_os_string();
            out_path.push(".sqlite");
            create_archive(&path, Path::new(&out_path), args.opts)
        }
        Extract { from, to } => {
            extract_archive(&from, &to, args.opts)
        }
    }
}
