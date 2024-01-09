#!/bin/bash
# Meant to be ran under `libdonet-docs` branch for deployment with GH pages.
cargo doc --no-deps
rm -rf ../docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=libdonet\">" > ../target/doc/index.html
cp -r ../target/doc ../docs
touch ../docs/CNAME
echo "libdonet.rs" > ../docs
