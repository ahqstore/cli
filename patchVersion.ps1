$newVersion = cargo metadata --no-deps --format-version 1 |
ConvertFrom-Json |
Select-Object -ExpandProperty packages |
Select-Object -First 1 -ExpandProperty version

$env:CARGO_VERSION = $newVersion

$ErrorActionPreference = "Stop"

./zscripts/updateGolang.ps1

pwsh "./zscripts/updateJs.ps1"
pwsh "./zscripts/updatePyPi.ps1"
pwsh "./zscripts/updateDart.ps1"
pwsh "./zscripts/updateNuget.ps1"