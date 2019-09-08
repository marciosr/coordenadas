
echo "Lembrar do workaround da croscompilação rust uwando mingw: for lib in crt2.o dllcrt2.o libmsvcrt.a; do cp -v /usr/x86_64-w64-mingw32/lib/$lib $HOME/.rustup/toolchains/$CHANNEL-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/; done"
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/x86_64-w64-mingw32/lib/pkgconfig
export GTK_INSTALL_PATH=/usr/x86_64-w64-mingw32
cargo build --target=x86_64-pc-windows-gnu --release
