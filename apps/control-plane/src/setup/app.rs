use std::sync::{Arc, RwLock};

use crate::setup::config::Config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROUTE_REGISTRY: RwLock<RouteRegistry> = RwLock::new(RouteRegistry::new());
}

#[derive(Default, Clone)]
pub struct RouteRegistry {
    routes: Vec<RouteInfo>,
}

impl RouteRegistry {
    fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, method: &str, path: &str) {
        self.routes.push(RouteInfo {
            method: method.to_string(),
            path: path.to_string(),
        });
    }

    pub fn routes(&self) -> Vec<RouteInfo> {
        self.routes.clone()
    }

    pub fn print(&self) {
        let routes = &self.routes;
        if routes.is_empty() {
            tracing::info!("No routes registered");
            return;
        }

        let max_method_len = routes.iter().map(|r| r.method.len()).max().unwrap_or(6);
        let max_path_len = routes.iter().map(|r| r.path.len()).max().unwrap_or(4);

        tracing::info!("");
        tracing::info!("{}", "═".repeat(max_method_len + max_path_len + 7));
        tracing::info!(
            "{:^width$}",
            "Available Routes",
            width = max_method_len + max_path_len + 1
        );
        tracing::info!("{}", "═".repeat(max_method_len + max_path_len + 7));

        for route in routes {
            tracing::info!(
                "  {:<method_width$} {}",
                route.method,
                route.path,
                method_width = max_method_len
            );
        }

        tracing::info!("{}", "═".repeat(max_method_len + max_path_len + 7));
        tracing::info!("");
    }
}

#[derive(Clone, Debug)]
pub struct RouteInfo {
    pub method: String,
    pub path: String,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Option<Arc<storage::DbConnection>>,
}

impl AppState {
    pub fn new(config: Arc<Config>, db: Option<storage::DbConnection>) -> Self {
        Self {
            config,
            db: db.map(Arc::new),
        }
    }
}

pub fn register_routes() {
    let mut registry = ROUTE_REGISTRY.write().unwrap();
    registry.register("GET", "/api/health");
    registry.register("POST", "/api/auth/login");
    registry.register("POST", "/api/auth/register");
    registry.register("GET", "/api/users/me");
    registry.register("GET", "/api-docs/openapi.json");
    drop(registry);
}

pub fn print_routes() {
    let registry = ROUTE_REGISTRY.read().unwrap();
    registry.print();
    drop(registry);
}
