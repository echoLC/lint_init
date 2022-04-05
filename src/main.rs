use ansi_term::Colour::Red;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name="lint_init", author="echoLC", version = "0.0.1", about = "init lint config for a project", long_about=None)]
struct Args {
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}", Red.paint(&args.name))
    }
}
