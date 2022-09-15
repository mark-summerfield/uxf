SET CARGO_TARGET_DIR=target
SET RELEASE_DIR=%CARGO_TARGET_DIR%\release

cargo build --release -- %*
copy /Y target\release\uxf.exe .
rcedit uxf.exe --set-icon uxf.ico
copy /Y uxf.exe C:\bin

cargo build --release --bin uxfcmp
copy /Y target\release\uxfcmp.exe .
rcedit uxfcmp.exe --set-icon uxf.ico
copy /Y uxfcmp.exe C:\bin
