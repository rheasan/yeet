set -xe
mkdir .tests
cargo test
rm -r .tests