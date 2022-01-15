use common::{helper::Helper, external_functions::IntoExternalFunctionRunner};

pub fn default_float_helper() -> Helper {
    let mut helper = Helper::new();

    helper.register("max", f32::MAX);
    helper.register("min", f32::MIN);
    helper.register("sqrt", sqrt.external());
    helper.register("power", power.external());

    helper
}

fn sqrt(val: f32) -> f32 {
    val.sqrt() as f32
}

fn power(val: f32, exp: f32) -> f32 {
    // TODO: Assert value is > 0
    val.powf(exp)
}