const DIRECTIONS: [(i8, i8); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];
const MINE: char = '*';
const ONE: char = '1';
const EMPTY: char = ' ';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return Vec::new();
    }

    let (mut board, size) = generate_board(minefield);
    for i in 0..size.0 {
        for j in 0..size.1 {
            if minefield[i].chars().nth(j).unwrap() == MINE {
                board[i][j] = MINE;

                for (row_d, col_d) in DIRECTIONS {
                    update_board(i, j, row_d, col_d, size, &mut board, minefield);
                }
            }
        }
    }

    return generate_answer(board);
}

fn add(u: usize, i: i8) -> Option<usize> {
    if i.is_negative() {
        let abs_i = i.wrapping_abs();
        if abs_i as usize > u {
            return None;
        }
        Some(u - abs_i as u8 as usize)
    } else {
        Some(u + i as usize)
    }
}
fn generate_board(minefield: &[&str]) -> (Vec<std::vec::Vec<char>>, (usize, usize)) {
    let size = (minefield.len(), minefield[0].len());
    let line = vec![' '; size.1];
    let board = vec![line; size.0];
    return (board, size);
}

fn generate_answer(board: Vec<std::vec::Vec<char>>) -> Vec<std::string::String> {
    let mut answer: Vec<String> = Vec::new();
    for row in board {
        let row_str = row.into_iter().collect();
        answer.push(row_str);
    }
    return answer;
}

fn update_board(
    i: usize,
    j: usize,
    row_d: i8,
    col_d: i8,
    size: (usize, usize),
    board: &mut Vec<std::vec::Vec<char>>,
    minefield: &[&str],
) {
    let row_index = add(i, row_d);
    let col_index = add(j, col_d);
    if row_index.is_none() || col_index.is_none() {
        return;
    }
    let row = row_index.unwrap();
    let col = col_index.unwrap();
    if row < size.0 && col < size.1 && minefield[row].chars().nth(col).unwrap() != MINE {
        if board[row][col] == EMPTY {
            board[row][col] = ONE;
        } else {
            board[row][col] = ((board[row][col] as u8) + 1) as char;
        }
    }
}
