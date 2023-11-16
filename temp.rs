use std::io;

fn cel_far(t:f64)->f64{
    t*9.0/5.0 + 32.0
}

fn far_cel(t:f64)->f64{
    (t-32.0)*5.0/9.0
}

fn main(){
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("couldn't read line");
    let a:i8 = x.trim().parse().expect("please enter a number");
    
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("couldn't read line");
    let t:f64 = x.trim().parse().expect("please enter a number");

    if a == 0 {
        println!("{} celsius is {} farhenheit", t, cel_far(t));
    }else if a == 1 {
        println!("{} farhenheit is {} celcius", t, far_cel(t));
    }else {
        println!("Not real!");
    }
}