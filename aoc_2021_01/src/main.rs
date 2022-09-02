use std::fs;

fn main() {
    //Get Input
    let depths = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();


    let p1_answer = part_one(&depths);
    println!("Part One: {}", p1_answer);

    let p2_answer = part_two(&depths);
    println!("Part Two: {}", p2_answer);
}

fn part_one(depths: &Vec<u32>) -> u32 {
    depths.iter()
        .zip(depths.iter().skip(1))
        .filter(|(i, j)| i < j)
        .count() as u32
}

fn part_two(depths: &Vec<u32>) -> u32 {
    depths.windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count() as u32
}
