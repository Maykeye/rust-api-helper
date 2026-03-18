#!/bin/sh
shopt -s globstar

if [ z"$1" == z"" ]; then
    echo "CALL $0 <dir>"
    exit 1
fi

cd $1

for p in **/*.rs; do
    if [[ $p =~ ^target/ ]]; then
        continue
    fi
    echo "FILE:" "$p"
    rust-api-helper "$p"
done
