#![allow(dead_code)]

pub mod responses;
pub mod route_logger;

use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global vector untuk simpan daftar route
pub static ROUTES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[macro_export]
macro_rules! route_print {
    ($scope:expr, $path:expr, $method:ident, $handler:expr, $base:expr) => {{
        
        let full_path = if $path.is_empty() {
            format!("{}", $base)
        } else {
            format!("{}/{}", $base.trim_end_matches('/'), $path.trim_start_matches('/'))
        };
        
        println!("{} -- {}", stringify!($method).to_uppercase(), full_path);

        // simpan ke global vector
        $crate::helpers::ROUTES
            .lock().unwrap()
            .push(format!("{} {}", stringify!($method).to_uppercase(), full_path));

        $scope.route($path, actix_web::web::$method().to($handler))
    }};
}