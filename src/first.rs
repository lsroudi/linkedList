use std::mem;

pub struct List{
    head : Link,
}

enum Link{
    Empty,
    More(Box<Node>),
}
struct Node{
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self{
        List{head: Link::Empty}
    }

    pub fn push(&mut self, val: i32){

        let new_node = Box::new(Node {
            elem: val,
            next: mem::replace(&mut self.head,Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            },
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }

        return result;
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

mod test {
    use std::mem;
  

    #[test]
    fn basics() {
        use std::{thread, time};

        let sleep_sec = time::Duration::from_millis(20000);
        let now = time::Instant::now();
        
        
        let mut x = 5;
        let y = 6;

        println!("x ref before mem is : {:p}", &x);
        println!("y ref before mem is : {:p}", &y);
        let val = mem::replace(&mut x,y);
        thread::sleep(sleep_sec);
        println!("val value is : {}", val);
        println!("x value is : {}", x);
        println!("y value is : {}", y);
        println!("val ref is : {:p}", &val);
        println!("x ref is : {:p}", &x);
        println!("y ref is : {:p}", &y);
        use super::List;
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}