use std::io;

fn factorial(x:i32)->i32{
    if x<=1{
        x
    }else{
        x*factorial(x-1)
    }
}

fn n_c_r(n:i32,r:i32)->i32{
    factorial(n)/(factorial(r)*factorial(n-r))
}

fn main(){
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1).expect("couldn't read line!");
    let n:i32 = x1.trim().parse().expect("Please enter integer!");

    let mut x2 = String::new();
    io::stdin().read_line(&mut x2).expect("couldn't read line!");
    let r:i32 = x2.trim().parse().expect("Please enter integer!");
    
    let c = n_c_r(n,r);
    println!("{}C{} is {}",n,r,c);
}