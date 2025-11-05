use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

use math_optim::geometry::{convex_hull::andrew, vector_2d::Vector2D};

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let t: usize = stdin.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = stdin.next().unwrap().parse().unwrap();
        let xy: Vec<Vector2D> = (0..n)
            .map(|_| {
                Vector2D::new(
                    stdin.next().unwrap().parse().unwrap(),
                    stdin.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        let hull = andrew(&xy);

        writeln!(stdout, "{}", hull.len()).ok();
        for point in &hull {
            writeln!(stdout, "{} {}", point.x(), point.y()).ok();
        }
    }
}
