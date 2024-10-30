// fn main() {
//     let x: i32 = 10; //let x = 10
//     let y= 10;
//     println!("x= {}, y={}", x,y);
// }

// fn main(){
//     let mut x = 1;
//     x = 2;
//     println!("{}",x);
// }

// fn main(){
//     let x= 1.2;
//     let y= x as i32;
//     println!("{} -> {}",x,y);
// }


const THSHRESHOLD: i32 = 18;

fn is_big(n: i32) -> bool {
    n> THSHRESHOLD
}
fn main(){
    println!("{}",THSHRESHOLD);
    println!("{}",is_big(5))
}