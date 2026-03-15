#!/bin/sh
if [ z"$1" == z"" ]; then
    echo "CALL $0 <dir>"
    exit 1
fi

cd $1

for p in */**.rs; do
    echo "FILE:" "$p"
    rust-api-helper "$p"
done
