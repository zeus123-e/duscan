mod cli;
mod formatador;
mod scanner;

use clap::Parser;
use cli::Cli;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    let usando_dialogo = cli.diretorio.is_none();
    let modo_gui = usando_dialogo && ocultar_console_se_for_janela_avulsa();

    let caminho = match cli.diretorio {
        Some(caminho) => caminho,
        None => match rfd::FileDialog::new()
            .set_title("Selecione uma pasta para analisar")
            .pick_folder()
        {
            Some(caminho) => caminho,
            None => {
                if modo_gui {
                    mostrar_mensagem("duscan", "Nenhuma pasta foi selecionada.", rfd::MessageLevel::Info);
                } else {
                    println!("Nenhuma pasta foi selecionada.");
                }
                return;
            }
        },
    };

    if !caminho.exists() {
        finalizar_com_erro(
            modo_gui,
            format!("Erro: '{}' nao existe.", caminho.display()),
        );
        std::process::exit(1);
    }

    if !caminho.is_dir() {
        finalizar_com_erro(
            modo_gui,
            format!("Erro: '{}' nao e um diretorio.", caminho.display()),
        );
        std::process::exit(1);
    }

    let resultado = scanner::varrer_diretorio(&caminho, cli.profundidade, cli.somente_diretorios);

    let mut itens = resultado.itens;

    if cli.minimo > 0 {
        itens.retain(|item| item.tamanho >= cli.minimo);
    }

    if cli.ordenar {
        itens.sort_by(|a, b| b.tamanho.cmp(&a.tamanho));
    }

    itens.truncate(cli.limite);

    if modo_gui {
        let relatorio = formatador::gerar_relatorio(
            &itens,
            resultado.tamanho_total,
            &caminho,
            cli.barra,
            resultado.total_arquivos,
            resultado.total_diretorios,
            resultado.erros,
        );

        match abrir_relatorio(&relatorio) {
            Ok(caminho_relatorio) => mostrar_mensagem(
                "duscan",
                &format!(
                    "Analise concluida.\nRelatorio salvo em:\n{}",
                    caminho_relatorio.display()
                ),
                rfd::MessageLevel::Info,
            ),
            Err(erro) => mostrar_mensagem("duscan", &erro, rfd::MessageLevel::Error),
        }
    } else {
        formatador::exibir_resultados(&itens, resultado.tamanho_total, &caminho, cli.barra);

        formatador::exibir_resumo(
            resultado.tamanho_total,
            resultado.total_arquivos,
            resultado.total_diretorios,
            resultado.erros,
        );
    }
}

fn finalizar_com_erro(modo_gui: bool, mensagem: String) {
    if modo_gui {
        mostrar_mensagem("duscan", &mensagem, rfd::MessageLevel::Error);
    } else {
        eprintln!("{mensagem}");
    }
}

fn mostrar_mensagem(titulo: &str, mensagem: &str, nivel: rfd::MessageLevel) {
    rfd::MessageDialog::new()
        .set_title(titulo)
        .set_description(mensagem)
        .set_level(nivel)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

fn abrir_relatorio(relatorio: &str) -> Result<PathBuf, String> {
    let caminho = std::env::temp_dir().join(format!("duscan-report-{}.txt", std::process::id()));
    std::fs::write(&caminho, relatorio)
        .map_err(|erro| format!("Nao foi possivel salvar o relatorio: {erro}"))?;

    #[cfg(windows)]
    {
        std::process::Command::new("notepad.exe")
            .arg(&caminho)
            .spawn()
            .map_err(|erro| format!("Nao foi possivel abrir o relatorio: {erro}"))?;
    }

    Ok(caminho)
}

#[cfg(windows)]
fn ocultar_console_se_for_janela_avulsa() -> bool {
    use windows_sys::Win32::System::Console::{
        FreeConsole, GetConsoleProcessList, GetConsoleWindow,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};

    let mut processos = [0_u32; 2];

    unsafe {
        let quantidade = GetConsoleProcessList(processos.as_mut_ptr(), processos.len() as u32);

        if quantidade <= 1 {
            let janela = GetConsoleWindow();
            if !janela.is_null() {
                ShowWindow(janela, SW_HIDE);
            }
            FreeConsole();
            true
        } else {
            false
        }
    }
}

#[cfg(not(windows))]
fn ocultar_console_se_for_janela_avulsa() -> bool {
    false
}
