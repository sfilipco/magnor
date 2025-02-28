use arrow::datatypes::{DataType, Schema as ArrowSchema};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a schema for a dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub arrow_schema: Arc<ArrowSchema>, 
}

/// Represents a partition key for data distribution
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PartitionKey(Vec<u8>);

/// Represents a node in the cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub node_type: NodeType,
}

/// Types of nodes in the cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Master,
    Worker,
}

/// Represents the state of a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Represents a task in the execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub state: TaskState,
    pub dependencies: Vec<String>,
}

impl Schema {
    pub fn new(arrow_schema: Arc<ArrowSchema>) -> Self {
        Self { arrow_schema }
    }

    pub fn fields(&self) -> &arrow::datatypes::Fields {
        self.arrow_schema.fields()
    }
}

impl PartitionKey {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Node {
    pub fn new(id: String, host: String, port: u16, node_type: NodeType) -> Self {
        Self {
            id,
            host,
            port,
            node_type,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
