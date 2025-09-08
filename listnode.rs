#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn main() {
    let v=vec![1,2,3];
    let mut list:Option<Box<ListNode>>=None;
    let mut tail=&mut list;
    
    for val in v{
        let new_node=Some(Box::new(ListNode{val,next:None}));
        *tail=new_node;
        
        if let Some( node) = tail{
            tail=&mut node.next;
        }
    }
    
    println!("{:?}",list);
}
