use std::{env, path::PathBuf, time::Duration};

use actix_files::NamedFile;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpRequest, HttpServer, Result, cookie::Key, middleware::Logger, web};
use anyhow::{Context, Error};
use log::{debug, info};

use crate::{config::Config, error::WebError};

pub struct AppData {
    config: Config,
}

pub async fn server_main(settings: &Config) -> Result<(), Error> {
    info!(
        "Server starting up: Port:{} working path: {} exe path: {}",
        settings.server.port,
        env::current_dir().unwrap().display(),
        env::current_exe().unwrap().display(),
    );

    let binding = (settings.server.bind_address.clone(), settings.server.port);

    // should this be using something from config here???
    let secret_key = Key::generate();

    let app_data = AppData {
        config: settings.clone(),
    };

    let web_data = web::Data::new(app_data);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web_data.clone())
            // General file routes. images, css, and javascript
            .route(
                "/img/{filename:.*\\.(jpg|png|webp)}",
                web::get().to(image_host),
            )
            .route("/css/{filename:.*\\.css}", web::get().to(css_host))
            .route("/scripts/{filename:.*\\.js}", web::get().to(script_host))
            // The index page routes
            .route("/index.html", web::get().to(index_page))
            .route("/", web::get().to(index_page))
    })
    .keep_alive(Duration::from_secs(settings.server.keep_alive))
    .workers(settings.server.workers)
    .bind(binding)
    .with_context(|| format!("Could not bind port {}", settings.server.port))?
    .run();

    _ = server.await;

    Ok(())
}

async fn image_host(req: HttpRequest) -> Result<NamedFile, WebError> {
    let root_path: PathBuf =
        get_root_path("static/open/img/").context("could not get root path")?;
    let path: PathBuf = req
        .match_info()
        .query("filename")
        .parse()
        .context("could not parse filename")?;
    let full_path = root_path.join(path);

    debug!("trying to serve image {}", full_path.display());
    Ok(NamedFile::open(full_path).context("could not open file")?)
}

async fn css_host(req: HttpRequest) -> Result<NamedFile, WebError> {
    let root_path: PathBuf =
        get_root_path("static/open/css/").context("could not get root path")?;
    let path: PathBuf = req
        .match_info()
        .query("filename")
        .parse()
        .context("could not parse filename")?;
    let full_path = root_path.join(path);

    debug!("trying to serve css {}", full_path.display());
    Ok(NamedFile::open(full_path).context("could not open file")?)
}

async fn script_host(req: HttpRequest) -> Result<NamedFile, WebError> {
    let root_path: PathBuf =
        get_root_path("static/open/scripts/").context("could not get root path")?;
    let path: PathBuf = req
        .match_info()
        .query("filename")
        .parse()
        .context("could not parse filename")?;
    let full_path = root_path.join(path);

    debug!("trying to serve script {}", full_path.display());
    Ok(NamedFile::open(full_path).context("Could not open file")?)
}

async fn index_page(_req: HttpRequest) -> Result<NamedFile, WebError> {
    let index_page_path: PathBuf =
        get_root_path("static/open/index.html").context("could not get root path")?;
    debug!("trying to serve index page {}", index_page_path.display());
    Ok(NamedFile::open(index_page_path).context("could not open file")?)
}

fn get_root_path(path: &str) -> Result<PathBuf, Error> {
    let sub_path: PathBuf = path
        .parse()
        .with_context(|| format!("could not parse path {}", path))?;
    Ok(env::current_exe()
        .context("Could not get current exe")?
        .parent()
        .context("could not get parent of current exe")?
        .to_path_buf()
        .join(sub_path))
}
