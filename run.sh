cargo build --release
rm -rf Project
./target/release/setup Project
cd Project
./../target/release/setup mod Mod
