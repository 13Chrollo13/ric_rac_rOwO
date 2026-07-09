use std::io;


fn main() {
    let mut line1 = ['x', 'x', 'x'];
    let mut line2 = ['x', 'x', 'x'];
    let mut line3 = ['x', 'x', 'x'];
    /*
    let mut line1 = ['x', 'x', 'x'];
    let mut line2 = ["x ", "x ", "x "];
    let mut line3 = ["x ", "x ", "x "];

     */
    //println!("   1 2 3 \n 1 x x x \n 2 x x x  \n 3 x x x ");

    //let line_fn = line1.clone();
    loop {
        let runde = userinput_i32();
        print_field(line1);   //   print_field(line1: Vec<>); das Vec braucht man nich
        break
    }
}
fn print_field(line1: [char; 3]){
    for line1 in &line1{
        println!("{}", line1)
    }
}

fn userinput_i32() -> i32 {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to readline");
    let a = a.trim();
    let userinput: i32 = a.parse().unwrap();
    userinput
}
