#[macro_export]
macro_rules! export_cpy {
    //(0) - Entry point of the macro.
    //      Matches the module name and block of items.
    //      It exports Python related modules only if required,
    //      see the examples.
    //      Each type have a well defined scope, in sections:
    //      (1) - Processing
    //      (2) - Structure
    //      (3) - Python Module Binding
    (mod $module_name:ident { $($item:tt)* }) => {
        export_cpy!(@process_item $($item)*);

        #[cfg(feature = "python")]
        #[pymodule]
        fn $module_name(_py: Python, m: &PyModule) -> PyResult<()> {
            export_cpy!(@add_py_binding m, $($item)*);
            Ok(())
        }
    };

    //(1) - This section defines the processing items patterns
    (@process_item) => {};
    (@process_item enum $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        export_cpy!(@generate_enum $name { $($variant,)* });
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item struct $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        export_cpy!(@generate_struct $name { $($field : $ftype,)* });
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn $name:ident() $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_function $name() $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };

    //(1) - This section defines the structure of the itens
    (@generate_enum $name:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
    (@generate_struct $name:ident { $($field:ident : $ftype:ty,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, set_all))]
        pub struct $name {
            $(
                pub $field: $ftype,
            )*
        }
    };
    (@generate_function $name:ident() $(-> $ret:ty)? $body:block) => {
        #[no_mangle]
        #[cfg_attr(feature = "python", pyfunction)]
        pub extern "C" fn $name() $(-> $ret)? {
            $body
        }
    };

    //(3) - This section defines the bindings to be exported to Python module
    (@add_py_binding $m:ident,) => {};
    (@add_py_binding $m:ident, enum $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, struct $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn $name:ident() $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        $m.add_wrapped(wrap_pyfunction!($name))?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
}
