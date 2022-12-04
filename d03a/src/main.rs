fn main() {
    let sum = include_str!("../input")
        .lines()
        .map(|inp| inp.split_at(inp.len() / 2))
        // Find character that is contained in both strings
        .map(|(r1, r2)| r1.chars().rfind(|&c| r2.contains(c)).unwrap())
        .map(|char| char as u32) // Convert to ASCII number
        .map(|char| if char > 90 { char - 96 } else { char - 38 })
        .sum::<u32>();

    println!("{}", sum);
}
