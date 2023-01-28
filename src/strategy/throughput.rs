use crate::sampling::{CollectionIterator, PayloadCollectionIterator, PayloadSamplingGenerator};
use crate::strategy::{Context, RunnableStrategy};
use crate::utils::kafka_clt;
use futures::executor::block_on;
use rand::thread_rng;
use rdkafka::producer::FutureProducer;
use rdkafka::util::Timeout;
use schema_registry_converter::async_impl::avro::AvroEncoder;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;
use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct ThroughputStrategy {
    pub rate_msg_per_sec: u32,
    pub size_in_bytes: u32,
    pub length: u32,
}

impl RunnableStrategy for ThroughputStrategy {
    type CollectionIterator = PayloadCollectionIterator;

    fn get_collection(&self) -> Self::CollectionIterator {
        let sampling_generator = Box::new(PayloadSamplingGenerator {
            length_of_data: self.size_in_bytes as usize,
        });
        PayloadCollectionIterator {
            _phantom_t: Default::default(),
            sampling_generator,
            pos: 0,
            length: self.length,
        }
    }

    ///
    /// run throughput strategy
    /// block on run, spawn thread to handle async produce
    /// # Arguments
    ///
    /// * `producer`: Future Kafka Producer Client
    /// * `topic`:
    /// * `ctx`: CollectionContext
    ///
    /// returns: ()
    ///
    fn run(&self, topic: &str, ctx: &Context) {
        let micro_sleep = (1E6 / (self.rate_msg_per_sec as f64)) as u64;
        let handles = self.get_collection().map(|item| {
            let _ = sleep(Duration::from_micros(micro_sleep));
            let encoded_item = block_on(
                ctx.async_encoder
                    .as_ref()
                    .unwrap()
                    .encode_struct(&item, &ctx.subject_name_strategy.as_ref().unwrap()),
            )
            .unwrap();
            kafka_clt::async_produce(
                &ctx.producer_clt.as_ref().unwrap(),
                &topic,
                encoded_item,
                Timeout::Never,
            )
        });

        block_on(futures::future::join_all(handles.collect::<Vec<_>>()));
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::StrategyType;
    use std::collections::HashMap;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_strategy() -> Result<(), ()> {
        // let st = ThroughputStrategy {
        //     payload: "hell".to_string(),
        //     rate: 0,
        //     size: 0,
        // };
        // let producer = &utils::kafka_clt::get_kk_producer_from_config(&HashMap::from([
        //     ("bootstrap.servers".to_string(), "node-01-on-premise-kafka-nprod.vn.nonprod:9090,node-02-on-premise-kafka-nprod.vn.nonprod:9090,node-03-on-premise-kafka-nprod.vn.nonprod:9090".to_string()),
        //                                                                             ("security.protocol".to_string(), "SASL_SSL".to_string()),
        //                                                                             ("ssl.ca.location".to_string(), "/Users/nhat.nguyen/working/hcvn/certs/hcvn_ca_root.crt".to_string()),
        //                                                                             ("sasl.username".to_string(), "data-business".to_string()),
        //                                                                             ("sasl.password".to_string(), "gKL5btsql9fB".to_string()),
        //                                                                             ("sasl.mechanisms".to_string(), "SCRAM-SHA-256".to_string())]));
        // st.run(&producer, "BDP.VN.DEV.INTERNAL")
        //     .expect("TODO: panic message");
        Ok(())
    }
}
