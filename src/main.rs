use std::str::FromStr as _;

use anyhow::{Context, Error};
use log::{LevelFilter, info};
use log4rs::{
    Handle,
    append::{
        console::ConsoleAppender,
        rolling_file::{
            RollingFileAppender,
            policy::compound::{
                CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
            },
        },
    },
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
};

use crate::{config::Config, server::server_main};

mod config;
mod error;
mod server;
mod utils;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    let res = working_main();
    if let Err(e) = res.await {
        // Can't use logging here as we might not have been able to set it up.
        println!("ERROR: {:?}", e)
    }
}

async fn working_main() -> Result<(), Error> {
    println!("Starting up");
    let settings = Config::load().context("Could not load config")?;
    println!("Config loaded. Attempting logging init");

    configure_logging(&settings)?;

    server_main(&settings).await
}

fn configure_logging(config: &Config) -> Result<Handle, Error> {
    let log_path = utils::data_dir().join("logs");

    let log_level = LevelFilter::from_str(&config.log_level)?;

    let window_size = 5;
    let fixed_window_roller = FixedWindowRoller::builder().build(
        log_path
            .join(format!("{}_{{}}.log", APPLICATION_NAME))
            .to_str()
            .unwrap(),
        window_size,
    )?;

    let size_limit = 50 * 1024 * 1024; // 50MB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);

    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let file_logger = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}",
        )))
        .append(true)
        .build(
            log_path.join(format!("{}.log", APPLICATION_NAME)),
            Box::new(compound_policy),
        )?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} | {h({({l}):5.5})} | {f}:{L} — {m}{n}",
        )))
        .build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file_logger)))
        // Turn down some dependencies to be less chatty
        .logger(Logger::builder().build("sled", LevelFilter::Info))
        .build(
            Root::builder()
                .appender("file")
                .appender("stdout")
                .build(log_level),
        )?;

    let handle = log4rs::init_config(config)?;

    info!("Logging configured");
    Ok(handle)
}
