use crate::models::conf::{AppConfig, ConfigFile};
use clap::ArgMatches;
use ron::de::from_reader;
use std::fs::File;

pub fn extract_config_from_command(
    args: ArgMatches,
) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let f = File::open(
        args.get_one::<String>("config")
            .ok_or("error parsing config")?,
    )?;
    let config_file: ConfigFile = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let topic = args
        .get_one::<String>("topic")
        .ok_or("error parsing topic")?
        .to_string();
    Ok(AppConfig { topic, config_file })
}

pub mod kafka_clt {
    use futures::executor::block_on;
    use rdkafka::producer::future_producer::OwnedDeliveryResult;
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::util::Timeout;
    use rdkafka::ClientConfig;
    use std::collections::HashMap;
    use tokio::task::JoinHandle;
    use uuid::Uuid;

    pub fn get_kk_producer_from_config(
        librdkafka_config: &HashMap<String, String>,
    ) -> FutureProducer {
        let mut kafka_config = ClientConfig::new();
        for (k, v) in librdkafka_config {
            kafka_config.set(k.to_owned(), v.to_owned());
        }
        kafka_config.create().expect("could not create producer")
    }

    fn log_produce_result(topic: &str, result: OwnedDeliveryResult) -> Result<(), ()> {
        result
            .and_then(|(p, o)| {
                println!(
                    "Successfully produced record to topic {} partition [{}] @ offset {}",
                    topic, p, o
                );
                Ok(())
            })
            .map_err(|(err, _)| eprintln!("kafka error: {}", err))
    }

    pub fn blocking_produce(
        producer: &FutureProducer,
        topic: &str,
        payload: &Vec<u8>,
        timeout: Timeout,
    ) {
        let uuid = Uuid::new_v4().to_string();
        let record = FutureRecord::to(topic).payload(&payload).key(&uuid);
        let result = block_on(producer.send(record, timeout));
        log_produce_result(&topic, result).expect("TODO: panic message");
    }

    pub fn async_produce(
        producer: &FutureProducer,
        topic: &str,
        payload: Vec<u8>,
        timeout: Timeout,
    ) -> JoinHandle<()> {
        tokio::spawn({
            let cloned_producer = producer.clone();
            let cloned_topic = topic.to_owned();
            let uuid = Uuid::new_v4().to_string();
            async move {
                let record = FutureRecord::to(cloned_topic.as_str())
                    .payload(&payload)
                    .key(&uuid);
                let result = cloned_producer.send(record, timeout).await;
                log_produce_result(&cloned_topic, result).expect("TODO: panic message");
            }
        })
    }
}
pub mod rand_sample {
    use rand::distributions::{Alphanumeric, DistString};

    pub fn rand_string_from_file(path: &str) -> String {
        String::from(format!("hello {}", path.to_owned()))
    }

    pub fn rand_string(length: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_from_file() {
        let input_path = "./config.ron";
        // let f = File::open(&input_path).expect("Failed opening file");
        // let config: ConfigFile = match from_reader(f) {
        //     Ok(x) => x,
        //     Err(e) => {
        //         println!("Failed to load config: {}", e);
        //         std::process::exit(1);
        //     }
        // };
        //
        // println!("Config: {:?}", config.topic);
    }
}
