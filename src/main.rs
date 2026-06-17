mod cli;
mod formatador;
mod scanner;

use clap::Parser;
use cli::Cli;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct ScanOptions {
    diretorio: Option<PathBuf>,
    profundidade: usize,
    limite: usize,
    ordenar: bool,
    somente_diretorios: bool,
    minimo: u64,
    barra: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            diretorio: None,
            profundidade: 3,
            limite: 20,
            ordenar: true,
            somente_diretorios: false,
            minimo: 0,
            barra: true,
        }
    }
}

impl From<Cli> for ScanOptions {
    fn from(cli: Cli) -> Self {
        Self {
            diretorio: cli.diretorio,
            profundidade: cli.profundidade,
            limite: cli.limite,
            ordenar: cli.ordenar && !cli.no_ordenar,
            somente_diretorios: cli.somente_diretorios,
            minimo: cli.minimo,
            barra: cli.barra && !cli.no_barra,
        }
    }
}

fn main() {
    if std::env::args_os().len() == 1 {
        run_shell();
        return;
    }

    let cli = Cli::parse();
    let mut options = ScanOptions::from(cli);

    if options.diretorio.is_none() {
        options.diretorio = escolher_diretorio();
    }

    if let Err(erro) = executar_scan(options) {
        eprintln!("{erro}");
        std::process::exit(1);
    }
}

fn run_shell() {
    println!("duscan interactive shell");
    println!("Type help for commands. Type exit to quit.");

    let stdin = io::stdin();

    loop {
        print!("DUSCAN> ");
        if io::stdout().flush().is_err() {
            return;
        }

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!();
                return;
            }
            Ok(_) => {}
            Err(erro) => {
                eprintln!("Could not read command: {erro}");
                continue;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match executar_comando(input) {
            ShellAction::Continue => {}
            ShellAction::Exit => return,
        }
    }
}

enum ShellAction {
    Continue,
    Exit,
}

fn executar_comando(input: &str) -> ShellAction {
    let tokens = match dividir_linha(input) {
        Ok(tokens) => tokens,
        Err(erro) => {
            eprintln!("{erro}");
            return ShellAction::Continue;
        }
    };

    if tokens.is_empty() {
        return ShellAction::Continue;
    }

    let comando = tokens[0].to_ascii_lowercase();

    match comando.as_str() {
        "exit" | "quit" | "q" => ShellAction::Exit,
        "help" | "?" => {
            imprimir_ajuda_shell();
            ShellAction::Continue
        }
        "clear" | "cls" => {
            limpar_tela();
            ShellAction::Continue
        }
        "open" | "dialog" | "pick" => {
            let mut options = match parse_scan_args(&tokens[1..]) {
                Ok(options) => options,
                Err(erro) => {
                    eprintln!("{erro}");
                    return ShellAction::Continue;
                }
            };
            options.diretorio = escolher_diretorio();
            if let Err(erro) = executar_scan(options) {
                eprintln!("{erro}");
            }
            ShellAction::Continue
        }
        "scan" => {
            let mut options = match parse_scan_args(&tokens[1..]) {
                Ok(options) => options,
                Err(erro) => {
                    eprintln!("{erro}");
                    return ShellAction::Continue;
                }
            };
            if options.diretorio.is_none() {
                options.diretorio = escolher_diretorio();
            }
            if let Err(erro) = executar_scan(options) {
                eprintln!("{erro}");
            }
            ShellAction::Continue
        }
        _ => {
            let options = match parse_scan_args(&tokens) {
                Ok(options) => options,
                Err(erro) => {
                    eprintln!("{erro}");
                    return ShellAction::Continue;
                }
            };
            if let Err(erro) = executar_scan(options) {
                eprintln!("{erro}");
            }
            ShellAction::Continue
        }
    }
}

fn imprimir_ajuda_shell() {
    println!();
    println!("Commands:");
    println!("  scan <path> [options]       Scan a path");
    println!("  <path> [options]            Scan a path directly");
    println!("  open [options]              Open a folder picker, then scan");
    println!("  dialog [options]            Same as open");
    println!("  clear                       Clear the screen");
    println!("  exit                        Quit");
    println!();
    println!("Options:");
    println!("  -p, --profundidade <n>      Maximum scan depth");
    println!("  -n, --limite <n>            Maximum rows to show");
    println!("  -d, --somente-diretorios    Show directories only");
    println!("  -m, --minimo <bytes>        Minimum size in bytes");
    println!("  --no-barra                  Hide the visual bar");
    println!("  --no-ordenar                Keep traversal order");
    println!();
    println!("Examples:");
    println!("  DUSCAN> scan C:\\Users\\black\\Downloads -p 2 -n 15");
    println!("  DUSCAN> \"C:\\Program Files\" --somente-diretorios");
    println!("  DUSCAN> open -p 3");
    println!();
}

