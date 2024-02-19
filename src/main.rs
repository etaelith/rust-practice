

use chrono::{ Datelike as _, Duration, LocalResult, TimeZone, Utc};


const INIT_REWARD: f64 = 50.0;
const HALVING_NUMBER: u32 = 210_000;
const MAX_BLOCKS:u32 = 21_000_000;

fn main() {
    let mut start_date = Utc.with_ymd_and_hms(2009, 1, 3, 0, 0, 0);

    
    let mut number_block = 0;
    let mut reward = reward_block(number_block);
    while reward > 0.0 && number_block < MAX_BLOCKS {
        let formatted = format!("{}/{}/{}", start_date.unwrap().day(), start_date.unwrap().month(), start_date.unwrap().year());
        println!("Numero de bloque: {}, reward: {} BTC, halving day: ~{:?}", number_block, reward,formatted);
        number_block += HALVING_NUMBER;
        reward = reward_block(number_block);
        let naive_start_date = start_date.unwrap().naive_utc();
        let new_date = naive_start_date + Duration::minutes(10 * HALVING_NUMBER as i64);
        start_date = LocalResult::Single(Utc.from_utc_datetime(&new_date));
    }

}
fn reward_block(num_block:u32) -> f64{
    let halvings = num_block/ HALVING_NUMBER;
    INIT_REWARD/(2.0f64.powi(halvings as i32))
}
/* fn main() {
    let x = 10.0e20;
    println!("x: {x}");
    let y = 153;
    println!("Result: {}", interproduct(120, 100, 248));
    println!("Result convert: {}",divider(x, y))
} */

/* 
fn divider(a: f64, b: i32) -> i32 {
    let a = a as i32;
    return a/b;
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
} */
/* 
fn main() {
   let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[11..15]);
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
    
}
 */

 /* 
 fn main() {
let mut x = 10;
let mut y = 20;
takes_i8(y);
takes_u32(x);
takes_u32(y); === Error 

} 
fn takes_i8(y:i8){
    println!("i8: {y}")
}
fn takes_u32(x:u32){
    println!("u32: {x}")
}*/

/* 
fn main() {
    let n = 5;

    println!("fib(n) = {}", fib(n));

}
fn fib(n: u32) -> u32{
    if n <= 2 {
        return 1;
    } else{
        return fib(n-1) + fib(n-2);
    }
}
 */

 /* 
     let x =20;
    let size = if x <20 {"Small"} else {"Large"};
    println!("Number size: {}", size);
     */