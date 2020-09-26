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

    pub fn add(&mut self) {
        if let Some(a) = self.stack.pop() {
            if let Some(b) = self.stack.pop() {
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        self.stack.push(StackValue::Number(a + b))
                    }
                }
            } else {
                self.stack.push(a)
            }
        }
    }

    pub fn stack_iter(&self) -> impl Iterator<Item = &StackValue> {
        self.stack.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use matches::assert_matches;

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

    #[test]
    fn add_two() {
        let mut calc = Calculator::new();
        calc.push(100.0);
        calc.push(123.0);
        calc.add();
        let mut iter = calc.stack_iter();
        let top = iter.next().unwrap();
        assert_matches!(top, StackValue::Number(_));
        match top {
            StackValue::Number(x) => assert_approx_eq!(100.0 + 123.0, x),
        }
        assert_eq!(None, iter.next());
    }

    #[test]
    fn add_three() {
        let mut calc = Calculator::new();
        calc.push(777.0);
        calc.push(100.0);
        calc.push(123.0);
        calc.add();
        {
            let mut iter = calc.stack_iter();
            let top = iter.next().unwrap();
            assert_matches!(top, StackValue::Number(_));
            match top {
                StackValue::Number(x) => assert_approx_eq!(100.0 + 123.0, x),
            }

            assert_eq!(&StackValue::Number(777.0), iter.next().unwrap());
        }

        calc.add();
        {
            let mut iter = calc.stack_iter();
            let top = iter.next().unwrap();
            assert_matches!(top, StackValue::Number(_));
            match top {
                StackValue::Number(x) => assert_approx_eq!(777.0 + 100.0 + 123.0, x),
            }
        }
    }
}
