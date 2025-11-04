use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::math::prime::factorize;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let q: usize = stdin.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let a: u64 = stdin.next().unwrap().parse().unwrap();
        let ans = factorize(a);
        write!(stdout, "{}", ans.len()).ok();
        for x in &ans {
            write!(stdout, " {}", x).ok();
        }
        writeln!(stdout).ok();
    }
}
