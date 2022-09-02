use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let p1 = part_one(&input);
    println!("P1: {}", p1);

    let p2 = part_two(&input);
    println!("P1: {}", p2);
}

fn part_one(input: &Vec<String>) -> u32 {
    let mut pos = (0,0);
    for ins in input {
        match ins.split_once(" ").unwrap() {
            (x, y) if x == "forward" => pos.0 += y.parse::<u32>().unwrap(),
            (x, y) if x == "down" => pos.1 += y.parse::<u32>().unwrap(),
            (x, y) if x =="up" => pos.1 -= y.parse::<u32>().unwrap(),
            _ => pos = (1, 1)
        }
    }
    pos.0 * pos.1
}

fn part_two(input: &Vec<String>) -> u32 {
    let mut pos = (0,0,0);
    for ins in input {
        match ins.split_once(" ").unwrap() {
            (x, y) if x == "forward" => {
                let x = y.parse::<u32>().unwrap();
                pos.0 += x;
                pos.1 += pos.2*x;
            },
            (x, y) if x == "down" => pos.2 += y.parse::<u32>().unwrap(),
            (x, y) if x =="up" => pos.2 -= y.parse::<u32>().unwrap(),
            _ => pos = (1,1,1)
        }
    }
    pos.0 * pos.1
}
