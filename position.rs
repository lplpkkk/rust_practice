
fn main() {
    let mut v:Vec<i32>=Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let len=v.len();
    println!("len={:?}",len);

    // be aware that iter() got &i32 type, need the dereference in the closure
    let idx=v.iter().position(|&x| x>2);
    println!("position idx={:?}",idx);    
}
