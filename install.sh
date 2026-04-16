cargo build --release
rm -rf Project
./target/release/setup new project
cd Project
./../target/release/setup mod mod
cd ..

echo "export PATH=\"$PATH:$(pwd)/target/release\"" >> ~/.bashrc
