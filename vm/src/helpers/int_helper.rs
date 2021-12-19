use common::helper::Helper;
use common::external_functions::IntoExternalFunctionRunner;


pub fn default_int_helper() -> Helper {
    let mut helper = Helper::new();

    helper.register("max", i32::MAX);
    helper.register("min", i32::MIN);
    helper.register("add", add.external());
        
    helper
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}