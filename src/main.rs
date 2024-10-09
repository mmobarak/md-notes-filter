use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    println!("Input file argument provided: {}", args.input_file);
}
