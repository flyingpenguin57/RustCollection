pub struct Node<T> {
    pub value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    len: u64,
    pub top: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> Stack<T> {
    pub fn new() -> Stack<T> { Stack{len: 0,top: None} }

    pub fn push(&mut self, element: T) {
        //get old top
        let old_top = self.top.take();
        //create new top, make its next = old top
        let new_top = Node{value: element, next: old_top};
        //top = new node
        self.top = Some(Box::new(new_top));
        //len ++
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<Box<Node<T>>> {
        let old_top = self.top.take();

        match old_top {
            Some(mut node) => {
                //top = old_top next
                self.top = node.next.take();
                self.len -= 1;
                Some(node)
            }
            None => {None}
        }
    }
}

