mod stack;

pub fn add(left: u64, right: u64) -> u64 {
    println!("hello world");
    left + right
}

struct S1 {
    a: u64,
    b: Option<Box<S1>>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let v1 = S1 {
            a: 1,
            b: Some(Box::new(S1 {a:2,b:None}))
        };

        let v3 = v1.b;



    }
}
