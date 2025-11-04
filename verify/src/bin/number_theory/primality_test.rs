use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::math::prime::is_prime;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let q: usize = stdin.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let n: u64 = stdin.next().unwrap().parse().unwrap();
        writeln!(stdout, "{}", if is_prime(n) { "Yes" } else { "No" }).ok();
    }
}
