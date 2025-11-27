#!/bin/bash -e

if [[ ! $1 ]]; then
    echo "Usage: $0 <part-name>"
    exit 1
fi

arg_dash="${1//_/-}"
arg_ident="${1//-/_}"

if [[ ! $arg_dash =~ ^[0-9]{4}-[0-9]{2}-[1-3]$ ]]; then
    echo "Invalid argument, expected YYYY-DD-P"
    exit 1
fi

input_file="input/${arg_dash}.txt"
part="part_${arg_ident}"
part_file="src/parts/${part}.rs"

if [[ -f "$input_file" || -f "$part_file" ]]; then
    echo "Part already exists!"
    exit 1
fi

cp src/current.rs "$part_file"
cp input/current.txt "$input_file"
sed -i -e "s|include_str!(\"../input/current.txt\")|include_str!(\"../../${input_file}\")|g" "$part_file"

sed -i -e "s|// <INSERTION_POINT>|pub mod ${part};\n    // <INSERTION_POINT>|" src/main.rs

echo -n "" > input/current.txt

if [[ "$1" == *3 ]]; then
    cat > src/current.rs <<EOF
use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/current.txt");
    // let input = "";

    let result = input
        ;

    result!(result);
}
EOF

    git add .
    git commit -m "Added Part ${arg_dash%-*}"
fi
