use shaku::module;
use dotenv::dotenv;
use std::env;

use crate::transport::router::{AppRouterImpl, AppRouterImplParameters};
use crate::transport::app_router::{
    helloworld_router::HelloWorldRouterImpl,
    balance_router::BalanceRouterImpl,
};

module! {
    pub AppModule {
        components = [AppRouterImpl, HelloWorldRouterImpl, BalanceRouterImpl],
        providers = [],
    }
}

pub fn init_app_module() -> AppModule {
    // Load environment variables from .env file if it exists
    dotenv().ok();

    // Read domain from environment variable with a default fallback
    let domain = env::var("SERVER_DOMAIN").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    AppModule::builder()
        .with_component_parameters::<AppRouterImpl>(AppRouterImplParameters {
            domain,
        })
        .build()
}
