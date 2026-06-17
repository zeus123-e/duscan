use embed_manifest::{embed_manifest, manifest::DpiAwareness, new_manifest};

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(new_manifest("duscan").dpi_awareness(DpiAwareness::PerMonitorV2))
            .expect("Falha ao incorporar o manifesto do Windows");
    }

    println!("cargo:rerun-if-changed=src/build.rs");
}
