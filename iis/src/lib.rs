#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

use std::env;

pub fn get_port() -> String {
    let port = match env::var("HTTP_PLATFORM_PORT") {
        Ok(val) => val,
        Err(e) => "6767".to_string(),
    };
    port
}
