use std::io;

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}

fn check_winner(board: &Vec<Vec<char>>, player: char) -> bool {
    for i in 0..3 {
        if (0..3).all(|j| board[i][j] == player) ||     // Проверка строк
            (0..3).all(|j| board[j][i] == player)         // Проверка столбцов
        {
            return true;
        }
    }

    (0..3).all(|i| board[i][i] == player) ||             // Проверка главной диагонали
        (0..3).all(|i| board[i][2 - i] == player)             // Проверка побочной диагонали
}


fn main() {
    let mut board = vec![vec![' '; 3]; 3];
    let mut current_player = 'X';

    loop {
        println!("Игровое поле:");
        print_board(&board);
        println!("Ход игрока {}: ", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

        let coords: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Введите числа"))
            .collect();

        if coords.len() != 2 {
            println!("Введите две координаты (например, 0 1)");
            continue;
        }

        let row = coords[0];
        let col = coords[1];

        if row >= 3 || col >= 3 {
            println!("Неверные координаты, попробуйте снова.");
            continue;
        }

        if board[row][col] != ' ' {
            println!("Эта клетка уже занята, попробуйте другую.");
            continue;
        }

        board[row][col] = current_player;

        if check_winner(&board, current_player) {
            println!("Игрок {} выиграл!", current_player);
            print_board(&board);
            break;
        }

        if current_player == 'X' {
            current_player = 'O';
        } else {
            current_player = 'X';
        }
    }
}
