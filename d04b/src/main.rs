fn main() {
    let sum = include_str!("../input")
        .replace(",", "-")
        .lines()
        .map(|l| {
            l.split("-")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|arr| match arr.as_slice() {
            [a, b, c, d] => (b >= c && a <= d) || (d >= a && c <= b),
            _ => false,
        })
        .count();

    println!("{:?}", sum);
}
