

#[derive(PartialEq, Clone, Copy,Debug)]
enum Color {
    Yellow, Red, None
}
type Board = [Color; 6 * 7];

const HEIGHT: usize = 6;
const WIDTH: usize = 7;


fn main() {
    let mut board: Board = [Color::None; 6 * 7];

    let mut turn_count: u32 = 0;
    for line in std::io::stdin().lines() {
        if let Ok(mut x) = line.unwrap().parse() {
            x = std::cmp::min(7, x);
            x = std::cmp::max(1, x);
            x -= 1;

            let color = if turn_count % 2 == 0 { Color::Yellow } else { Color::Red};

            if let Some(pos) = place(x, color, &mut board) {
                if check_win(&board, color, pos.0, pos.1) {
                    println!("You won!");
                    print(&board);
                    return;
                }

                print(&board);
                println!("");
                turn_count += 1;
            } else {
                println!("Full, try again");
            }
        } else {
            println!("Enter number between 1 and 7");
        }
    }
}

fn check_win(board: &Board, color: Color, x: usize, y: usize) -> bool {
    let did_win =
          check_ray(board, color, x as i32, y as i32, 0, 1)
        || check_ray(board, color, x as i32, y as i32, 1, 1)
        || check_ray(board, color, x as i32, y as i32, -1, 1)
        || check_ray(board, color, x as i32, y as i32, -1, -1)
        || check_ray(board, color, x as i32, y as i32, 1, -1);

    did_win
}

fn check_ray(board: &Board, color: Color, mut x: i32, mut y: i32, dx: i32, dy: i32) -> bool {
    let mut count = 0;
    loop {
        x += dx;
        y += dy;

        // hit edge, ray is finished.
        if x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32 {
            break;
        }

        if board[y as usize * WIDTH + x as usize] != color {
            break;
        }

        count += 1;
    }

    count >= 3

}

fn print(board: &Board) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let piece = board[y * WIDTH + x];
            if piece == Color::Yellow {
                print!("#");
            } else if piece == Color::Red {
                print!("*");
            } else {
                print!("_");
            }
        }

        println!("");
    }
}

fn place(x: u32, color: Color, board: &mut Board) -> Option<(usize, usize)> {
    let mut y = HEIGHT - 1;
    let mut current_piece = board[y * WIDTH + x as usize];

    while current_piece != Color::None && y != 0 {
        y -= 1;
        current_piece = board[y * WIDTH + x as usize];
    }

    if current_piece != Color::None {
        return None;
    }

    board[y * WIDTH + x as usize] = color;

    Some((x as usize, y))
} 
