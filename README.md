#### set up (MacOS)
1.
```shell
rustup target add x86_64-unknown-linux-musl
```
2.
```shell
sudo brew install filosottile/musl-cross/musl-cross
```
3.
```
echo $'[target.x86_64-unknown-linux-musl]\nlinker = "x86_64-linux-musl-gcc"' > .cargo/config
```
#### build
1.
```shell
cargo build --release --target x86_64-unknown-linux-musl
```
2.
```shell
cp ./target/x86_64-unknown-linux-musl/release/lambda_rust ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
```