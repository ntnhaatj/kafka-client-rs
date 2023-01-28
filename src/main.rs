use clap::{Arg, ArgMatches, Command};
use kafka_client_rs::{strategy, utils};

#[tokio::main]
async fn main() {
    let matches: ArgMatches = Command::new("rust kafka client")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .arg(
            Arg::new("config")
                .help("path to confluent cloud config file")
                .long("config")
                .required(true),
        )
        .arg(
            Arg::new("topic")
                .help("test topic to use")
                .long("topic")
                .required(true),
        )
        .get_matches();

    let app_config =
        utils::extract_config_from_command(matches).expect("couldnt parse command arguments");
    let strategy = strategy::get_strategy(&app_config.config_file.strategy).unwrap();
    let ctx = &strategy::Context::new()
        .with_serializer(
            &app_config.config_file.schema_registry_url,
            &app_config.topic,
        )
        .create_producer_clt(&app_config.config_file.librdkafka);
    strategy.run(&app_config.topic, &ctx)
}
