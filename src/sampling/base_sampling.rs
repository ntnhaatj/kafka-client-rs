use std::marker::PhantomData;

///
/// base iterator to generate serializable data
///
pub struct CollectionIterator<U, T>
where
    T: SamplingGenerator<U> + ?Sized,
{
    pub sampling_generator: Box<T>,
    pub(crate) _phantom_t: PhantomData<U>, // to pass unused U
    pub pos: u32,
    pub length: u32,
}

///
/// trait to impl generator for sampling collection
///
/// # Examples:
/// - to read data from random row from file
/// ```
/// use chrono::Utc;
/// use uuid::Uuid;
/// use kafka_client_rs::sampling::SamplingGenerator;
///
/// impl SamplingGenerator<PayloadV1> for PayloadV1SamplingGenerator {
///     fn generate(&self) -> PayloadV1 {
///         PayloadV1 {
///             id: Uuid::new_v4().to_string(),
///             timestamp: Utc::now().to_rfc3339(),
///             data: utils::rand_row_string_from_file(&self.file_path),
///         }
///     }
/// }
/// ```

pub trait SamplingGenerator<T> {
    fn generate(&self) -> T {
        unimplemented!()
    }
}
