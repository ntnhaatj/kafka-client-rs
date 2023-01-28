use crate::utils;
use rdkafka::producer::FutureProducer;
use schema_registry_converter::async_impl::avro::AvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::blocking::avro::AvroEncoder as BlockingAvroEncoder;
use schema_registry_converter::schema_registry_common::{SubjectNameStrategy, SuppliedSchema};
use serde::Serialize;
use std::collections::HashMap;

pub trait RunnableStrategy {
    type CollectionIterator;

    /// get collection
    fn get_collection(&self) -> Self::CollectionIterator;

    /// run strategy
    fn run(&self, topic: &str, ctx: &Context);
}

pub struct Context<'a: 'static> {
    pub blocking_encoder: Option<BlockingAvroEncoder>,
    pub async_encoder: Option<AvroEncoder<'a>>,
    pub subject_name_strategy: Option<SubjectNameStrategy>,
    pub producer_clt: Option<FutureProducer>,
}
impl Context<'_> {
    pub fn new() -> Self {
        Context {
            blocking_encoder: None,
            async_encoder: None,
            subject_name_strategy: None,
            producer_clt: None,
        }
    }

    pub fn with_serializer(self, schema_url: &String, topic: &String) -> Self {
        let encoder = AvroEncoder::new(SrSettings::new(schema_url.to_owned()));

        Context {
            async_encoder: Some(encoder),
            subject_name_strategy: Some(SubjectNameStrategy::TopicNameStrategy(
                topic.to_owned(),
                false,
            )),
            ..self
        }
    }
    pub fn create_producer_clt(self, config: &HashMap<String, String>) -> Self {
        Context {
            producer_clt: Some(utils::kafka_clt::get_kk_producer_from_config(&config)),
            ..self
        }
    }
}
