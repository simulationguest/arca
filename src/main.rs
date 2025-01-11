use clap::Parser;
use sqlar::create_archive;
mod sqlar;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(index = 1)]
    dir: String,

    #[clap(index = 2)]
    out: String,
}

fn main() {
    let args = Args::parse();

    create_archive(&args.dir, &args.out).unwrap();
}
