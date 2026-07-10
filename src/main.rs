use std::io;


fn main() {
    let mut runden_score = 1;
    let mut line1 = ['x', 'x', 'x'];
    let mut line2 = ['x', 'x', 'x'];
    let mut line3 = ['x', 'x', 'x'];
    let mut vec_roe:Vec<i32> = vec![];
    let mut vec_collum:Vec<i32> = vec![];
    //println!("   1 2 3 \n 1 x x x \n 2 x x x  \n 3 x x x ");

    loop {
        let mut buchstabe = 'a';
        runden_score += 1;
        if runden_score %2 == 0 {
            println!("player 1 ist dran");
            buchstabe = 'U';
        }
        else {
            println!("player 2 ist dran");
            buchstabe = 'w';
        }

        println!("gib deine zeile an");// welcher array
        let mut zeile = userinput_i32();

        println!("gib deine spalte ein");//wo im array
        let spalte = userinput_i32();

        if vec_roe.contains(&zeile) && vec_collum.contains(&spalte) {
            println!("das existiert schom");
            zeile = 3;
        }

        vec_collum.push(spalte);
        vec_roe.push(zeile);

        let spalte_usize = spalte as usize;
        match zeile {
            0 => line1[spalte_usize] = buchstabe,
            1 => line2[spalte_usize] = buchstabe,
            2 => line3[spalte_usize] = buchstabe,
            _=> runden_score -= 1
        };


        print_field(line1); //   print_field(line1: Vec<>); das Vec braucht man nich
        print_field(line2);
        print_field(line3);
        if runden_score == 0 {
            break
        }
    }
}
fn print_field(line1: [char; 3]){
    for line1 in &line1{
        print!(" {}", line1)
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
fn wincondition() {

}
