pub use crate::strategy::base::{Context, RunnableStrategy};
pub use crate::strategy::throughput::ThroughputStrategy;

mod base;
mod throughput;

use crate::models::conf::{StrategyConfig, StrategyType};

pub fn get_strategy(
    strategy_config: &StrategyConfig,
) -> Result<Box<dyn RunnableStrategy<CollectionIterator = impl Iterator>>, &'static str> {
    let strategy = match strategy_config.typ3 {
        StrategyType::Throughput => strategy_config
            .throughput
            .clone()
            .ok_or("throughput config cannot be parsed"),
        _ => return Err("unsupported strategy type"),
    };
    Ok::<_, &str>(Box::new(strategy.unwrap()))
}
