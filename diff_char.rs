use std::io;

fn main(){
    let mut s1 = String::new();
    let mut s2 = String::new();
    io::stdin().read_line(&mut s1).expect("couldn't read line");
    io::stdin().read_line(&mut s2).expect("couldn't read line");
    let mut char_iter = s1.chars().zip(s2.chars());
    let diff_index = char_iter.position(|(c1, c2)| c1 != c2);
    match diff_index{
        Some(index)=>{
            println!("First different character is at index {:?}", index);
        }
        None=>{
            println!("Identical strings!");
        }
    }

}


