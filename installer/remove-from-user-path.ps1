param(
  [Parameter(Mandatory = $true)]
  [string]$Directory
)

$ErrorActionPreference = "Stop"

function Normalize-PathEntry {
  param([string]$PathEntry)

  try {
    return [System.IO.Path]::GetFullPath($PathEntry).TrimEnd("\")
  } catch {
    return $PathEntry.TrimEnd("\")
  }
}

function Broadcast-EnvironmentChange {
  $signature = @"
[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(
  IntPtr hWnd,
  uint Msg,
  UIntPtr wParam,
  string lParam,
  uint fuFlags,
  uint uTimeout,
  out UIntPtr lpdwResult);
"@

  $type = Add-Type -MemberDefinition $signature -Name NativeMethods -Namespace DuscanInstaller -PassThru
  $result = [UIntPtr]::Zero
  [void]$type::SendMessageTimeout([IntPtr]0xffff, 0x1A, [UIntPtr]::Zero, "Environment", 0x2, 5000, [ref]$result)
}

$target = Normalize-PathEntry $Directory
$current = [Environment]::GetEnvironmentVariable("Path", "User")

if ([string]::IsNullOrWhiteSpace($current)) {
  exit 0
}

$parts = @($current -split ";" | Where-Object {
  -not [string]::IsNullOrWhiteSpace($_) -and (Normalize-PathEntry $_) -ine $target
})

[Environment]::SetEnvironmentVariable("Path", ($parts -join ";"), "User")
Broadcast-EnvironmentChange
