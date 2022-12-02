fn main() {
    let score = include_str!("../input")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(0, |n, (opp, me)| match (opp, me, n) {
            ("A", "X", n) => n + 3 + 1,
            ("A", "Y", n) => n + 6 + 2,
            ("A", "Z", n) => n + 0 + 3,
            ("B", "X", n) => n + 0 + 1,
            ("B", "Y", n) => n + 3 + 2,
            ("B", "Z", n) => n + 6 + 3,
            ("C", "X", n) => n + 6 + 1,
            ("C", "Y", n) => n + 0 + 2,
            ("C", "Z", n) => n + 3 + 3,
            _ => unreachable!(),
        });

    println!("{}", score)
}
