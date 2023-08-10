use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we cant read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
