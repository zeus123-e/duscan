use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "duscan")]
#[command(about = "Scan disk usage and list the largest paths")]
pub struct Cli {
    /// Directory to scan. If omitted with other options, opens a folder picker.
    pub diretorio: Option<PathBuf>,

    /// Maximum scan depth.
    #[arg(short = 'p', long, default_value_t = 3)]
    pub profundidade: usize,

    /// Maximum number of results to show.
    #[arg(short = 'n', long, default_value_t = 20)]
    pub limite: usize,

    /// Sort by size, largest first.
    #[arg(short, long, action = ArgAction::SetTrue, default_value_t = true)]
    pub ordenar: bool,

    /// Keep traversal order instead of sorting by size.
    #[arg(long = "no-ordenar", action = ArgAction::SetTrue)]
    pub no_ordenar: bool,

    /// Show only directories, ignoring individual files.
    #[arg(short = 'd', long)]
    pub somente_diretorios: bool,

    /// Minimum size to show, in bytes.
    #[arg(short = 'm', long, default_value_t = 0)]
    pub minimo: u64,

    /// Show a visual proportion bar.
    #[arg(short = 'b', long, action = ArgAction::SetTrue, default_value_t = true)]
    pub barra: bool,

    /// Hide the visual proportion bar.
    #[arg(long = "no-barra", action = ArgAction::SetTrue)]
    pub no_barra: bool,
}
