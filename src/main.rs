use std::io;

fn terminal(s: [i32; 9]) -> bool {
    let winning_combinations = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
        [0, 4, 8], [2, 4, 6],            // diagonals
    ];

    for combo in winning_combinations.iter() {
        if s[combo[0]] != 0 && s[combo[0]] == s[combo[1]] && s[combo[1]] == s[combo[2]] {
            return true;
        }
    }

    !s.contains(&0) // If no empty spots, it's a draw
}

fn evaluate(s: [i32; 9]) -> i32 {
    let winning_combinations = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
        [0, 4, 8], [2, 4, 6],            // diagonals
    ];

    for combo in winning_combinations.iter() {
        if s[combo[0]] != 0 && s[combo[0]] == s[combo[1]] && s[combo[1]] == s[combo[2]] {
            return if s[combo[0]] == 1 { 1 } else { -1 };
        }
    }

    0 // No winner yet
}

fn minimax(s: [i32; 9], depth: i32, is_max: bool) -> i32 {
    let score = evaluate(s);
    if score == 1 {
        return score - depth; // Maximize X, subtract depth for faster win
    }
    if score == -1 {
        return score + depth; // Minimize O, add depth for faster win
    }
    if !s.contains(&0) {
        return 0; // Draw
    }

    if is_max {
        let mut best = -1000;
        for i in 0..9 {
            if s[i] == 0 {
                let mut new_state = s;
                new_state[i] = 1;
                best = best.max(minimax(new_state, depth + 1, false));
            }
        }
        best
    } else {
        let mut best = 1000;
        for i in 0..9 {
            if s[i] == 0 {
                let mut new_state = s;
                new_state[i] = 2;
                best = best.min(minimax(new_state, depth + 1, true));
            }
        }
        best
    }
}

fn best_move(s: [i32; 9], player: i32) -> usize {
    let mut best_val = if player == 1 { -1000 } else { 1000 };
    let mut best_move = 0;

    for i in 0..9 {
        if s[i] == 0 {
            let mut new_state = s;
            new_state[i] = player;
            let move_val = if player == 1 {
                minimax(new_state, 0, false)
            } else {
                minimax(new_state, 0, true)
            };

            if (player == 1 && move_val > best_val) || (player == 2 && move_val < best_val) {
                best_val = move_val;
                best_move = i;
            }
        }
    }

    best_move
}

fn print_board(s: [i32; 9]) {
    for i in 0..9 {
        if i % 3 == 0 && i != 0 {
            println!();
        }
        let symbol = match s[i] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        };
        print!("{} ", symbol);
    }
    println!();
}

fn main() {
    let mut game = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut current_player = 1; // X starts first

    loop {
        print_board(game);

        if terminal(game) {
            let score = evaluate(game);
            if score == 1 {
                println!("X wins!");
            } else if score == -1 {
                println!("O wins!");
            } else {
                println!("It's a draw!");
            }
            break;
        }

        if current_player == 1 {
            // Human player's turn (X)
            let mut input = String::new();
            println!("Enter your move (0-8): ");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mv: usize = match input.trim().parse() {
                Ok(num) if num < 9 => num,
                _ => {
                    println!("Invalid move, try again.");
                    continue;
                }
            };
            if game[mv] == 0 {
                game[mv] = 1;
                current_player = 2;
            } else {
                println!("Invalid move, try again.");
            }
        } else {
            // Computer player's turn (O)
            let mv = best_move(game, 2);
            game[mv] = 2;
            current_player = 1;
        }
    }
}
