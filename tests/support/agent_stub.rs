use std::io::{self, BufRead};

fn main() {
    for argument in std::env::args().skip(1) {
        println!("ARG {argument:?}");
    }
    println!(
        "CWD {:?}",
        std::env::current_dir().unwrap().canonicalize().unwrap()
    );
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    println!("STDIN {line:?}");
    eprintln!("STUB STDERR");
    std::process::exit(
        std::env::var("PREAGENT_TEST_EXIT")
            .unwrap()
            .parse()
            .unwrap(),
    );
}
