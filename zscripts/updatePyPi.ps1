$version = $env:CARGO_VERSION
$pypi = Get-Content "./python/pyproject.toml" -Raw

$output = $pypi -replace "version = `"([^`"\s<]+)`"", "version = `"$version`""

$pypi > "./python/pyproject.bak.toml"
$output > "./python/pyproject.toml"