fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let mut winner = "";
    // check rows
    for row in table.iter() {
        if row[0] != "#" && row.iter().all(|&x| x == row[0]) {
            winner = row[0];
            break;
        }
    }
    // check columns
    for i in 0..3 {
        if table[0][i] != "#" && table.iter().all(|row| row[i] == table[0][i]) {
            winner = table[0][i];
            break;
        }
    }
    // check diagonals
    if table[0][0] != "#" && table.iter().enumerate().all(|(i, row)| row[i] == table[0][0]) {
        winner = table[0][0];
    }
    if table[0][2] != "#" && table.iter().enumerate().all(|(i, row)| row[2-i] == table[0][2]) {
        winner = table[0][2];
    }
    if winner == "O" {
        return "player O won".to_string();
    } else if winner == "X" {
        return "player X won".to_string();
    } else if table.iter().all(|row| row.iter().all(|&x| x != "")) {
        return "tie".to_string();
    } else {
        return "".to_string();
    }
}


pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut win = false;
    if table.iter().enumerate().all(|(i, row)| row[i] == player) {
        win = true;
    }
    if table.iter().enumerate().all(|(i, row)| row[2-i] == player) {
        win = true;
    }
    return win
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut win = false;
    for row in table.iter() {
        if row.iter().all(|&x| x == player) {
            win = true;
        }
    }
    return win
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut win = false;
    for i in 0..3 {
        if table.iter().all(|row| row[i] == player) {
            win = true;
        }
    }
    return win
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         println!(
//             "{:?}",
//             tic_tac_toe(vec![
//                 vec!["O", "X", "O"],
//                 vec!["O", "O", "X"],
//                 vec!["X", "#", "X"]
//             ])
//         );
    
//         println!(
//             "{:?}",
//             tic_tac_toe(vec![
//                 vec!["X", "O", "O"],
//                 vec!["X", "O", "O"],
//                 vec!["#", "O", "X"]
//             ])
//         );
    
//         let dig = vec![
//                 vec!["O", "O", "X"],
//                 vec!["O", "X", "O"],
//                 vec!["X", "#", "X"]
//             ];
    
//         println!("{:?}",tic_tac_toe(dig));
//     }
// }
