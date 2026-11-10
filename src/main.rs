use clap::Parser;

/// Stream the audio of selected apps to stdout as PCM.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {}

fn main() {
    let _args = Args::parse();
}
