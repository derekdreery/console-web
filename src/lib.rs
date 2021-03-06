#[doc(hidden)]
pub use web_sys as _web_sys;
#[doc(hidden)]
pub use js_sys as _js_sys;

/// This macro works the same as the `println!` from the standard library.
///
/// # Examples
///
/// ```rust,no_run
/// console_web::println!("The number 42 is {}", 42);
/// ```
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::_web_sys::console::log_1(&format!($($args)*).into())
    };
}

/// This macro works the same as the `eprintln!` from the standard library.
///
/// # Examples
///
/// ```rust,no_run
/// console_web::eprintln!("The number 42 is {}", 42);
/// ```
#[macro_export]
macro_rules! eprintln {
    ($($args:tt)*) => {
        $crate::_web_sys::console::error_1(&format!($($args)*).into())
    };
}

/// This macro works like the `console.log` function in javascript.
///
/// # Examples
///
/// ```rust,no_run
/// console_web::log!("a string", 42, true);
/// ```
#[macro_export]
macro_rules! log {
    ($($args:tt,)*) => {
        $crate::log!($($args),*)
    };
    ($($args:tt),*) => {
        let mut arr = $crate::_js_sys::Array::new();
        $(
            arr.push(&$args.into());
        )*
        $crate::_web_sys::console::log(&arr)
    };
}

/// This macro works like the `console.error` function in javascript.
///
/// # Examples
///
/// ```rust,no_run
/// console_web::error!("a string", 42, true);
/// ```
#[macro_export]
macro_rules! error {
    ($($args:tt,)*) => {
        $crate::error!($($args),*)
    };
    ($($args:tt),*) => {
        let mut arr = $crate::_js_sys::Array::new();
        $(
            arr.push(&$args.into());
        )*
        $crate::_web_sys::console::error(&arr)
    };
}
