struct Board {
    board : Vec<u32>,
    marked : Vec<u32>,
    won: bool
}

impl Board {
    fn new(b: Vec<u32>) -> Self{
        Board {
            board : b,
            marked : vec![0;25],
            won : false,
        }
    }
}


fn main() {
    parse_input("input.txt");
}

fn parse_input(input: &str) {
    println!("Parsing Input");
    let mut nums: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new(); 

    //Parsing input into nums/boards
    match std::fs::read_to_string(input) {
        Ok(lines) => {
            for (idx, line) in lines.as_str().split("\n\n").enumerate() {
                if idx == 0 {
                    nums = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                    continue;
                }
                let mut b: Vec<u32> = Vec::new();
                for b_line in line.lines() {
                    for x in b_line.split_whitespace() {
                        b.push(x.parse::<u32>().unwrap());
                    }
                }
                boards.push(Board::new(b));
            }
        },
        Err(_) => (),
    }

    play(nums, boards);
}

fn play(nums: Vec<u32>, boards: Vec<Board>) {
    let mut boards = boards;
    for num in nums {
        for x in 0..boards.len() {
            match boards[x].board.iter().position(|&x| x == num) {
                Some(idx) => boards[x].marked[idx] = 1,
                None => ()
            }
            if check_win(&boards[x]) {
                if boards.iter().filter(|&x| x.won == false).count() == 1 as usize{
                    get_score(&boards[x], num);
                }
                boards[x].won = true;
            }
        }
    }
    println!("We get here");
}

fn get_score(board: &Board, num: u32) {
    let unmarked_indexes: Vec<usize> = board.marked
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == 0)
        .map(|(index, _)| index)
        .collect();
    let mut sum = 0;
    for x in unmarked_indexes {
        sum += board.board[x];
    }
    let final_score = sum*num;
    println!("Final Score: {}", final_score);
        
}

fn check_win(board: &Board) -> bool {
    check_horizontal(board) || check_vertical(board)
}

fn check_vertical(board: &Board) -> bool {
    let board_size = 5;
    for i in 0..board_size {
        let mut win = true;
        for j in 0..board_size {
            if board.marked[(i*board_size+j)] == 0 {
                win = false;
            }
        }
        if win == true {
            return win;
        }
    }
    false
}

fn check_horizontal(board: &Board) -> bool {
    let board_size = 5;
    
    for i in 0..board_size {
        let mut win = true;
        for j in 0..board_size {
            if board.marked[j*board_size+i] == 0 {
                win = false;
            }
        }
        if win == true {
            return win;
        }
    }
    false
}
