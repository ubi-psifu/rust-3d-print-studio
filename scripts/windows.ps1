param([switch]$Installer)
$ErrorActionPreference = 'Stop'
Push-Location (Join-Path $PSScriptRoot '..')
try {
    function Invoke-Cargo {
        & cargo @args
        if ($LASTEXITCODE -ne 0) { throw "Cargo fehlgeschlagen: $args" }
    }
    Invoke-Cargo fmt --all --check
    Invoke-Cargo check --workspace --all-targets --locked
    Invoke-Cargo clippy --workspace --all-targets --locked '--' '-D' warnings
    Invoke-Cargo test --workspace --locked
    Invoke-Cargo build --workspace --release --locked --target x86_64-pc-windows-msvc
    New-Item -ItemType Directory -Force dist | Out-Null
    Copy-Item target/x86_64-pc-windows-msvc/release/rust-3d-print-studio.exe dist/
    if ($Installer) {
        & makensis scripts/installer.nsi
        if ($LASTEXITCODE -ne 0) { throw 'Installationspaket konnte nicht erstellt werden.' }
    }
} finally { Pop-Location }
