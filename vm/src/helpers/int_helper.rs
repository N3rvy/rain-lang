use common::helper::Helper;

pub fn default_int_helper() -> Helper {
    let mut helper = Helper::new();

    helper.register("max", i32::MAX);
    helper.register("min", i32::MIN);

    helper
}