use std::io;

fn main(){
    let mut x =String::new();
    io::stdin().read_line(&mut x).expect("couldn't reas line");
    let n:i32 = x.trim().parse().expect("please enter a number");
    hanoi(n, 1,2,3);
}

fn hanoi(n:i32, s:i32, m:i32, e:i32){
    if n==1 {
        println!("{} to {}",s,e);
    }else if n==2 {
        println!("{} to {}",s,m);
        println!("{} to {}",s,e);
        println!("{} to {}",m,e);
    }else if n==3 {
        println!("{} to {}",s,e);
        println!("{} to {}",s,m);
        println!("{} to {}",e,m);
        println!("{} to {}",s,e);
        println!("{} to {}",m,s);
        println!("{} to {}",m,e);
        println!("{} to {}",s,e);
    }else{
        hanoi(n-1, s, m, e);
        println!("{} to {}",s,e);
        hanoi(n-1, m, e, s);
    }
}