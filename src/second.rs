use std::mem;

pub struct List<T>{
    head : Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl <T>List<T> {
    pub fn new() -> Self{
        List{head: Link::None}
    }

    pub fn push(&mut self, val: T){

        let new_node = Box::new(Node {
            elem: val,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::None);

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

mod test {  


    #[test]
    fn basics() {
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