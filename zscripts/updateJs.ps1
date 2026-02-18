$version = $env:CARGO_VERSION
$npmjs = Get-Content "./package.json" -Raw

$output = $npmjs -replace "`"version`": `"([^`"\s<]+)`"", "`"version`": `"$version`""

$npmjs > "package.bak.json"
$output > "package.json"

# JSR
$jsr = Get-Content "./jsr.json" -Raw

$output = $jsr -replace "`"version`": `"([^`"\s<]+)`"", "`"version`": `"$version`""

$jsr > "jsr.bak.json"
$output > "jsr.json"