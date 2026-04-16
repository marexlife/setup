PROJECT_NAME=project
MOD_NAME=mod

cargo build --release
rm -rf ${PROJECT_NAME}
./target/release/setup new ${PROJECT_NAME}
cd ${PROJECT_NAME}
./../target/release/setup mod ${MOD_NAME}
cd ..

echo "export PATH=\"$PATH:$(pwd)/target/release\"" >> ~/.bashrc
