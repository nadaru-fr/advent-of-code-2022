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
            [a, b, c, d] => a >= c && b <= d || c >= a && d <= b,
            _ => false,
        })
        .count();

    println!("{:?}", sum);
}
