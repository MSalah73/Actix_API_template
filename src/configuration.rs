use reqwest;
use serde_aux::field_attributes::deserialize_number_from_string;
// List of valid runtime time environments
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported enviroment. \
                Use either `local` or `production`.",
                other
            )),
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub base_url: String,
}

impl ApplicationSettings {
    pub fn base_url(&self) -> Result<reqwest::Url, String> {
        match reqwest::Url::parse(&self.base_url.clone()) {
            Ok(url) => Ok(url),
            Err(_) => Err(format!("{} is not a valid url.", self.base_url)),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get curretn directory
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");

    // Append 'configuration' to path
    let configuration_directory = base_path.join("configuration");

    // Check app environment value
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Based on the environment variable, append '.yaml'
    let environment_filename = format!("{}.yaml", environment.as_str());

    // Initialize our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named configuration.yaml.
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // This mainaly used for cloud and sensitive data
        .add_source(
            config::Environment::with_prefix("App")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    // Try to convert the configuration values into the Settings type
    settings.try_deserialize::<Settings>()
}
