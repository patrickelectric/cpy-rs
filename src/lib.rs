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
        #[cfg(feature = "python")]
        use pyo3::{prelude::*, wrap_pyfunction};

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
    (@process_item enum $comment:literal $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        export_cpy!(@generate_enum $comment $name { $($variant,)* });
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item enum $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        export_cpy!(@generate_enum "No documentation" $name { $($variant,)* });
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item struct $comment:literal $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        export_cpy!(@generate_struct $comment $name { $($field : $ftype,)* });
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item struct $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        export_cpy!(@generate_struct "No documentation" $name { $($field : $ftype,)* });
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn $comment:literal $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_function $comment $name($($param : $ptype),*) $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_function "No documentation" $name($($param : $ptype),*) $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn_c $comment:literal $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_c_function $comment $name($($param : $ptype),*) $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn_c $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_c_function "No documentation" $name($($param : $ptype),*) $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn_py $comment:literal $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_py_function $comment $name($($param : $ptype),*) $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };
    (@process_item fn_py $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@generate_py_function "No documentation" $name($($param : $ptype),*) $(-> $ret)? $body);
        export_cpy!(@process_item $($rest)*);
    };

    //(2) - This section defines the structure of the itens
    (@generate_enum $comment:literal $name:ident { $($variant:ident,)* }) => {
        #[doc = $comment]
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
    (@generate_struct $comment:literal $name:ident { $($field:ident : $ftype:ty,)* }) => {
        #[doc = $comment]
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, set_all))]
        pub struct $name {
            $(
                pub $field: $ftype,
            )*
        }
    };
    (@generate_function $comment:literal $name:ident($($arg:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block) => {
        #[doc = $comment]
        #[no_mangle]
        #[cfg_attr(feature = "python", pyfunction)]
        pub extern "C" fn $name($($arg: $arg_type),*) $(-> $ret)?
            $body
    };
    (@generate_c_function $comment:literal $name:ident($($arg:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block) => {
        #[doc = $comment]
        #[no_mangle]
        #[cfg(not(feature = "python"))]
        pub extern "C" fn $name($($arg: $arg_type),*) $(-> $ret)?
            $body
    };
    (@generate_py_function $comment:literal $name:ident($($arg:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block) => {
        #[doc = $comment]
        #[cfg(feature = "python")]
        #[pyfunction]
        pub fn $name($($arg: $arg_type),*) $(-> $ret)?
            $body
    };

    //(3) - This section defines the bindings to be exported to Python module
    (@add_py_binding $m:ident,) => {};
    (@add_py_binding $m:ident, enum $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, enum $comment:literal $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, struct $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, struct $comment:literal $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        $m.add_wrapped(wrap_pyfunction!($name))?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn $comment:literal $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        $m.add_wrapped(wrap_pyfunction!($name))?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn_c $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn_c $comment:literal $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn_py $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        $m.add_wrapped(wrap_pyfunction!($name))?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
    (@add_py_binding $m:ident, fn_py $comment:literal $name:ident($($param:ident : $ptype:ty),*) $(-> $ret:ty)? $body:block $($rest:tt)*) => {
        $m.add_wrapped(wrap_pyfunction!($name))?;
        export_cpy!(@add_py_binding $m, $($rest)*);
    };
}
