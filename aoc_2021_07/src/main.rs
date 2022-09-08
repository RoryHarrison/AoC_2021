fn main() {
    let mut values = parse_input("input.txt");
    let mut fuel_cost = 0;
    values.sort();
    let median = values[values.len()/2];

    for n in &values {
        fuel_cost += u32::abs_diff(*n, median);
    }
    
    println!("Fuel Cost: {}", fuel_cost);

    let mut crab_max = 0;
    for n in &values {
        crab_max = crab_max.max(*n);
    }
    let mut fuel_cost = vec![0; (crab_max+1) as usize];

    for pos in &values {
        for i in 0..crab_max+1 {
            let n = u32::abs_diff(i, *pos);
            fuel_cost[i as usize] += n*(n+1)/2;
        }
    }

    let min_fuel_two = fuel_cost.iter().min().unwrap();

    println!("Fuel Cost 2: {}", min_fuel_two);
}

fn parse_input(file: &str) -> Vec<u32>{
    std::fs::read_to_string(file)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}
