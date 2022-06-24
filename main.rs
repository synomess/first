fn main() {
    let mut a: i32=25;
    let mut b:i32=22; 
    let mut ab:i32=abc(a,b); 
    println!("a+b={}",ab);
    while ab!=66 {
      println!("{}",ab);  ab=ab+1}
    }
  
  fn abc(a:i32,b:i32)->i32 {
    a+b
  }