fn parse_scan_args(args: &[String]) -> Result<ScanOptions, String> {
    let mut options = ScanOptions::default();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--profundidade" => {
                i += 1;
                options.profundidade = parse_required(args, i, "profundidade")?;
            }
            "-n" | "--limite" => {
                i += 1;
                options.limite = parse_required(args, i, "limite")?;
            }
            "-m" | "--minimo" => {
                i += 1;
                options.minimo = parse_required(args, i, "minimo")?;
            }
            "-d" | "--somente-diretorios" => {
                options.somente_diretorios = true;
            }
            "-b" | "--barra" => {
                options.barra = true;
            }
            "--no-barra" => {
                options.barra = false;
            }
            "-o" | "--ordenar" => {
                options.ordenar = true;
            }
            "--no-ordenar" => {
                options.ordenar = false;
            }
            token if token.starts_with('-') => {
                return Err(format!("Unknown option: {token}"));
            }
            path => {
                if options.diretorio.is_some() {
                    return Err(format!("Unexpected argument: {path}"));
                }
                options.diretorio = Some(PathBuf::from(path));
            }
        }

        i += 1;
    }

    Ok(options)
}

fn parse_required<T>(args: &[String], index: usize, nome: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    let valor = args
        .get(index)
        .ok_or_else(|| format!("Missing value for {nome}"))?;

    valor
        .parse::<T>()
        .map_err(|_| format!("Invalid value for {nome}: {valor}"))
}

fn dividir_linha(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut atual = String::new();
    let mut quote: Option<char> = None;

    for c in input.chars() {
        match quote {
            Some(q) if c == q => quote = None,
            Some(_) => atual.push(c),
            None if c == '"' || c == '\'' => quote = Some(c),
            None if c.is_whitespace() => {
                if !atual.is_empty() {
                    tokens.push(std::mem::take(&mut atual));
                }
            }
            None => atual.push(c),
        }
    }

    if quote.is_some() {
        return Err("Unclosed quote in command".to_string());
    }

    if !atual.is_empty() {
        tokens.push(atual);
    }

    Ok(tokens)
}

fn escolher_diretorio() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Selecione uma pasta para analisar")
        .pick_folder()
}

fn executar_scan(options: ScanOptions) -> Result<(), String> {
    let caminho = options
        .diretorio
        .ok_or_else(|| "No directory selected.".to_string())?;

    if !caminho.exists() {
        return Err(format!("Error: '{}' does not exist.", caminho.display()));
    }

    if !caminho.is_dir() {
        return Err(format!(
            "Error: '{}' is not a directory.",
            caminho.display()
        ));
    }

    let resultado =
        scanner::varrer_diretorio(&caminho, options.profundidade, options.somente_diretorios);

    let mut itens = resultado.itens;

    if options.minimo > 0 {
        itens.retain(|item| item.tamanho >= options.minimo);
    }

    if options.ordenar {
        itens.sort_by(|a, b| b.tamanho.cmp(&a.tamanho));
    }

    itens.truncate(options.limite);

    formatador::exibir_resultados(&itens, resultado.tamanho_total, &caminho, options.barra);

    formatador::exibir_resumo(
        resultado.tamanho_total,
        resultado.total_arquivos,
        resultado.total_diretorios,
        resultado.erros,
    );

    Ok(())
}

fn limpar_tela() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_linha_com_path_entre_aspas() {
        let tokens = dividir_linha(r#"scan "C:\Program Files" -p 2"#).unwrap();
        assert_eq!(tokens, vec!["scan", r#"C:\Program Files"#, "-p", "2"]);
    }

    #[test]
    fn parse_scan_args_com_opcoes() {
        let args = vec![
            "C:\\tmp".to_string(),
            "-p".to_string(),
            "2".to_string(),
            "-n".to_string(),
            "5".to_string(),
            "--no-barra".to_string(),
        ];

        let options = parse_scan_args(&args).unwrap();
        assert_eq!(options.diretorio, Some(PathBuf::from("C:\\tmp")));
        assert_eq!(options.profundidade, 2);
        assert_eq!(options.limite, 5);
        assert!(!options.barra);
    }
}
