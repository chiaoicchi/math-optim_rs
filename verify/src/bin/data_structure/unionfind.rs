use std::io::{read_to_string, stdin, stdout, BufWriter, Write};

use math_optim::ds::dsu::DSU;
fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let mut dsu = DSU::new(n);
    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        let u: usize = stdin.next().unwrap().parse().unwrap();
        let v: usize = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            dsu.union(u, v);
        } else {
            writeln!(stdout, "{}", dsu.is_same(u, v) as u8).ok();
        }
    }
}
