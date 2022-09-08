fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let uniques: Vec<usize> = vec![2, 4, 3, 7];

    let mut count = 0;
    for line in input.lines() {
        count += line
            .split('|')
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .filter(|x| uniques.contains(&x.len()))
            .count();
    }
    println!("count: {}", count);
}

fn part_two(input: &String) {
    
}
