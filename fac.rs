use std::io;

fn main(){
    let mut x =String::new();
    io::stdin().read_line(&mut x).expect("couldn't reas line");
    let n:i32 = x.trim().parse().expect("please enter a number");
    let res=fac(n);
    println!("{:?}",res);
}

fn fac(n:i32)->i32{
    if n==0 {return 1;}
    n*fac(n-1)
}