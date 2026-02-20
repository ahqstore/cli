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

(Get-Date).ToString('yyyy-MM-dd HH:mm:ss') > ".build"

# README Patch
$readme = Get-Content "./README.md" -Raw

$out = $readme -replace "https://jsr.io/@ahqstore/cli/([^/\s<]+)/js/cli.js", "https://jsr.io/@ahqstore/cli/$newVersion/js/cli.js"
$out > "./README.md"