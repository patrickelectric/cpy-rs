use rand::Rng;

#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

use cpy_binder::export_cpy;

export_cpy!(
    mod example {
        enum Material {
            Plastic,
            Rubber,
        }

        struct Size2D {
            width: f64,
            height: f64,
        }

        struct Tire {
            material: Material,
            pressure: f64,
            size: Size2D,
        }

        fn create_random_tire() -> Tire {
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

        fn func_with_no_return() {
            println!("Yep, no returns");
        }

        fn wheel_size_aspect(height: f32, width: f32) -> f32 {
            (height / width) * 100.0
        }
    }
);
