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
    $binary = Join-Path $PWD 'dist\rust-3d-print-studio.exe'
    Copy-Item -LiteralPath 'target\x86_64-pc-windows-msvc\release\rust-3d-print-studio.exe' -Destination $binary
    if (!(Test-Path -LiteralPath $binary -PathType Leaf)) { throw 'Portable x64-EXE fehlt.' }
    if ($Installer) {
        Set-Location $PSScriptRoot
        & makensis /INPUTCHARSET UTF8 installer.nsi
        if ($LASTEXITCODE -ne 0) { throw 'Installationspaket konnte nicht erstellt werden.' }
    }
} finally { Pop-Location }
