#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn rev(mut head: Option<Box<ListNode>>)->Option<Box<ListNode>>{
    let mut prev=None;
    let mut cur=head;
    
    while let Some(mut node)=cur.take(){
        //store next one
        let next_node=node.next.take();
        //point back
        node.next=prev.take();
        //update prev 
        prev=Some(node);
        // move forward
        cur=next_node;
    }
    
    prev
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
    let rev_list=rev(list);
    println!("{:?}",rev_list);
    
}
