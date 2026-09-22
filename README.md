alansare cs2 cheat

linux cs2 project written in rust.

requirements
linux
rust
cargo
install rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

build

```bash
git clone https://github.com/q2hp0/alansare-cheat.git
cd alansare-cheat
cargo build --release
```

run

```bash
sudo ./target/release/alansare-cs2-cheat
```

update

```bash
cd alansare-cheat
git pull
cargo build --release
