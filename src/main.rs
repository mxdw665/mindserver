mod cli;
mod download;
mod logger;
mod run;

use clap::Parser;

fn main() {
    logger::init();
    cli::Main::parse().run();
}
