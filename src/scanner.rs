use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ItemDisco {
    pub caminho: PathBuf,
    pub tamanho: u64,
    pub e_diretorio: bool,
    #[allow(dead_code)]
    pub profundidade: usize,
}

pub struct ResultadoVarredura {
    pub itens: Vec<ItemDisco>,
    pub tamanho_total: u64,
    pub total_arquivos: usize,
    pub total_diretorios: usize,
    pub erros: usize,
}

pub fn varrer_diretorio(
    caminho_raiz: &Path,
    profundidade_max: usize,
    somente_diretorios: bool,
) -> ResultadoVarredura {
    let raiz = caminho_raiz;
    let mut tamanhos_dirs: HashMap<PathBuf, u64> = HashMap::new();
    let mut total_arquivos: usize = 0;
    let mut total_diretorios: usize = 0;
    let mut erros: usize = 0;

    for entrada in WalkDir::new(raiz).into_iter() {
        match entrada {
            Ok(e) => {
                if e.file_type().is_file() {
                    total_arquivos += 1;
                    let tamanho = e.metadata().map(|m| m.len()).unwrap_or(0);

                    let mut ancestral = e.path().parent();
                    while let Some(dir) = ancestral {
                        *tamanhos_dirs.entry(dir.to_path_buf()).or_insert(0) += tamanho;

                        if dir == raiz {
                            break;
                        }
                        ancestral = dir.parent();
                    }
                } else if e.file_type().is_dir() {
                    total_diretorios += 1;
                    tamanhos_dirs.entry(e.path().to_path_buf()).or_insert(0);
                }
            }
            Err(_) => {
                erros += 1;
            }
        }
    }

    let tamanho_total = tamanhos_dirs.get(raiz).copied().unwrap_or(0);
    let profundidade_raiz = raiz.components().count();

    let mut itens: Vec<ItemDisco> = tamanhos_dirs
        .into_iter()
        .filter_map(|(caminho, tamanho)| {
            let profundidade_abs = caminho.components().count();
            let profundidade_rel = profundidade_abs.saturating_sub(profundidade_raiz);

            if profundidade_rel > profundidade_max {
                return None;
            }

            if caminho == raiz && profundidade_rel == 0 {
                return None;
            }

            Some(ItemDisco {
                caminho,
                tamanho,
                e_diretorio: true,
                profundidade: profundidade_rel,
            })
        })
        .collect();

    if !somente_diretorios {
        for entrada in WalkDir::new(raiz)
            .max_depth(profundidade_max)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entrada.file_type().is_file() {
                let profundidade_abs = entrada.path().components().count();
                let profundidade_rel = profundidade_abs.saturating_sub(profundidade_raiz);

                if profundidade_rel <= profundidade_max {
                    let tamanho = entrada.metadata().map(|m| m.len()).unwrap_or(0);
                    itens.push(ItemDisco {
                        caminho: entrada.into_path(),
                        tamanho,
                        e_diretorio: false,
                        profundidade: profundidade_rel,
                    });
                }
            }
        }
    }

    ResultadoVarredura {
        itens,
        tamanho_total,
        total_arquivos,
        total_diretorios,
        erros,
    }
}
