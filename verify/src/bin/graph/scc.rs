// Cannot verify local Library Checker.
// Checked by online Library Checker.

use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::graph::scc::kosaraju;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let m: usize = stdin.next().unwrap().parse().unwrap();

    let ab: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            (
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let ans = kosaraju(n, &ab);

    writeln!(stdout, "{}", ans.len()).ok();

    for v in &ans {
        write!(stdout, "{}", v.len()).ok();
        for v in v {
            write!(stdout, " {}", v).ok();
        }
        writeln!(stdout).ok();
    }
}
