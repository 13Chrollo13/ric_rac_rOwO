use std::collections::HashMap;
use std::io;
// 9 wahrscheinlichkeiten 3 waargerechte 3 senkrechte 2 diagonale

fn main() {
    let mut runden_score = 1;
    let mut line:[[i32; 3]; 3] = [[3i32; 3]; 3];
    //[[3, 3, 3], [3, 3, 3], [3, 3, 3]]
    println!("{:?}", line);
    /*
    let mut line1 = ['x', 'x', 'x'];
    let mut line2 = ['x', 'x', 'x'];
    let mut line3 = ['x', 'x', 'x'];
    let mut wincondition_array = [1,2,3];
     */

    let mut exist_check:HashMap<i32, i32> = HashMap::new();


    /*
    let mut vec_roe:Vec<i32> = vec![];
    let mut vec_collum:Vec<i32> = vec![];
     */
    //println!("   1 2 3 \n 1 x x x \n 2 x x x  \n 3 x x x ");

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

        /*
        match zeile {
            0 => line1[spalte_usize] = buchstabe,
            1 => line2[spalte_usize] = buchstabe,
            2 => line3[spalte_usize] = buchstabe,
            _=> runden_score -= 1
        };


        let bit1 = print_field(line); //   print_field(line1: Vec<>); das Vec braucht man nich
        let bit2 = print_field(line2);
        let bit3 = print_field(line3);

         */


        if runden_score == 0 {
            break
        }
        //let a = wincondition(bit1, bit2, bit3);
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

//todo!("use a 3d array instead of three different")


/*
fn wincondition(bit1: i32, bit2: i32,bit3: i32) -> i32 {
    if bit1 == 255 || bit1 == 357 || bit2 == 255 || bit2 == 357 || bit3 == 255 || bit3 == 357 {
        1
    }
}
 U U U 255
 w w w 357
 x x x 360
 */