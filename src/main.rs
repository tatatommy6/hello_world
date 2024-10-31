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

// const THSHRESHOLD: i32 = 18;

// fn is_big(n: i32) -> bool {
//     n> THSHRESHOLD
// }
// fn main(){
//     println!("{}",THSHRESHOLD);
//     println!("{}",is_big(5))
// }

// fn swap(num1: i32, num2: i32)-> (i32, i32){
//     (num2,num1)
// }
// fn main(){
//     let (num1,num2)=swap(1,2);
//     println!("{num1},{num2}");
// }

// fn hello(name:String){
//     println!("hello {}",name);
// }
// fn main(){
//     let my_name="buzzi".to_string();

//     {
//         println!("my name is {}",my_name);
//         let my_nme="mellon";
//     }

//     hello(my_name);
// }

// fn main(){
//     let my_num=|x| x+1;
//     println!("{}",my_num(3));
// }

// fn main(){
//     let my_fnc=|mut x:i32|{
//         x+=1;
//         println!("{}",x);
//     };
//     my_fnc(3);
// }


//클로저(||)가 자신을 부를 수 없기 때문에 컴파일되지 않음
// fn fib(n: u32) {
//     let cache = vec![0, 1];
//     let _fib = |n| {
//         if n < cache.len() {
//             cache[n]
//         } else {
//             let result = _fib(n - 1) + _fib(n - 2);  //오류
//             cache.push(result);
//             result
//         }
//     };
//     _fib(n);
// }
// fn main() {
//     println!("{}", fib(10));
// }


// fn main(){
//     let x=1.0;
//     let y=10;
//     let result = if x< (y as f64){  //if 문을 바로 변수에 할당하는 let if 문법 
//         "x is less than y"          //주의: 할당하는 겂들이 모두 같은 타입이여야 함
//     }
//     else if x== (y as f64){
//         "x is equal to y"
//     }
//     else{
//         "x is not less than y"
//     };
//     println!("{}",result);
// }


//for 문
// fn main(){
//     for i in 6..10{   //for i in range(6,10)
//         print!("{} ",i);
//     }

//     let num_range=6..10;
//     for i in num_range{
//         print!("{} ",i);
//     }
// }


// fn main(){
//     for i in 6..=10{  //10 포함
//         print!("{} ",i);
//     }
// }


// fn main(){
//     let mut x=0;
//     while x<5{
//         print!("{} ",x);
//         x+=1;
//     }
// }


// fn main(){
//     let mut x=0;
//     let y = loop{ //무한반복문 (let으로 변수처럼 사용 가능)
//         x+=1;
//         if x==5{
//             break x;
//         }
//         print!("{} ",x);
//     };
//     println!("{} ",y);
// }


// fn main(){
//     let name="jhon";
//     match name{
//         "jhon" => println!("hello, jhon"),
//         "marry" => println!("hello, marry"),
//         _ => println!("hello, stranger"),   //나머지 경우엔 매칭할 값을 생략하는 표기로 _ 사용
//     }
// }
//변수로도 사용가능 (변수일땐 print문 안 씀)

