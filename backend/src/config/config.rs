use anyhow::{Context,Result};
use dotenvy::dotenv;
use std::env;

pub struct AppConfig{
    pub database_url:String,
    pub allowed_origin:String,
}

pub fn load_config() -> Result<AppConfig>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .context("DATABASE URL MUST BE SET CORRECTLY")?;

    let allowed_origin = env::var("ALLOWED_ORIGIN")
        .context("ALLOWED_ORIGIN must be set")?;


    Ok(AppConfig{
        database_url,
        allowed_origin,
    })
}
