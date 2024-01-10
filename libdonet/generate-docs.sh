#!/bin/bash
# Run `git checkout libdonet-docs` beforehand!

cargo doc --no-deps --config build.rustdocflags=\"--default-theme=ayu\"
rm -rf ../docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=libdonet\">" > ../target/doc/index.html
cp -r ../target/doc ../docs
touch ../docs/CNAME
echo "libdonet.rs" > ../docs/CNAME
