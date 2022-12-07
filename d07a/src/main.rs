use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut pwd = String::from("/");
    let mut directories = HashMap::new();

    let lines = include_str!("../input")
        .lines()
        .skip(1)
        // Unused commands
        .filter(|&l| l != "$ ls" && !l.starts_with("dir"));

    for line in lines {
        if line.starts_with("$ cd ..") {
            pwd = get_parent(pwd);
            continue;
        }

        if line.starts_with("$ cd") {
            pwd.push_str(line.split(' ').last().unwrap());
            pwd.push('/');
            continue;
        }

        let size = parse_first_as_number(line);

        let mut path = pwd.clone();
        loop {
            directories.insert(path.clone(), directories.get(&path).unwrap_or(&0) + size);

            if path == "/" {
                break;
            }

            path = get_parent(path)
        }
    }

    let res = directories.values().filter(|&&v| v <= 100000).sum::<u32>();

    println!("{}", res);
}

fn get_parent(dir: String) -> String {
    let re = Regex::new(r"[a-z]+/$").unwrap();
    return re.replace(&dir, "").to_string();
}

fn parse_first_as_number(line: &str) -> u32 {
    return line
        .split(' ')
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .parse::<u32>()
        .unwrap();
}
