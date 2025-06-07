use axum::{Router, routing::get};

use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::handler::serve_dir;

#[derive(Clone)]
pub struct ServiceSettings {
    pub(crate) directory: String,
    pub(crate) suffixes: Vec<String>,
    pub(crate) allowed_origins: Vec<String>,
    pub(crate) compression: bool,
    pub(crate) fallback_file: Option<String>,
    pub(crate) append_index_html_on_directories: bool,
}

impl From<ServiceSettings> for Router {
    fn from(value: ServiceSettings) -> Self {
        let service_serve_dir = ServeDir::new(value.directory)
            .append_index_html_on_directories(value.append_index_html_on_directories);

        let service = if let Some(path) = value.fallback_file {
            get(move |request| {
                serve_dir(
                    service_serve_dir.fallback(ServeFile::new(path)),
                    value.suffixes,
                    request,
                )
            })
        } else {
            get(move |request| serve_dir(service_serve_dir, value.suffixes, request))
        };

        let service = if value.compression {
            service.layer(CompressionLayer::new())
        } else {
            service
        };

        let service = if value.allowed_origins.contains(&"*".to_owned()) {
            service.layer(CorsLayer::new().allow_origin(Any))
        } else {
            service.layer(
                CorsLayer::new().allow_origin(
                    value
                        .allowed_origins
                        .iter()
                        .map(|origin| origin.parse().unwrap())
                        .collect::<Vec<_>>(),
                ),
            )
        };

        Router::new().fallback_service(service)
    }
}

pub struct ServerSettings {
    pub(crate) port: u16,
}
