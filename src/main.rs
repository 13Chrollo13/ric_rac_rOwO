use std::collections::HashMap;
use std::io;
// 9 wahrscheinlichkeiten 3 waargerechte 3 senkrechte 2 diagonale

fn main() {
    let mut runden_score = 1;
    let mut line:[[i32; 3]; 3] = [[3i32; 3]; 3];
    //[[3, 3, 3], [3, 3, 3], [3, 3, 3]]
    println!("{:?}", line);

    let mut exist_check:HashMap<i32, i32> = HashMap::new();

    loop {
        let mut zahl = 3;
        runden_score += 1;
        if runden_score %2 == 0 {
            println!("player 1 ist dran");
            zahl = 1;
        }
        else {
            println!("player 2 ist dran");
            zahl = 2;
        }

        println!("gib deine zeile an");// welcher array
        let mut zeile = userinput_i32();

        println!("gib deine spalte ein");//wo im array
        let spalte = userinput_i32();



        for (roe,collum) in &exist_check  {
              if &spalte == roe && &zeile == collum {
                  println!("das existiert schon");
                  zeile = 3
              }
        }

        exist_check.insert(spalte, zeile);

        println!("{}",line[1][2]);
        let spalte_usize = spalte as usize;

        match zeile {
            0 => line[0][spalte_usize] = zahl,
            1 => line[1][spalte_usize] = zahl,
            2 => line[2][spalte_usize] = zahl,
            _=> runden_score -= 1
        }
        print_field(line);

        if runden_score == 0 {
            break
        }
    }
}
fn print_field(line: [[i32; 3]; 3]){
    let mut for_runde = 0;
    for line in &line{
        print!(" {}", line[for_runde]);
        for_runde += 1;
    }
    print!("\n")
}


fn userinput_i32() -> i32 {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to readline");
    let a = a.trim();
    let userinput: i32 = a.parse().unwrap();
    userinput
}
