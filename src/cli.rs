use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "duscan")]
#[command(about = "Scan disk usage and list the largest paths")]
pub struct Cli {
    /// Directory to scan. Opens a folder picker when omitted on Windows.
    pub diretorio: Option<PathBuf>,

    /// Maximum scan depth.
    #[arg(short = 'p', long, default_value_t = 3)]
    pub profundidade: usize,

    /// Maximum number of results to show.
    #[arg(short = 'n', long, default_value_t = 20)]
    pub limite: usize,

    /// Sort by size, largest first.
    #[arg(short, long, default_value_t = true)]
    pub ordenar: bool,

    /// Show only directories, ignoring individual files.
    #[arg(short = 'd', long)]
    pub somente_diretorios: bool,

    /// Minimum size to show, in bytes.
    #[arg(short = 'm', long, default_value_t = 0)]
    pub minimo: u64,

    /// Show a visual proportion bar.
    #[arg(short = 'b', long, default_value_t = true)]
    pub barra: bool,
}
