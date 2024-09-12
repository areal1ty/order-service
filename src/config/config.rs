use clap::Parser;
use std::env;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[clap(name = "Order Service")]
pub struct Config {
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    #[clap(short, long, env = "DATABASE_URL")]
    pub database_url: String,

    #[clap(short, long, default_value = "redis://127.0.0.1/", env = "REDIS_URL")]
    pub redis_url: String,

    #[clap(short, long, default_value = "debug", env = "LOG_LEVEL")]
    pub log_level: String,

    /// Таймаут для запросов в секундах
    #[clap(short, long, default_value = "30")]
    pub timeout: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Config::parse()
    }
}
