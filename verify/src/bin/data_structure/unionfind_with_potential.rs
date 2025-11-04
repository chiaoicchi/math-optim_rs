use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::{algebra::group::Group, ds::potential_dsu::PotentialDSU};

const MOD: u32 = 998_244_353;

enum O {}
impl Group for O {
    type G = u32;
    fn identity() -> Self::G {
        0
    }
    fn op(a: &Self::G, b: &Self::G) -> Self::G {
        let x = a + b;
        if x >= MOD { x - MOD } else { x }
    }
    fn inv(a: &Self::G) -> Self::G {
        if *a == 0 { *a } else { MOD - a }
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
            let x: u32 = stdin.next().unwrap().parse().unwrap();
            writeln!(stdout, "{}", potential_dsu.union(v, u, &x) as u8).ok();
        } else {
            writeln!(
                stdout,
                "{}",
                potential_dsu.potential(v, u).unwrap_or(!0) as i32
            )
            .ok();
        }
    }
}
