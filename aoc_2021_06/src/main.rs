fn main() {
    let input = parse_input("input.txt");
    let start_fish: Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

    const SIM_DAYS: usize = 256;
    const FISH_AGE: usize = 6;
    const NEW_FISH_AGE: usize = 8;

    let mut fish_schedule: Vec<u64> = vec![0; SIM_DAYS+NEW_FISH_AGE+1];
    for f in &start_fish {
        fish_schedule[*f] += 1;
    }

    let mut fish_count = start_fish.len() as u64;

    for day in 0..SIM_DAYS {
        let todays_fish = fish_schedule[day];
        fish_schedule[day + FISH_AGE+1] += todays_fish;
        fish_schedule[day + NEW_FISH_AGE+1] += todays_fish;
        fish_count += todays_fish;
    }
    println!("\nNumber of fish after {} days: {}", SIM_DAYS, fish_count);
}

fn parse_input(file: &str) -> String {
    match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => "Error".to_string()
    }
}
