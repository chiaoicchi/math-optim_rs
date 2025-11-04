use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::math::{conv::ntt::ntt_conv, gf::GF};

const MOD: u32 = 998_244_353;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let m: usize = stdin.next().unwrap().parse().unwrap();

    let a: Vec<GF<MOD>> = (0..n)
        .map(|_| GF::<MOD>::new(stdin.next().unwrap().parse().unwrap()))
        .collect();
    let b: Vec<GF<MOD>> = (0..m)
        .map(|_| GF::<MOD>::new(stdin.next().unwrap().parse().unwrap()))
        .collect();

    let ans = ntt_conv(&a, &b);
    write!(stdout, "{}", ans[0]).ok();
    for x in &ans[1..] {
        write!(stdout, " {}", x).ok();
    }
    writeln!(stdout).ok();
}
