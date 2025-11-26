#!/bin/bash -e

if [[ ! $1 ]]; then
    echo "Usage: $0 <part-name>"
    exit 1
fi

input_file="input/${1//_/-}.txt"
part="part_${1//-/_}"
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
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/current.txt");
    // let input = "";

    let result = input
        ;

    result!(result);
}
EOF

    commit_name="${1//_/-}"
    commit_name="${commit_name%_*}"
    git commit -am "Added Part ${commit_name}"
fi
