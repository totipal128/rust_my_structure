use actix_web::{web, Responder, Scope};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// pub static ROUTES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ROUTES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));


pub trait RouterPrintExt {
    fn route_print<F, Fut, R>(self, path: &str, method: &str, handler: F, base: &str) -> Scope
    where
        F: Fn() -> Fut + Clone + 'static,
        Fut: std::future::Future<Output = R> + 'static,
        R: Responder + 'static;
}

impl RouterPrintExt for Scope {
    fn route_print<F, Fut, R>(self, path: &str, method: &str, handler: F, base: &str) -> Scope
    where
        F: Fn() -> Fut + Clone + 'static,
        Fut: std::future::Future<Output = R> + 'static,
        R: Responder + 'static,
    {
        let full_path = if path.is_empty() {
            base.to_string()
        } else {
            format!("{}/{}", base.trim_end_matches('/'), path.trim_start_matches('/'))
        };

        ROUTES.lock().unwrap().push(format!("{} {}", method.to_uppercase(), full_path));
        ROUTES.lock().unwrap().push(format!("ROUTER PUSH DATA"));

        match method.to_lowercase().as_str() {
            "get" => self.route(path, web::get().to(handler)),
            "post" => self.route(path, web::post().to(handler)),
            "put" => self.route(path, web::put().to(handler)),
            "delete" => self.route(path, web::delete().to(handler)),
            _ => self,
        }
    }
}
