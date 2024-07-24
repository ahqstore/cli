mkdir dist
cp ./target/release/ahqstore.exe ./dist/ahqstore.exe
cp ./target/release/ahqstore.exe ./dist/ahqstore_cli_rs.exe
compress-archive ./dist/* ./ahqstore_cli_rs-$env:TARGET