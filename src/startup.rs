use crate::configuration::Settings;
use crate::routes::example_fn;
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub reqwest::Url);

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let base_url = configuration
            .application
            .base_url()
            .expect("Invalid application base url");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();

        let server = run(listener, base_url).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(listener: TcpListener, base_url: reqwest::Url) -> Result<Server, anyhow::Error> {
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(example_fn)
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
