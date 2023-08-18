use cpy_binder::{cpy_enum, cpy_fn, cpy_fn_c, cpy_fn_py, cpy_module, cpy_struct};

#[cpy_enum]
#[comment = "Material types"]
enum Material {
    Plastic,
    Rubber,
}

#[cpy_struct]
#[comment = "2D Size"]
struct Size2D {
    width: f64,
    height: f64,
}

#[cpy_struct]
#[comment = "Tire structure"]
struct Tire {
    material: Material,
    pressure: f64,
    size: Size2D,
}

#[cpy_fn]
#[comment = "Create a random tire"]
fn create_random_tire() -> Tire {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let random_material = if rng.gen_bool(0.5) {
        Material::Plastic
    } else {
        Material::Rubber
    };

    Tire {
        material: random_material,
        pressure: rng.gen_range(30.0..60.0),
        size: Size2D {
            width: rng.gen_range(5.0..10.0),
            height: rng.gen_range(10.0..20.0),
        },
    }
}

#[cpy_fn_c]
#[comment = "Function for C ABI"]
fn format_wheel_identifier_c(dimensions: &[u8; 3]) {
    println!("Wheel identifier: {:?}", dimensions);
}

#[cpy_fn_py]
#[comment = "Format wheel identifier for Python"]
fn format_wheel_identifier_py(dimensions: [u8; 3]) {
    println!("Wheel identifier: {:?}", dimensions);
}

#[cpy_fn_c]
#[comment = "Format size of wheels for C ABI"]
fn format_size_of_wheels_c(sizes: *const u8, length: usize) {
    let values = unsafe {
        assert!(!sizes.is_null());
        std::slice::from_raw_parts(sizes, length)
    };
    println!("Wheel sizes: {:?}", values);
}

#[cpy_fn_py]
#[comment = "Format size of wheels for Python"]
fn format_size_of_wheels_py(sizes: Vec<u8>) {
    println!("Wheel sizes: {:?}", sizes);
}

#[cpy_fn]
#[comment = "Function with no return"]
fn func_with_no_return() {
    println!("Yep, no returns");
}

#[cpy_fn]
#[comment = "Calculate wheel size aspect"]
fn wheel_size_aspect(height: f32, width: f32) -> f32 {
    (height / width) * 100.0
}

cpy_module!(
    name = example,
    types = [Material, Size2D, Tire],
    functions = [
        create_random_tire,
        format_wheel_identifier,
        format_size_of_wheels,
        func_with_no_return,
        wheel_size_aspect
    ]
);
