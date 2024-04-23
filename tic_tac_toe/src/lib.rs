pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let mut result = String::from("tie");

    let players = vec!["X", "O"];

    for player in players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            result = format!("player {} won", player);
            break;
        }
    }

    result
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    table[1][1] == player
        && ((table[0][0] == player && table[2][2] == player)
            || (table[0][2] == player && table[2][0] == player))
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for i in 0..3 {
        if table.iter().all(|row| row[i] == player) {
            return true;
        }
    }
    false
}
