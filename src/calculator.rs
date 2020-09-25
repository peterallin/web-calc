pub struct Calculator {
    stack: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { stack: vec![] }
    }

    pub fn push(&mut self, value: String) {
        self.stack.push(value);
    }

    pub fn drop(&mut self) {
        self.stack.pop();
    }

    pub fn stack_iter(&self) -> impl Iterator<Item = &String> {
        self.stack.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn starts_empty() {
        let calc = Calculator::new();
        assert_eq!(calc.stack_iter().count(), 0);
    }

    #[test]
    fn push_two() {
        let mut calc = Calculator::new();
        calc.push("1".into());
        calc.push("2".into());
        let mut iter = calc.stack_iter();
        assert_eq!("2", iter.next().unwrap());
        assert_eq!("1", iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn push_drop() {
        let mut calc = Calculator::new();
        calc.push("1".into());
        calc.push("2".into());
        calc.drop();
        calc.push("3".into());
        let mut iter = calc.stack_iter();
        assert_eq!("3", iter.next().unwrap());
        assert_eq!("1", iter.next().unwrap());
        assert_eq!(None, iter.next());
    }
}
