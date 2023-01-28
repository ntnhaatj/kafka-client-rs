use crate::models::supplied_schema::Payload;
use crate::sampling::{CollectionIterator, SamplingGenerator};
use crate::utils::rand_sample;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

///
/// concrete collection iterator
/// =========
///
/// Payload generator from file path
///
pub struct PayloadSamplingGenerator {
    pub length_of_data: usize,
}

impl SamplingGenerator<Payload> for PayloadSamplingGenerator {
    fn generate(&self) -> Payload {
        Payload {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            data: rand_sample::rand_string(self.length_of_data),
        }
    }
}

pub type PayloadCollectionIterator = CollectionIterator<Payload, dyn SamplingGenerator<Payload>>;

impl Iterator for PayloadCollectionIterator {
    type Item = Payload;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.length {
            return None;
        }
        self.pos += 1;
        let data = self.sampling_generator.generate();
        Some(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::sampling::CollectionIterator;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_generate_collection() {
        let payload_v1_generator = Box::new(PayloadSamplingGenerator { length_of_data: 15 });
        let items = PayloadCollectionIterator {
            _phantom_t: Default::default(),
            sampling_generator: payload_v1_generator,
            pos: 0,
            length: 100,
        };
        assert_eq!(items.length, 100);

        for item in items {
            println!("{:?}", item);
            assert_eq!(item.data.len(), 15);
        }
    }
}
