$version = $env:CARGO_VERSION

# Update pubspec
$pubspec = Get-Content "./dart/pubspec.yaml" -Raw

$output = $pubspec -replace "version: ([^`"\s<]+)", "version: $version"

$output > "./dart/pubspec.yaml"

# Update the dart file
$dartbin = Get-Content "./dart/bin/ahqstore.dart" -Raw

$output = $dartbin -replace "const String version = `"([^`"\s<]+)`";", "const String version = `"$version`";"

$output > "./dart/bin/ahqstore.dart"

# Add CHANGELOG.md
$changelog = Get-Content "./dart/CHANGELOG.md"

"## $version

Please refer to https://github.com/ahqstore/cli/releases/tag/$version for changelog

$changelog" > "./dart/CHANGELOG.md"