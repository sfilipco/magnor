use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::{Parser, ParserError};

use crate::common::{MagnorError, Result};

pub struct QueryParser {
    dialect: GenericDialect,
}

impl QueryParser {
    pub fn new() -> Self {
        Self {
            dialect: GenericDialect {},
        }
    }

    pub fn parse(&self, sql: &str) -> Result<Vec<Statement>> {
        Parser::parse_sql(&self.dialect, sql)
            .map_err(|e| MagnorError::QueryParseError(e.to_string()))
    }
}

impl Default for QueryParser {
    fn default() -> Self {
        Self::new()
    }
}
