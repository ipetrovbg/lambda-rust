#### set up (MacOS)
1. Add x86_64 target architecture for Rust
```shell
rustup target add x86_64-unknown-linux-musl
```
2. Install cross system **C** linker 
```shell
sudo brew install filosottile/musl-cross/musl-cross
```
3. Create .cargo/config and targeting x86_64 architecture for linux 
```
echo $'[target.x86_64-unknown-linux-musl]\nlinker = "x86_64-linux-musl-gcc"' > .cargo/config
```
#### build
1. Build the project
```shell
cargo build --release --target x86_64-unknown-linux-musl
```
2. Packaging binaries and zipping them into lambda.zip file
```shell
cp ./target/x86_64-unknown-linux-musl/release/lambda_rust ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
```