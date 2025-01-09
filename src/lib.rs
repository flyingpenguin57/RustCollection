mod stack;

pub fn add(left: u64, right: u64) -> u64 {
    println!("hello world");
    left + right
}

#[cfg(test)]
mod tests {
    use crate::stack::Stack;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let mut s = Stack::new();

        s.print_top();
        s.push(1);
        s.print_top();
        s.push(2);
        s.print_top();
        s.push(3);
        s.print_top();


        let v1 = s.pop();
        s.print_top();
        let v2 = s.pop();
        s.print_top();
        let v3 = s.pop();
        s.print_top();

        println!("get {}", v1.unwrap().value);
        println!("get {}", v2.unwrap().value);
        println!("get {}", v3.unwrap().value);
    }
}
