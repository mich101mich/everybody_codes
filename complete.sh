#!/bin/bash -e

cd "$(dirname "$0")"

part=$(command ls -1 src/parts/part_*.rs | sort | tail -n 1)
part="${part#src/parts/part_}"
part="${part%.rs}"

max_input=$(command ls -1 input/[0-9]*.txt | sort | tail -n 1)
max_input="${max_input#input/}"
max_input="${max_input%.txt}"
max_input="${max_input//-/_}"
if [[ "$part" != "$max_input" ]]; then
    echo "Input-part mismatch: '${max_input}' vs '${part}'"
    exit 1
fi

IFS='_' read -r year day step <<<"$part"

if [[ $step != "3" ]]; then
    step=$((step + 1))
else
    step=1
    if [[ $day != "20" ]]; then
        day="${day#0}" # strip leading 0, as it would cause octal parsing
        day=$((day + 1))
        day=$(printf "%02d" "$day")
    else
        day="01"
        year=$((year + 1))
    fi
fi
echo "year='$year' day='$day' step='$step'"

input_file="input/${year}-${day}-${step}.txt"
part="part_${year}_${day}_${step}"
part_file="src/parts/${part}.rs"

cp src/current.rs "$part_file"
cp input/current.txt "$input_file"
sed -i -e "s|include_str!(\"../input/current.txt\")|include_str!(\"../../${input_file}\")|g" "$part_file"

sed -i -e "s|// <INSERTION_POINT>|pub mod ${part};\n    // <INSERTION_POINT>|" src/main.rs

echo -n "" > input/current.txt

if [[ "$step" == "3" ]]; then
    cat > src/current.rs <<EOF
use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/current.txt");
    // let input = "";

    let parsed = input
        ;

    let result = parsed
        ;

    result!(result);
}
EOF

    git add .
    git commit -m "Added Part ${year}-${day}"
    git push
fi
