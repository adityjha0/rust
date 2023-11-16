use std::io;

fn fib(x:i64)->i64{
    if x==0 || x==1{
        x
    }else{
        fib(x-1)+fib(x-2)
    }
}

fn main(){
    let mut x =String::new();
    io::stdin().read_line(&mut x).expect("couldn't reas line");
    let n:i64 = x.trim().parse().expect("please enter a number");

    println!("The {}th fibonacci is {}",n,fib(n));    
}