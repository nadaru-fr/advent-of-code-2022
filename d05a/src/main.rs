fn main() {
    let (matrix, instructions) = include_str!("../input").split_once("\n\n").unwrap();

    let mut reversed = matrix.lines().rev();

    let size = (reversed.next().unwrap().len() + 1) / 4;

    // TODO: Check if there's a better way to create matrix
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..size {
        stacks.push(Vec::new())
    }

    loop {
        match reversed.next() {
            None => break,
            Some(line) => {
                let chars = line.chars().collect::<Vec<char>>();
                for i in 0..size {
                    let &char = chars.get(i * 4 + 1).unwrap();
                    if char != ' ' {
                        stacks[i].push(char);
                    }
                }
            }
        }
    }

    instructions
        .lines()
        .map(|i| i.split_whitespace().collect::<Vec<_>>())
        .map(|i| {
            (
                i[1].parse::<usize>().unwrap(),
                i[3].parse::<usize>().unwrap(),
                i[5].parse::<usize>().unwrap(),
            )
        })
        .for_each(|(n, from, to)| {
            for _i in 0..n {
                let val = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(val);
            }
        });

    let res = stacks
        .iter()
        .fold(String::new(), |a, b| a + &b.last().unwrap().to_string());

    println!("{:?}", res)
}
