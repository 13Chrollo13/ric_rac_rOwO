use std::collections::HashMap;
use std::io;
// 9 wahrscheinlichkeiten 3 waargerechte 3 senkrechte 2 diagonale

fn main() {
    let mut runden_score = 1;
    let mut line: [[i32; 3]; 3] = [[3i32; 3]; 3];
    //[[3, 3, 3], [3, 3, 3], [3, 3, 3]]
    println!("{:?}", line);

    let mut exist_check: HashMap<i32, i32> = HashMap::new();

    loop {
        let mut zahl = 0;
        runden_score += 1;
        if runden_score % 2 == 0 {
            println!("player 1 ist dran");
            zahl = 1;
        } else {
            println!("player 2 ist dran");
            zahl = 2;
        }

        println!("gib deine zeile an"); // welcher array
        let mut zeile = userinput_i32();

        println!("gib deine spalte ein"); //wo im array
        let spalte = userinput_i32();

        for (roe, collum) in &exist_check {
            if &spalte == roe && &zeile == collum {
                println!("das existiert schon");
                zeile = 3
            }
        }

        exist_check.insert(spalte, zeile);

        let spalte_usize = spalte as usize;

        match zeile {
            0 => line[0][spalte_usize] = zahl,
            1 => line[1][spalte_usize] = zahl,
            2 => line[2][spalte_usize] = zahl,
            _ => runden_score -= 1,
        }
        println!("UwU {}", line[0 as usize][1 as usize]);
        println!("AwA {}", line[1 as usize][0 as usize]);
        print_field(line);

        let won = row_won(line);

        if won == true {
            if runden_score %2 == 0 {
                println!("player 1 hat gewonnen");
                break
            }
            if runden_score %2 == 1 {
                println!("player 2 hat gewonnen");
                break
            }
        }
    }
}
fn print_field(line: [[i32; 3]; 3]) {
    let mut roe_runde = 0;
    let mut collum_runde = 0;
    loop {
        match line[collum_runde][roe_runde] {
            1 => print!("U "),
            2 => print!("w "),
            _ => print!("x "),
        }

        roe_runde += 1;
        if roe_runde % 3 == 0 {
            print!("\n");
            collum_runde += 1;
            roe_runde = 0
        }
        if collum_runde == 3 {
            break;
        }
    }
    print!("\n")
}

fn userinput_i32() -> i32 {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to readline"); //todo add error handling for this line
    let a = a.trim();
    match a.parse() {
        Ok(userinput) => userinput,
        Err(err) => userinput_i32(),
    }
}

fn row_won(line: [[i32; 3]; 3]) -> bool {
    let mut row = 0;
    let mut collum = 0;
    loop {
        if line[row][0 as usize] == line[row][1 as usize]
            && line[row][0 as usize] == line[row][2 as usize]
            && line[row][0 as usize] != 3
        {
            return true
        }
        if line[0 as usize][collum] == line[1 as usize][collum]
            && line[0 as usize][collum] == line[2 as usize][collum]
            && line[0 as usize][collum] != 3
        {
            return true
        }
        collum += 1;
        if collum %3 == 0 {
            row += 1;
            collum = 0
        }
        if row == 3 {
            break;
        }
    }
    if line[0 as usize][0 as usize] == line[1 as usize][1 as usize]
        && line[2 as usize][2 as usize] == line[0 as usize][0 as usize]
        && line[0 as usize][0 as usize] != 3
    {
        return true
    }
    if line[0 as usize][2 as usize] == line[1 as usize][1 as usize]
        && line[2 as usize][0 as usize] == line[1 as usize][1 as usize]
        && line[1 as usize][1 as usize] != 3
    {
        return true
    }
    false
}
