mod stack;

pub fn add(left: u64, right: u64) -> u64 {
    println!("hello world");
    left + right
}

#[cfg(test)]
mod tests {
    use std::{panic, result};
    use crate::stack::{Node, Stack};
    use super::*;

    #[test]
    fn stack_test_1() {
        let mut s = Stack::new();

        print_stack_top(&s);
        s.push(1);
        print_stack_top(&s);
        s.push(2);
        print_stack_top(&s);
        s.push(3);
        print_stack_top(&s);

        let v1 = s.pop();
        print_stack_top(&s);
        let v2 = s.pop();
        print_stack_top(&s);
        let v3 = s.pop();
        print_stack_top(&s);
        let v4 = s.pop();
        print_stack_top(&s);

        println!("get {}", v1.unwrap().value);
        println!("get {}", v2.unwrap().value);
        println!("get {}", v3.unwrap().value);
        let result = panic::catch_unwind(|| {
            println!("get {}", v4.expect("haha, it's none").value);
        });
        match result {
            Ok(_) => println!("Function executed successfully."),
            Err(e) => println!("Caught a panic: {:?}", e.downcast_ref::<String>()),
        }
    }

    fn print_stack_top<T: std::fmt::Display>(s: &Stack<T>) {
        match &(s.top) {
            None => {println!("empty stack");},
            Some(node) => {
                println!("{}", node.value);
            }
        }
    }
}
