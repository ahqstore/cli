# const version = "0.15.0"

$version = $env:CARGO_VERSION
$golang = Get-Content "./go/ahqstore.go" -Raw

$output = $golang -replace "const version = `"([^`"\s<]+)`"", "const version = `"$version`""

if ($golang -ne $output) {
  $err = $PSStyle.Foreground.Red;

  Write-Error -Message "$err`ERROR$($PSStyle.Reset): Golang output is inconsistent";
  throw "Golang output is inconsistent"

  exit 1;
}