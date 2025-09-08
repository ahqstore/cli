mkdir dist

Copy-Item "./target/$env:TARGET/release/ahqstore.exe" "./dist/ahqstore.exe" -ErrorAction Continue
Copy-Item "./target/$env:TARGET/release/ahqstore.exe" "./dist/ahqstore_cli_rs.exe" -ErrorAction Continue

Copy-Item "./target/$env:TARGET/release/ahqstore" "./dist/ahqstore" -ErrorAction Continue
Copy-Item "./target/$env:TARGET/release/ahqstore" "./dist/ahqstore_cli_rs" -ErrorAction Continue

compress-archive ./dist/* ./ahqstore_cli_rs-$env:TARGET

Copy-Item "./target/$env:TARGET/release/ahqstore_cli_rs.dll" "./ahqstore_cli_rs-$env:TARGET.dll" -ErrorAction Continue
Copy-Item "./target/$env:TARGET/release/libahqstore_cli_rs.so" "./libahqstore_cli_rs-$env:TARGET.so" -ErrorAction Continue
Copy-Item "./target/$env:TARGET/release/libahqstore_cli_rs.dylib" "./libahqstore_cli_rs-$env:TARGET.dylib" -ErrorAction Continue
