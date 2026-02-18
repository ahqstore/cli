$version = $env:CARGO_VERSION
$pypi = Get-Content "./dotnet/dotnet.csproj" -Raw

$output = $pypi -replace "<Version>([^`"\s<]+)</Version>", "<Version>$version</Version>"

$pypi > "./dotnet/dotnet.bak.csproj"
$output > "./dotnet/dotnet.csproj"