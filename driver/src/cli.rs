use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub init: bool,

    #[arg(short, long)]
    pub utrace: Option<PathBuf>,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long, value_delimiter = ',')]
    pub filter: Option<Vec<String>>,

    #[arg(short, long)]
    pub call_trace: bool,
}
