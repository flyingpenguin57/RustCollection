struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    len: u64,
    top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> { Self::new_with_len(32) }

    fn new_with_len(len:u64) -> Stack<T> { Stack{len,top: None} }

    fn push(&mut self, element: T) {
        //get old top
        let old_top = *self.top;
        //new node next = top
        let new_node = Node{value: element, next: old_top};
        //top = new node
        self.top = Some(Box::new(new_node));
        //len ++
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        let old_top = self.top;

        match &old_top {
            Some(node) => {
                //top = old_top next
                self.top = node.next;
                self.len -= 1;
            }
            None => {}
        }


        old_top
    }
}

