#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use reverse::{IntoExternalFunctionRunner, IntoScript};
    use std::{assert_matches::{assert_matches, self}, borrow::Borrow};
    use reverse::{LangValue, Vm};

    #[test]
    fn basic() {
        let script = r#"
        var sum = func(a, b) {
            return a + b
        }
        
        return sum(20, 10)
        "#.to_string().script().unwrap();
        
        let vm = Vm::new();
        let result = vm.evaluate(script).unwrap();
        
        assert_matches!(result, LangValue::Int(30))
    }
    
    #[test]
    fn external() {
        let script = r#"
        return add2(2)
        "#.to_string().script().unwrap();
        
        let vm = Vm::new();
        vm.register("add2", ext_add2.external());
        vm.register("sum", ext_sum.external());
        
        let result = vm.evaluate(script);
        
        assert_matches!(result, Ok(LangValue::Int(4)))
    }
    
    #[test]
    fn helpers() {
        let script = r#"
        var a = 10
        var i = a.max
        return i
        "#.to_string().script().unwrap();
        
        let vm = Vm::new();
        
        let result = vm.evaluate(script);
        
        assert_matches!(result, Ok(LangValue::Int(i32::MAX)));
    }
    
    #[test]
    fn vectors() {
        let script = r#"
        return [10, 0, 10]
        "#.to_string().script().unwrap();
        
        let vm = Vm::new();
        
        let result = vm.evaluate(script).unwrap();
        
        assert!(match result {
            LangValue::Vector(vec)
                => matches!(vec.as_ref()[..], [LangValue::Int(10), LangValue::Int(0), LangValue::Int(10)]),
            _ => false,
        })
    }
    
    fn ext_add2(i: i32) -> i32 {
        i + 2
    }
    
    fn ext_sum(a: i32, b: i32) -> i32 {
        a + b
    }
}