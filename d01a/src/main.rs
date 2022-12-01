fn main() {
    let max = include_str!("../input")
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|n| n.parse::<u32>().unwrap()))
        .map(|s| s.sum::<u32>())
        .max()
        .unwrap();

    println!("{}", max)
}
