# Prüft die portable EXE mit kurzem Strg+S auf dem isolierten Windows-Runner.
$ErrorActionPreference = 'Stop'
if ($env:RUNNER_OS -ne 'Windows' -or $env:GITHUB_ACTIONS -ne 'true') {
    throw 'Eine isolierte Windows-Testmaschine in GitHub Actions ist erforderlich.'
}
$root = Split-Path $PSScriptRoot
$bin = 'C:\msys64\mingw64\bin'
$env:PATH = "$bin;$env:PATH"
$env:GALLIUM_DRIVER = 'llvmpipe'
$env:RUST_BACKTRACE = '1'
$portable = Resolve-Path "$root/dist/rust-3d-print-studio.exe"
Copy-Item $portable "$bin/rust-3d-print-studio.exe"
$copy = Join-Path $env:RUNNER_TEMP 'Prüfung mit Leerzeichen.r3dp'
Copy-Item "$root/examples/modelle/Bogenlasche.r3dp" $copy
(Get-Item $copy).LastWriteTimeUtc = [datetime]::UtcNow.AddYears(-1)
$errfile = Join-Path $env:RUNNER_TEMP 'studio-stderr.txt'
$process = Start-Process "$bin/rust-3d-print-studio.exe" -ArgumentList ('"{0}"' -f $copy) -RedirectStandardError $errfile -PassThru
try {
  $shell = New-Object -ComObject WScript.Shell
  for ($i=0; $i -lt 40; $i++) {
    Start-Sleep -Seconds 1
    $process.Refresh()
    if ($process.HasExited) { Get-Content $errfile; throw "Programmende $($process.ExitCode)" }
    if ($process.MainWindowTitle -eq 'Rust 3D Print Studio' -and $shell.AppActivate($process.Id)) {
      $shell.SendKeys('^s')
      Start-Sleep -Milliseconds 500
      if ((Get-Item $copy).LastWriteTimeUtc -gt [datetime]::UtcNow.AddMinutes(-5)) {
        if ((Get-Content $copy -Raw | ConvertFrom-Json).objects[0].name -ne 'Bogenlasche') { throw 'Falsches Projekt gespeichert' }
        Write-Output 'Bogenlasche mit Strg+S im Windows-Fenster gespeichert.'
        return
      }
    }
  }
  Get-Content $errfile
  throw 'Speichervorgang nicht bestätigt'
} finally { if (!$process.HasExited) { $process.Kill(); $process.WaitForExit() } }
