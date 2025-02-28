use std::sync::Arc;
use async_trait::async_trait;
use arrow::record_batch::RecordBatch;

use crate::common::{Result, Schema};

/// Represents the execution context for a query
pub struct ExecutionContext {
    pub schema: Arc<Schema>,
}

/// Trait for physical operators
#[async_trait]
pub trait PhysicalOperator: Send + Sync {
    /// Get the schema of the operator's output
    fn schema(&self) -> Arc<Schema>;
    
    /// Execute the operator and produce record batches
    async fn execute(&mut self) -> Result<Vec<RecordBatch>>;
    
    /// Get children operators
    fn children(&self) -> Vec<Arc<dyn PhysicalOperator>>;
}

impl ExecutionContext {
    pub fn new(schema: Arc<Schema>) -> Self {
        Self { schema }
    }
} 