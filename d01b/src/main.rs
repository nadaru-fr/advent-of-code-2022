fn main() {
    let mut values = include_str!("../input")
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|n| n.parse::<u32>().unwrap()))
        .map(|s| s.sum::<u32>())
        .collect::<Vec<u32>>();

    values.sort();

    println!("{:?}", &values[(values.len() - 3)..].iter().sum::<u32>());
}
