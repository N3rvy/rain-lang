use common::{helper::Helper, external_functions::IntoExternalFunctionRunner};

pub fn default_int_helper() -> Helper {
    let mut helper = Helper::new();

    helper.register("max", i32::MAX);
    helper.register("min", i32::MIN);
    helper.register("sqrt", sqrt.external());
    helper.register("power", power.external());

    helper
}

fn sqrt(val: i32) -> i32 {
    (val as f32).sqrt() as i32
}

fn power(val: i32, exp: i32) -> i32 {
    // TODO: Assert value is > 0
    val.pow(exp as u32)
}