Unicode true
Target x86-unicode
RequestExecutionLevel user
!searchparse /file "..\Cargo.toml" 'version = "' STUDIO_VERSION '"'
Name "Rust 3D Print Studio ${STUDIO_VERSION}"
OutFile "..\dist\Rust-3D-Print-Studio-Setup-x64.exe"
InstallDir "$LOCALAPPDATA\Rust 3D Print Studio"
ShowInstDetails show
!include "MUI2.nsh"
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "German"

!include "x64.nsh"
Function .onInit
  ${IfNot} ${RunningX64}
    MessageBox MB_OK "Rust 3D Print Studio benötigt 64-Bit-Windows."
    Abort
  ${EndIf}
FunctionEnd

Section "Programm"
  SetRegView 64
  SetOutPath "$INSTDIR"
  File "..\dist\rust-3d-print-studio.exe"
  File "..\USER_MANUAL.md"
  File "..\LICENSE"
  File "..\THIRD_PARTY_NOTICES.txt"
  SetOutPath "$INSTDIR\profiles"
  File "..\profiles\p1s.json"
  SetOutPath "$INSTDIR\examples"
  File "..\examples\modelle\Wandhaken.r3dp"
  File "..\examples\modelle\Bemasste-Platte.r3dp"
  File "..\examples\modelle\Bemasste-Lasche.r3dp"
  File "..\examples\modelle\Winkellasche.r3dp"
  File "..\examples\modelle\Keilschale.r3dp"
  File "..\examples\modelle\Kantenquader.r3dp"
  File "..\examples\modelle\Bogenlasche.r3dp"
  File "..\examples\modelle\Seitlich-offene-Schale.r3dp"
  SetOutPath "$INSTDIR"
  WriteUninstaller "$INSTDIR\Deinstallieren.exe"
  CreateDirectory "$SMPROGRAMS\Rust 3D Print Studio"
  CreateShortcut "$SMPROGRAMS\Rust 3D Print Studio\Studio.lnk" "$INSTDIR\rust-3d-print-studio.exe"
  CreateShortcut "$SMPROGRAMS\Rust 3D Print Studio\Beispiel.lnk" "$INSTDIR\rust-3d-print-studio.exe" '"$INSTDIR\examples\Wandhaken.r3dp"'
  WriteRegStr HKCU "Software\Classes\Applications\rust-3d-print-studio.exe" "FriendlyAppName" "Rust 3D Print Studio"
  WriteRegStr HKCU "Software\Classes\Applications\rust-3d-print-studio.exe\SupportedTypes" ".r3dp" ""
  WriteRegStr HKCU "Software\Classes\Applications\rust-3d-print-studio.exe\shell\open\command" "" '"$INSTDIR\rust-3d-print-studio.exe" "%1"'
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio" "DisplayName" "Rust 3D Print Studio"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio" "DisplayVersion" "${STUDIO_VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio" "UninstallString" '"$INSTDIR\Deinstallieren.exe"'
SectionEnd

Section "Uninstall"
  SetRegView 64
  Delete "$SMPROGRAMS\Rust 3D Print Studio\Studio.lnk"
  Delete "$SMPROGRAMS\Rust 3D Print Studio\Beispiel.lnk"
  RMDir "$SMPROGRAMS\Rust 3D Print Studio"
  Delete "$INSTDIR\rust-3d-print-studio.exe"
  Delete "$INSTDIR\USER_MANUAL.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\THIRD_PARTY_NOTICES.txt"
  Delete "$INSTDIR\profiles\p1s.json"
  Delete "$INSTDIR\examples\Wandhaken.r3dp"
  RMDir "$INSTDIR\profiles"
  Delete "$INSTDIR\examples\Bemasste-Platte.r3dp"
  Delete "$INSTDIR\examples\Bemasste-Lasche.r3dp"
  Delete "$INSTDIR\examples\Winkellasche.r3dp"
  Delete "$INSTDIR\examples\Keilschale.r3dp"
  Delete "$INSTDIR\examples\Kantenquader.r3dp"
  Delete "$INSTDIR\examples\Bogenlasche.r3dp"
  Delete "$INSTDIR\examples\Seitlich-offene-Schale.r3dp"
  RMDir "$INSTDIR\examples"
  Delete "$INSTDIR\Deinstallieren.exe"
  RMDir "$INSTDIR"
  DeleteRegKey HKCU "Software\Classes\Applications\rust-3d-print-studio.exe"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio"
SectionEnd
