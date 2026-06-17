use crate::scanner::ItemDisco;
use colored::*;
use std::path::Path;

pub fn formatar_tamanho(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn gerar_barra(proporcao: f64, largura: usize) -> String {
    let preenchido = (proporcao * largura as f64).round() as usize;
    let vazio = largura.saturating_sub(preenchido);
    format!("{}{}", "#".repeat(preenchido), ".".repeat(vazio))
}

pub fn exibir_resultados(
    itens: &[ItemDisco],
    tamanho_total: u64,
    raiz: &Path,
    mostrar_barra: bool,
) {
    let raiz_display = raiz.display().to_string();

    println!(
        "\n{} {}\n",
        "Uso de disco:".bold().cyan(),
        raiz_display.bold()
    );

    if itens.is_empty() {
        println!("{}", "Nenhum item encontrado.".yellow());
        return;
    }

    let largura_tamanho = 10;
    let largura_barra = 25;

    if mostrar_barra {
        println!(
            "{:>largura$}  {:>5}  {:<barra$}  {}",
            "Tamanho".bold(),
            "%".bold(),
            "Proporcao".bold(),
            "Caminho".bold(),
            largura = largura_tamanho,
            barra = largura_barra
        );
        println!("{}", "-".repeat(largura_tamanho + largura_barra + 40));
    } else {
        println!(
            "{:>largura$}  {:>5}  {}",
            "Tamanho".bold(),
            "%".bold(),
            "Caminho".bold(),
            largura = largura_tamanho
        );
        println!("{}", "-".repeat(largura_tamanho + 40));
    }

    for item in itens {
        let tam = formatar_tamanho(item.tamanho);
        let proporcao = if tamanho_total > 0 {
            item.tamanho as f64 / tamanho_total as f64
        } else {
            0.0
        };
        let percentual = proporcao * 100.0;
        let caminho_display = formatar_caminho(&item.caminho, raiz);

        let tam_colorido = if percentual > 50.0 {
            tam.red().bold().to_string()
        } else if percentual > 20.0 {
            tam.yellow().to_string()
        } else {
            tam.to_string()
        };

        let icone = if item.e_diretorio { "DIR" } else { "   " };

        if mostrar_barra {
            let barra = gerar_barra(proporcao, largura_barra);
            let barra_colorida = if percentual > 50.0 {
                barra.red().to_string()
            } else if percentual > 20.0 {
                barra.yellow().to_string()
            } else {
                barra.green().to_string()
            };

            println!(
                "{:>largura$}  {:>4.1}%  {:<barra$}  {} {}",
                tam_colorido,
                percentual,
                barra_colorida,
                icone.dimmed(),
                caminho_display,
                largura = largura_tamanho,
                barra = largura_barra
            );
        } else {
            println!(
                "{:>largura$}  {:>4.1}%  {} {}",
                tam_colorido,
                percentual,
                icone.dimmed(),
                caminho_display,
                largura = largura_tamanho
            );
        }
    }
}

fn formatar_caminho(caminho: &Path, raiz: &Path) -> String {
    caminho
        .strip_prefix(raiz)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| caminho.display().to_string())
}

pub fn exibir_resumo(
    tamanho_total: u64,
    total_arquivos: usize,
    total_diretorios: usize,
    erros: usize,
) {
    println!("\n{}", "Resumo".bold().cyan());
    println!(
        "  Tamanho total:  {}",
        formatar_tamanho(tamanho_total).bold()
    );
    println!("  Arquivos:       {}", total_arquivos);
    println!("  Diretorios:     {}", total_diretorios);
    if erros > 0 {
        println!(
            "  Erros:          {} (permissao negada ou inacessivel)",
            erros.to_string().red()
        );
    }
}
