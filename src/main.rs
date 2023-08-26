use clap::Parser;

/// Commandline application to list DICOM RTPlan files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path from which the DICOM RTPlans will be listed.
    #[arg(short, long)]
    path: String,
    /// Speed up the listing by only parsing files that start with RP*.
    #[arg(short, long)]
    fast: bool,
}

fn main() {
    let args = Args::parse();
    println!("path: {}", &args.path);
    println!("fast: {}", &args.fast);
}
