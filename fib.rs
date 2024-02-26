use std::io;

// fn fibrec(x:i64)->i64{
//     if x==0 || x==1{
//         x
//     }else{
//         fibrec(x-1)+fibrec(x-2)
//     }
// }

fn fibdp(x:i64)->i64{
    if x==0 || x==1{
        return x;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    for _i in 2..(x+1) {
        c=a+b;
        a=b;
        b=c;
    }
    c
}

fn main(){
    let mut x =String::new();
    io::stdin().read_line(&mut x).expect("couldn't read line");
    let n:i64 = x.trim().parse().expect("please enter a number");

    println!("The {}th fibonacci is {}",n,fibdp(n));    
}
