fn main() {
    let allchars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    let sum = include_str!("../input")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            allchars
                .clone()
                .find(|&char| chunk.iter().all(|r| r.contains(char)))
        })
        .map(|char| char.unwrap() as u32)
        .map(|char| if char > 90 { char - 96 } else { char - 38 })
        .sum::<u32>();

    println!("{}", sum);
}
