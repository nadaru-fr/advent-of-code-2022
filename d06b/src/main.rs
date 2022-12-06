use std::collections::HashSet;

fn main() {
    let res = include_str!("../input")
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .position(|x| x.into_iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14;

    println!("{:?}", res)
}
