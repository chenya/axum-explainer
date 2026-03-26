use std::net::SocketAddr;

pub struct Config {
    pub bind_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("AXUM_EXPLAINER_APP_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = std::env::var("AXUM_EXPLAINER_APP_PORT")
            .unwrap_or_else(|_| "3001".into())
            .parse::<u16>()
            .expect("invalid PORT — expected a number between 1 and 65535");

        let bind_addr = format!("{host}:{port}")
            .parse()
            .expect("invalid HOST — expected a valid IP address");

        Self { bind_addr }
    }
}
