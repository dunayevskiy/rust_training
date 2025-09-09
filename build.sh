cargo test

cargo build --release --target x86_64-unknown-linux-musl

docker build -t rust-calc .

docker run --rm rust-calc