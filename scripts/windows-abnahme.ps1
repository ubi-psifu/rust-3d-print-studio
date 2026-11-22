# Führt Setup und Deinstallation ausschließlich auf einem frischen GitHub-Runner aus.
param([switch]$MitFenster)
$ErrorActionPreference = 'Stop'
if ($env:RUNNER_OS -ne 'Windows' -or $env:GITHUB_ACTIONS -ne 'true') {
    throw 'Für eine isolierte Windows-Testmaschine in GitHub Actions vorgesehen.'
}
$root = Split-Path $PSScriptRoot
$setup = Join-Path $root 'dist\Rust-3D-Print-Studio-Setup-x64.exe'
$install = Join-Path $env:LOCALAPPDATA 'Rust 3D Print Studio'
$appKey = 'HKCU:\Software\Classes\Applications\rust-3d-print-studio.exe'
$uninstallKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio'
if ((Test-Path $install) -or (Test-Path $appKey) -or (Test-Path $uninstallKey)) {
    throw 'Eine bestehende Installation verhindert die isolierte Prüfung.'
}
$version = (Select-String -Path "$root\Cargo.toml" -Pattern '^version = "([^"]+)"').Matches[0].Groups[1].Value
if ((Start-Process $setup -ArgumentList '/S' -Wait -PassThru).ExitCode -ne 0) { throw 'Setup fehlgeschlagen.' }
try {
    $exe = Join-Path $install 'rust-3d-print-studio.exe'
    if ((Get-FileHash $exe).Hash -ne (Get-FileHash "$root\dist\rust-3d-print-studio.exe").Hash) { throw 'Programmdatei weicht vom Archiv ab.' }
    if ((Get-ItemProperty $uninstallKey).DisplayVersion -ne $version) { throw 'Falsche installierte Version.' }
    $command = (Get-Item "$appKey\shell\open\command").GetValue('')
    if ($command -ne ('"{0}" "%1"' -f $exe)) { throw 'Fehlerhafter Öffnen-Befehl.' }
    if ((Get-Item "$appKey\SupportedTypes").GetValueNames() -notcontains '.r3dp') { throw 'Dateityp fehlt.' }
    $example = Join-Path $install 'examples\Bogenlasche.r3dp'
    if ((Get-FileHash $example).Hash -ne (Get-FileHash "$root\examples\modelle\Bogenlasche.r3dp").Hash) { throw 'Beispiel fehlt oder weicht ab.' }
    Write-Output "Installation, Version $version, Beispiel und Öffnen-Befehl geprüft."
    if ($MitFenster) {
        $copy = Join-Path $env:RUNNER_TEMP 'Prüfung mit Leerzeichen.r3dp'
        Copy-Item $example $copy
        $before = [datetime]::UtcNow.AddYears(-1)
        (Get-Item $copy).LastWriteTimeUtc = $before
        $process = Start-Process $exe -ArgumentList ('"{0}"' -f $copy) -PassThru
        try {
            $shell = New-Object -ComObject WScript.Shell
            $saved = $false
            for ($i = 0; $i -lt 30; $i++) {
                Start-Sleep -Seconds 1
                $process.Refresh()
                if ($process.HasExited) { throw "Prozess beendet: $($process.ExitCode)" }
                if ($process.MainWindowTitle -eq 'Rust 3D Print Studio' -and $shell.AppActivate($process.Id)) {
                    $shell.SendKeys('^s')
                    Start-Sleep -Milliseconds 500
                    if ((Get-Item $copy).LastWriteTimeUtc -gt $before.AddDays(1)) { $saved = $true; break }
                }
            }
            if (!$saved -or (Get-Content $copy -Raw | ConvertFrom-Json).objects[0].name -ne 'Bogenlasche') {
                throw 'Öffnen und Speichern über das Programmfenster nicht bestätigt.'
            }
            Write-Output 'Bogenlasche im installierten Studio geöffnet und mit Strg+S gespeichert.'
        } finally {
            if (!$process.HasExited) { $process.Kill(); $process.WaitForExit() }
        }
    }
} finally {
    $uninstaller = Join-Path $install 'Deinstallieren.exe'
    if ((Start-Process $uninstaller -ArgumentList '/S' -Wait -PassThru).ExitCode -ne 0) { throw 'Deinstallation fehlgeschlagen.' }
    for ($i = 0; $i -lt 15 -and (Test-Path $install); $i++) { Start-Sleep -Seconds 1 }
    if ((Test-Path $install) -or (Test-Path $appKey) -or (Test-Path $uninstallKey)) { throw 'Installationsreste gefunden.' }
    Write-Output 'Deinstallation geprüft.'
}
