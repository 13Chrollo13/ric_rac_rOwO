use std::io;


fn main() {
    let mut runden_score = 1;
    let mut line1 = ['x', 'x', 'x'];
    let mut line2 = ['x', 'x', 'x'];
    let mut line3 = ['x', 'z', 'x'];

    //println!("   1 2 3 \n 1 x x x \n 2 x x x  \n 3 x x x ");

    loop {
        runden_score += 1;
        if runden_score %2 == 0 {
            println!("player 1 ist dran")
        }
        else {
            println!("player 2 ist fran")
        }
        println!("gib deine zeile an");// welcher array
        let zeile = userinput_i32();

        println!("gib deine spalte ein");//wo im array
        let spalte = userinput_i32();
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
