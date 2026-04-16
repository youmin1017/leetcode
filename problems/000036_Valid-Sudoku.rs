struct Solution;

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let res = Solution::is_valid_sudoku(board);
    println!("{}", res)
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }
                if Solution::is_contains_duplicated_in_sub_box(&board, r, c)
                    || Solution::is_contains_duplicated_in_rc(&board, r, c)
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_contains_duplicated_in_sub_box(board: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
        let start_r = (r / 3) * 3;
        let start_c = (c / 3) * 3;
        let target = board[r][c];

        for cr in start_r..(start_r + 3) {
            for cc in start_c..(start_c + 3) {
                if cr == r && cc == c {
                    continue;
                }
                if board[cr][cc] == target {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_contains_duplicated_in_rc(board: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
        let target = board[r][c];

        for i in 0..9 {
            if i != c && board[r][i] == target {
                return true;
            }
            if i != r && board[i][c] == target {
                return true;
            }
        }
        false
    }
}
