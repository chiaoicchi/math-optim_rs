use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::{algebra::monoid::Monoid, ds::segtree::SegTree};

const MOD: u32 = 998_244_353;

enum O {}
impl Monoid for O {
    type S = (u32, u32);
    fn identity() -> Self::S {
        (1, 0)
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        let x = (a.0 as u64 * b.0 as u64 % MOD as u64) as u32;
        let y = (a.1 as u64 * b.0 as u64 % MOD as u64) as u32 + b.1;
        (x, if y >= MOD { y - MOD } else { y })
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let ab: Vec<(u32, u32)> = (0..n)
        .map(|_| {
            (
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut segtree = SegTree::<O>::from_vec(&ab);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            let p: usize = stdin.next().unwrap().parse().unwrap();
            let c: u32 = stdin.next().unwrap().parse().unwrap();
            let d: u32 = stdin.next().unwrap().parse().unwrap();
            segtree.set(p, (c, d));
        } else {
            let l: usize = stdin.next().unwrap().parse().unwrap();
            let r: usize = stdin.next().unwrap().parse().unwrap();
            let x: u64 = stdin.next().unwrap().parse().unwrap();
            let (a, b) = segtree.range_fold(l..r);
            let ans = (a as u64 * x % MOD as u64) as u32 + b;
            writeln!(stdout, "{}", if ans >= MOD { ans - MOD } else { ans }).ok();
        }
    }
}
