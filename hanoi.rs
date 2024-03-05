use std::io;

fn main(){
    let mut x =String::new();
    io::stdin().read_line(&mut x).expect("couldn't read line");
    let n:i32 = x.trim().parse().expect("please enter a number");
    hanoi(n, 1,2,3);
}

fn hanoi(n:i32, s:i32, m:i32, e:i32){
    if n==0 {
        return;
    }else{
        hanoi(n-1, s, e, m);
        println!("{} to {}",s,e);
        hanoi(n-1, m, s, e);
    }
}
