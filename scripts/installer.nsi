Unicode true
Target amd64-unicode
RequestExecutionLevel user
Name "Rust 3D Print Studio 0.6.0"
OutFile "../dist/Rust-3D-Print-Studio-Setup-x64.exe"
InstallDir "$LOCALAPPDATA\Rust 3D Print Studio"
ShowInstDetails show
!include "MUI2.nsh"
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "German"

Section "Programm"
  SetOutPath "$INSTDIR"
  File "../dist/rust-3d-print-studio.exe"
  File "../USER_MANUAL.md"
  File "../LICENSE"
  File "../THIRD_PARTY_NOTICES.txt"
  SetOutPath "$INSTDIR\profiles"
  File "../profiles/p1s.json"
  SetOutPath "$INSTDIR\examples"
  File "../examples/modelle/Wandhaken.r3dp"
  SetOutPath "$INSTDIR"
  WriteUninstaller "$INSTDIR\Deinstallieren.exe"
  CreateDirectory "$SMPROGRAMS\Rust 3D Print Studio"
  CreateShortcut "$SMPROGRAMS\Rust 3D Print Studio\Studio.lnk" "$INSTDIR\rust-3d-print-studio.exe"
  CreateShortcut "$SMPROGRAMS\Rust 3D Print Studio\Beispiel.lnk" "$INSTDIR\rust-3d-print-studio.exe" '"$INSTDIR\examples\Wandhaken.r3dp"'
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio" "DisplayName" "Rust 3D Print Studio"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio" "DisplayVersion" "0.6.0"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio" "UninstallString" '"$INSTDIR\Deinstallieren.exe"'
SectionEnd

Section "Uninstall"
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
  RMDir "$INSTDIR\examples"
  Delete "$INSTDIR\Deinstallieren.exe"
  RMDir "$INSTDIR"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Rust3DPrintStudio"
SectionEnd
