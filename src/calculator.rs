#[derive(Debug, PartialEq)]
pub enum StackValue {
    Number(f64),
}

impl StackValue {
    // TODO: The UI should choose the formatting
    pub fn as_string(&self) -> String {
        match self {
            StackValue::Number(n) => format!("{}", n),
        }
    }
}

pub struct Calculator {
    stack: Vec<StackValue>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { stack: vec![] }
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(StackValue::Number(value));
    }

    pub fn drop(&mut self) {
        self.stack.pop();
    }

    pub fn stack_iter(&self) -> impl Iterator<Item = &StackValue> {
        self.stack.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty() {
        let calc = Calculator::new();
        assert_eq!(calc.stack_iter().count(), 0);
    }

    #[test]
    fn push_two() {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(2.0);
        let mut iter = calc.stack_iter();
        assert_eq!(&StackValue::Number(2.0), iter.next().unwrap());
        assert_eq!(&StackValue::Number(1.0), iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn push_drop() {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(2.0);
        calc.drop();
        calc.push(3.0);
        let mut iter = calc.stack_iter();
        assert_eq!(&StackValue::Number(3.0), iter.next().unwrap());
        assert_eq!(&StackValue::Number(1.0), iter.next().unwrap());
        assert_eq!(None, iter.next());
    }
}
