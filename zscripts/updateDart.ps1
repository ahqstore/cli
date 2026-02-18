$version = $env:CARGO_VERSION

# Update pubspec
$pubspec = Get-Content "./dart/pubspec.yaml" -Raw

$output = $pubspec -replace "version: ([^`"\s<]+)", "version: $version"

$pubspec > "./dart/pubspec.bak.yaml"
$output > "./dart/pubspec.yaml"

# Update the dart file
$dartbin = Get-Content "./dart/bin/ahqstore.dart" -Raw

$output = $dartbin -replace "const String version = `"([^`"\s<]+)`";", "const String version = `"$version`";"

$dartbin > "./dart/bin/ahqstore.bak.dart"
$output > "./dart/bin/ahqstore.dart"
