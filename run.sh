cargo build --release
rm -rf Project
./target/release/setup new Project
cd Project
./../target/release/setup mod Mod
