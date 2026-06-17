# duscan

`duscan` is a small Windows-friendly CLI for scanning disk usage and listing the
largest folders or files under a path.

It is meant to feel like a practical terminal tool: short command name, plain
output, sortable results and a release binary you can drop into your PATH.

## Features

- Scan a directory recursively.
- Limit scan depth.
- Show the largest paths first.
- Filter by minimum size in bytes.
- Show directories only when you do not want individual files.
- Print a visual proportion bar.
- Open into an interactive `DUSCAN>` shell when no arguments are passed.
- Open a folder picker from the shell with `open` or `dialog`.

## Usage

Run without arguments to open the interactive shell:

```powershell
duscan
```

Inside the shell:

```text
DUSCAN> scan C:\Users\black\Downloads -p 2 -n 15
DUSCAN> "C:\Program Files" --somente-diretorios
DUSCAN> open -p 3
DUSCAN> help
DUSCAN> exit
```

You can still run a one-shot scan directly:

```powershell
duscan C:\Users\black\Downloads
duscan . --profundidade 2 --limite 15
duscan C:\Projects --somente-diretorios --minimo 104857600
```

Short flags:

```powershell
duscan . -p 3 -n 20 -d -m 104857600
```

## Options

- `diretorio`: directory to scan. If omitted with other options, a folder picker opens.
- `-p, --profundidade`: maximum scan depth. Default: `3`.
- `-n, --limite`: maximum number of rows. Default: `20`.
- `-d, --somente-diretorios`: show directories only.
- `-m, --minimo`: minimum size in bytes.
- `-b, --barra`: show the visual proportion bar. Default: `true`.

## Build

```powershell
cargo build --release
```

The executable is generated at:

- `target/release/duscan.exe`

## Windows MSI Installer

```powershell
powershell -ExecutionPolicy Bypass -File scripts/package-msi.ps1
```

The installer is generated at:

- `release/duscan-v0.1.0-x86_64-pc-windows-msvc.msi`

By default, the MSI installs `duscan.exe` for all users in Program Files and
adds the install folder to the system PATH, so Windows may ask for administrator
approval. During setup, you can deselect the `Add duscan to PATH (recommended)`
feature if you do not want PATH changes. Uninstalling the MSI removes the PATH
entry when that feature was installed.

## Release Assets

The GitHub release should include:

- `duscan-v0.1.0-x86_64-pc-windows-msvc.zip`
- `duscan-v0.1.0-x86_64-pc-windows-msvc.zip.sha256`
- `duscan-v0.1.0-x86_64-pc-windows-msvc.msi`
- `duscan-v0.1.0-x86_64-pc-windows-msvc.msi.sha256`

## Name

`duscan` is short for disk usage scan. It is lowercase, easy to type, and close
to familiar CLI naming such as `du`.
