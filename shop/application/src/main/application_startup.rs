use std::{env, error::Error};

use dotenvy::dotenv;
use log::info;

use crate::main::configuration::{
    telnet_server_configuration::telnet_backend_startup,
    web_api_configuration::rest_backend_startup,
};

#[tokio::main]
pub async fn start_web_backend() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(env::var("LOG_LEVEL")?));
    info!("Log level is set to {:?}", env::var("LOG_LEVEL")?);

    let rest_backend_startup = rest_backend_startup();
    let telnet_backend_startup = telnet_backend_startup();

    rest_backend_startup.await?;
    telnet_backend_startup.await?;
    Ok(())
}
