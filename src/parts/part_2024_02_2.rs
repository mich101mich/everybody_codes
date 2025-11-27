pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-02-2.txt");
    //     let input = "WORDS:THE,OWE,MES,ROD,HER,QAQ

    // AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
    // THE FLAME SHIELDED THE HEART OF THE KINGS
    // POWE PO WER P OWE R
    // THERE IS THE END
    // QAQAQ";

    let mut lines = input.lines();
    let words = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();

    let words = words
        .iter()
        .map(|s| s.to_string())
        .chain(words.iter().map(|w| w.chars().rev().collect()))
        .collect::<Vec<_>>();

    lines.next().unwrap(); // empty line

    let mut is_runic = vec![];

    let mut result = 0;
    for row in lines {
        is_runic.clear();
        is_runic.resize(row.len(), false);
        for word in &words {
            let mut pos = 0;
            while pos < row.len()
                && let Some(offset) = row[pos..].find(word)
            {
                pos += offset;
                is_runic[pos..pos + word.len()].fill(true);
                pos += 1;
            }
        }
        result += is_runic.iter().filter(|x| **x).count();
    }

    result!(result);
}
