use app::configuration::get_configuration;
use app::startup::Application;
use app::telemetry::{get_subscriber, subscriber_init};
use once_cell::sync::Lazy;

// Zero2Prod
//Given that we never refer to TRACING after its initialization, we could have used std::sync::Once
//with its call_once method. Unfortunately, as soon as the requirements change (i.e. you need to use
//it after initialization), you end up reaching for std::sync::SyncOnceCell, which is not stable yet.
//once_cell covers both usecases - this seemed like a great opportunity to introduce a useful
//crate into your toolkit
//
//TODO: Replace with std::sync::Once when std::sync::SyncOnceCell is stabilized
//
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        subscriber_init(subscriber)
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        subscriber_init(subscriber)
    };
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut configuration = get_configuration().expect("Failed to read configuration");
        configuration.application.port = 0;
        configuration
    };

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let application_port = application.port();

    let address = format!("http://127.0.0.1:{}", application_port);
    let test_app = TestApp {
        address,
        port: application_port,
    };

    let _ = tokio::spawn(application.run_until_stopped());

    test_app
}
