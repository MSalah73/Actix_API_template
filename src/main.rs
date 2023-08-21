use app::configuration::get_configuration;
use app::startup::Application;
use app::telemetry::{get_subscriber, subscriber_init};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setting up log environment
    let subscriber = get_subscriber("app".into(), "trace".into(), std::io::stdout);
    subscriber_init(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let application = Application::build(configuration).await?;

    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select! {

        o = application_task => {report_exit("API", o)},
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => tracing::info!("{} has exited", task_name),
        Ok(Err(e)) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} failed", task_name)
        }
        Err(e) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} task failed to complate", task_name)
        }
    }
}
