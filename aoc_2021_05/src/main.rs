struct Line {
    x1 : u32,
    x2 : u32,
    y1 : u32,
    y2 : u32,
}
impl Line {
    fn new(x1:u32, x2:u32, y1:u32, y2:u32) -> Self {
        Line {
            x1,
            x2,
            y1,
            y2,
        }
    }
}

fn main() {
    parse_input("input.txt");
}

fn populate_grid(line_list: Vec<Line>, grid: Vec<u32>) {
    let mut grid = grid;
    for line in line_list {
        println!("Line: x1:{}, x2:{}, y1:{}, y2:{}", line.x1, line.x2, line.y1, line.y2);
        let mut points: Vec<(u32, u32)> = Vec::new();
        for x in line.x1.min(line.y1)..line.x1.max(line.y1) {
            for y in line.x2.min(line.y2)..line.x2.max(line.y2) {
                points.push((x,y));
            }
        }
        for coord in points {
            grid[(coord.0*1000+coord.1) as usize] += 1;
        }
    }

    let score = grid.iter()
        .filter(|&x| x>&1)
        .count();

    println!("score: {}", score);
}

fn check_orientation(line: &Vec<u32>) -> bool {
    line[0] == line[1] || line [2] == line[3]
}

fn parse_input(file: &str) {
    let mut line_list: Vec<Line> = Vec::new();
    let grid: Vec<u32> = vec![0;1_000_000];
    match std::fs::read_to_string(file) {
        Ok(input) => {
            for row in input.as_str().lines() {
                let l: Vec<u32> = row.
                    replace(" -> ", ",")
                    .split(',')
                    .map(|x| x.to_owned().parse::<u32>().unwrap())
                    .collect();
                if check_orientation(&l) {
                line_list.push(Line::new(l[0], l[1], l[2], l[3]));
                } 
            }
        },
        Err(_) => ()
    }
    // for line in &line_list {
    //     println!("Line: x1:{}, x2:{}, y1:{}, y2:{}", line.x1, line.x2, line.y1, line.y2);
    // }
    populate_grid(line_list, grid);
}
