use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::{algebra::monoid_action::MonoidAction, ds::lazy_segtree::LazySegTree};

const MOD: u32 = 998_244_353;

enum O {}
impl MonoidAction for O {
    type S = (u32, u32);
    type F = (u32, u32);
    fn identity_s() -> Self::S {
        (0, 0)
    }
    fn identity_f() -> Self::F {
        (1, 0)
    }
    fn op_s(a: &Self::S, b: &Self::S) -> Self::S {
        let x = a.0 + b.0;
        (if x >= MOD { x - MOD } else { x }, a.1 + b.1)
    }
    fn op_f(a: &Self::F, b: &Self::F) -> Self::F {
        let x = (a.0 as u64 * b.0 as u64 % MOD as u64) as u32;
        let y = (a.1 as u64 * b.0 as u64 % MOD as u64) as u32 + b.1;
        (x, if y >= MOD { y - MOD } else { y })
    }
    fn apply(x: &mut Self::S, f: &Self::F) {
        let a = (x.0 as u64 * f.0 as u64 % MOD as u64) as u32
            + (x.1 as u64 * f.1 as u64 % MOD as u64) as u32;
        *x = (if a >= MOD { a - MOD } else { a }, x.1);
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let a: Vec<(u32, u32)> = (0..n)
        .map(|_| (stdin.next().unwrap().parse().unwrap(), 1))
        .collect();

    let mut lazy_segtree = LazySegTree::<O>::from_slice(&a);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        let l: usize = stdin.next().unwrap().parse().unwrap();
        let r: usize = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            let b: u32 = stdin.next().unwrap().parse().unwrap();
            let c: u32 = stdin.next().unwrap().parse().unwrap();
            lazy_segtree.range_apply(l..r, &(b, c));
        } else {
            writeln!(stdout, "{}", lazy_segtree.range_fold(l..r).0).ok();
        }
    }
}
