#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use std::{assert_matches::assert_matches, sync::Arc};
    use reverse::{LangValue, Scope, IntoExtFunc, evaluate_scope};

    #[test]
    fn basic() {
        let script = r#"
        var sum = func(a, b) {
            return a + b
        }
        
        return sum(20, 10)
        "#;
        
        let result = reverse::evaluate(script.to_string());
        
        assert_matches!(result, Ok(LangValue::Int(30)))
    }
    
    #[test]
    fn external() {
        let script = r#"
        return getnum()
        "#;
        
        let get_10: fn() -> LangValue = || LangValue::Int(10);
        let mut scope = Scope::new(None);
        scope.declare_var("getnum".to_string(), LangValue::ExtFunction(Arc::new(get_10.external_func())));
        
        let result = evaluate_scope(script.to_string(), &mut scope);
        
        assert_matches!(result, Ok(LangValue::Int(10)))
    }
}