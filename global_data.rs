static mut x:[i32;3] =[0,1,2];

fn main() {
    unsafe{
      println!("Hello, world!->{:?}",x[1]);  
    }
    //println!("Hello, world!->{:?}",x[0]);
}
