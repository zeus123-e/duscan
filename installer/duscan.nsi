!ifndef VERSION
  !define VERSION "0.1.0"
!endif

!ifndef OUTFILE
  !define OUTFILE "..\release\duscan-v${VERSION}-x86_64-pc-windows-msvc-setup.exe"
!endif

!ifndef EXE_FILE
  !define EXE_FILE "..\target\release\duscan.exe"
!endif

!ifndef README_FILE
  !define README_FILE "..\README.md"
!endif

!ifndef ADD_PATH_SCRIPT
  !define ADD_PATH_SCRIPT "add-to-user-path.ps1"
!endif

!ifndef REMOVE_PATH_SCRIPT
  !define REMOVE_PATH_SCRIPT "remove-from-user-path.ps1"
!endif

Unicode True
Name "duscan"
OutFile "${OUTFILE}"
InstallDir "$LOCALAPPDATA\Programs\duscan"
RequestExecutionLevel user
SetCompressor /SOLID lzma
ShowInstDetails show
ShowUninstDetails show

VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "duscan"
VIAddVersionKey "CompanyName" "duscan"
VIAddVersionKey "FileDescription" "duscan installer"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"

Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles

Section "Install"
  SetOutPath "$INSTDIR"
  File /oname=duscan.exe "${EXE_FILE}"
  File /oname=README.md "${README_FILE}"
  File /oname=add-to-user-path.ps1 "${ADD_PATH_SCRIPT}"
  File /oname=remove-from-user-path.ps1 "${REMOVE_PATH_SCRIPT}"

  WriteUninstaller "$INSTDIR\uninstall.exe"

  nsExec::ExecToLog 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\add-to-user-path.ps1" -Directory "$INSTDIR"'

  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "DisplayName" "duscan"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "Publisher" "duscan"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "DisplayIcon" "$INSTDIR\duscan.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "NoModify" 1
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan" "NoRepair" 1
SectionEnd

Section "Uninstall"
  nsExec::ExecToLog 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\remove-from-user-path.ps1" -Directory "$INSTDIR"'

  Delete "$INSTDIR\duscan.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\add-to-user-path.ps1"
  Delete "$INSTDIR\remove-from-user-path.ps1"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\duscan"
SectionEnd
