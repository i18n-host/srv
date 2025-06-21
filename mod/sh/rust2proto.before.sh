DIR_HPC=$(dirname $(dirname $DIR))/hpc/coffee/rust2proto

mkdir -p gen/js
cd gen
find . -mindepth 1 ! -name 'package.json' -delete
cargo new --lib pb
cd ..

cargo build -p mod
