use std::collections::HashSet;

fn main() {
    let res = include_str!("../input")
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|x| x.into_iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        + 4;

    println!("{:?}", res)
}
