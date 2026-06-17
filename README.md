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
- Open a folder picker on Windows when no path is passed.

## Usage

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

- `diretorio`: directory to scan. If omitted on Windows, a folder picker opens.
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

## Windows Installer

```powershell
powershell -ExecutionPolicy Bypass -File scripts/package-installer.ps1
```

The installer is generated at:

- `release/duscan-v0.1.0-x86_64-pc-windows-msvc-setup.exe`

It installs `duscan.exe` to `%LOCALAPPDATA%\Programs\duscan` and adds that
folder to the current user's PATH. The uninstaller removes the PATH entry.

## Release Assets

The GitHub release should include:

- `duscan-v0.1.0-x86_64-pc-windows-msvc.zip`
- `duscan-v0.1.0-x86_64-pc-windows-msvc.zip.sha256`
- `duscan-v0.1.0-x86_64-pc-windows-msvc-setup.exe`
- `duscan-v0.1.0-x86_64-pc-windows-msvc-setup.exe.sha256`

## Name

`duscan` is short for disk usage scan. It is lowercase, easy to type, and close
to familiar CLI naming such as `du`.
