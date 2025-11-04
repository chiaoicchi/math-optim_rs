use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::{algebra::group::Group, ds::potential_dsu::PotentialDSU};

const MOD: u32 = 998_244_353;

enum O {}
impl Group for O {
    type G = (u32, u32, u32, u32);
    fn identity() -> Self::G {
        (1, 0, 0, 1)
    }
    fn op(a: &Self::G, b: &Self::G) -> Self::G {
        let x = (a.0 as u64 * b.0 as u64 % MOD as u64) as u32
            + (a.1 as u64 * b.2 as u64 % MOD as u64) as u32;
        let y = (a.0 as u64 * b.1 as u64 % MOD as u64) as u32
            + (a.1 as u64 * b.3 as u64 % MOD as u64) as u32;
        let z = (a.2 as u64 * b.0 as u64 % MOD as u64) as u32
            + (a.3 as u64 * b.2 as u64 % MOD as u64) as u32;
        let w = (a.2 as u64 * b.1 as u64 % MOD as u64) as u32
            + (a.3 as u64 * b.3 as u64 % MOD as u64) as u32;
        (
            if x >= MOD { x - MOD } else { x },
            if y >= MOD { y - MOD } else { y },
            if z >= MOD { z - MOD } else { z },
            if w >= MOD { w - MOD } else { w },
        )
    }
    fn inv(a: &Self::G) -> Self::G {
        (
            a.3,
            if a.1 == 0 { 0 } else { MOD - a.1 },
            if a.2 == 0 { 0 } else { MOD - a.2 },
            a.0,
        )
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let mut potential_dsu = PotentialDSU::<O>::new(n);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        let u: usize = stdin.next().unwrap().parse().unwrap();
        let v: usize = stdin.next().unwrap().parse().unwrap();

        if t == 0 {
            let x: (u32, u32, u32, u32) = (
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            );
            writeln!(stdout, "{}", potential_dsu.union(v, u, &x) as u8).ok();
        } else {
            if let Some(x) = potential_dsu.potential(v, u) {
                writeln!(stdout, "{} {} {} {}", x.0, x.1, x.2, x.3).ok();
            } else {
                writeln!(stdout, "-1").ok();
            }
        }
    }
}
