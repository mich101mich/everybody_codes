#![allow(unused)]

macro_rules! pv {
    ($var: expr) => {
        println!("{}: {:?}", stringify!($var), $var)
    };
    ($start: expr, $($var: expr),+) => {
        let mut s = format!("{}: {:?}", stringify!($start), $start);
        $(
            s += &format!(",  {}: {:?}", stringify!($var), $var);
        );*
        println!("{}", s);
    };
}

macro_rules! print_arr {
    ($var: ident) => {
        print!("{}: ", stringify!($var));
        for v in $var.iter() {
            print!("{}", v);
        }
        println!();
    };
}

pub fn copy(out: String) {
    use copypasta::ClipboardProvider;
    copypasta::ClipboardContext::new()
        .unwrap()
        .set_contents(out)
        .unwrap()
}

macro_rules! result {
    ($var: expr) => {
        let mut s = format!("{:?}", $var);
        if s.starts_with('"') && s.ends_with('"') {
            s = s[1..s.len() - 1].to_string();
        }
        println!("result: {}", s);
        crate::utils::copy(s);
    };
    ($format: literal, $($args: expr),+) => {
        let s = format!($format, $($args),+);
        println!("result: {}", s);
        crate::utils::copy(s);
    };
}
