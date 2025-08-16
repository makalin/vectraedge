use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSQL {
    pub statement_type: StatementType,
    pub table_name: Option<String>,
    pub columns: Vec<ColumnDefinition>,
    pub conditions: Vec<Condition>,
    pub vector_operations: Vec<VectorOperation>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatementType {
    Select,
    Create,
    Insert,
    Update,
    Delete,
    CreateIndex,
    DropIndex,
    VectorSearch,
    StreamSubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Boolean,
    Vector(usize), // Vector with dimension
    Timestamp,
    Json,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    PrimaryKey,
    NotNull,
    Unique,
    Default(String),
    ForeignKey(String, String), // (table, column)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: Value,
    pub logical_operator: Option<LogicalOperator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Like,
    In,
    VectorSimilarity, // <-> operator for vector similarity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Vector(Vec<f32>),
    Null,
    FunctionCall(String, Vec<Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorOperation {
    pub operation_type: VectorOperationType,
    pub column: String,
    pub query_vector: Vec<f32>,
    pub distance_metric: DistanceMetric,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorOperationType {
    Search,
    Insert,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    Manhattan,
    DotProduct,
}

pub struct SQLParser {
    // Parser state and configuration
}

impl SQLParser {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn parse(&self, sql: &str) -> Result<ParsedSQL> {
        // This is a simplified parser - in production you'd use a proper SQL parser
        let sql_lower = sql.to_lowercase();
        
        if sql_lower.starts_with("select") {
            self.parse_select(sql)
        } else if sql_lower.starts_with("create") {
            self.parse_create(sql)
        } else if sql_lower.starts_with("insert") {
            self.parse_insert(sql)
        } else if sql_lower.starts_with("update") {
            self.parse_update(sql)
        } else if sql_lower.starts_with("delete") {
            self.parse_delete(sql)
        } else if sql_lower.contains("vector") && sql_lower.contains("search") {
            self.parse_vector_search(sql)
        } else {
            Err(anyhow::anyhow!("Unsupported SQL statement: {}", sql))
        }
    }
    
    fn parse_select(&self, sql: &str) -> Result<ParsedSQL> {
        // Parse SELECT statement
        let mut parsed = ParsedSQL {
            statement_type: StatementType::Select,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        // Extract table name (simplified)
        if let Some(from_index) = sql_lower.find(" from ") {
            if let Some(where_index) = sql_lower.find(" where ") {
                let table_part = &sql[from_index + 6..where_index];
                parsed.table_name = Some(table_part.trim().to_string());
            } else if let Some(limit_index) = sql_lower.find(" limit ") {
                let table_part = &sql[from_index + 6..limit_index];
                parsed.table_name = Some(table_part.trim().to_string());
            } else {
                let table_part = &sql[from_index + 6..];
                parsed.table_name = Some(table_part.trim().to_string());
            }
        }
        
        // Extract LIMIT
        if let Some(limit_index) = sql_lower.find(" limit ") {
            let limit_part = &sql[limit_index + 7..];
            if let Ok(limit_val) = limit_part.trim().parse::<usize>() {
                parsed.limit = Some(limit_val);
            }
        }
        
        // Check for vector operations
        if sql_lower.contains("<->") {
            parsed.vector_operations.push(VectorOperation {
                operation_type: VectorOperationType::Search,
                column: "embedding".to_string(), // Default column
                query_vector: vec![], // Would be extracted from ai_embedding() function
                distance_metric: DistanceMetric::Cosine,
                limit: parsed.limit,
            });
        }
        
        Ok(parsed)
    }
    
    fn parse_create(&self, sql: &str) -> Result<ParsedSQL> {
        let mut parsed = ParsedSQL {
            statement_type: StatementType::Create,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        // Extract table name
        if let Some(table_index) = sql_lower.find(" table ") {
            if let Some(paren_index) = sql.find('(') {
                let table_part = &sql[table_index + 7..paren_index];
                parsed.table_name = Some(table_part.trim().to_string());
            }
        }
        
        // Extract column definitions
        if let Some(paren_start) = sql.find('(') {
            if let Some(paren_end) = sql.rfind(')') {
                let columns_part = &sql[paren_start + 1..paren_end];
                parsed.columns = self.parse_column_definitions(columns_part)?;
            }
        }
        
        Ok(parsed)
    }
    
    fn parse_insert(&self, sql: &str) -> Result<ParsedSQL> {
        let mut parsed = ParsedSQL {
            statement_type: StatementType::Insert,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        // Extract table name
        if let Some(into_index) = sql_lower.find(" into ") {
            if let Some(paren_index) = sql.find('(') {
                let table_part = &sql[into_index + 6..paren_index];
                parsed.table_name = Some(table_part.trim().to_string());
            }
        }
        
        Ok(parsed)
    }
    
    fn parse_update(&self, sql: &str) -> Result<ParsedSQL> {
        let mut parsed = ParsedSQL {
            statement_type: StatementType::Update,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        // Extract table name
        if let Some(set_index) = sql_lower.find(" set ") {
            let table_part = &sql[7..set_index];
            parsed.table_name = Some(table_part.trim().to_string());
        }
        
        Ok(parsed)
    }
    
    fn parse_delete(&self, sql: &str) -> Result<ParsedSQL> {
        let mut parsed = ParsedSQL {
            statement_type: StatementType::Delete,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        // Extract table name
        if let Some(from_index) = sql_lower.find(" from ") {
            if let Some(where_index) = sql_lower.find(" where ") {
                let table_part = &sql[from_index + 6..where_index];
                parsed.table_name = Some(table_part.trim().to_string());
            } else {
                let table_part = &sql[from_index + 6..];
                parsed.table_name = Some(table_part.trim().to_string());
            }
        }
        
        Ok(parsed)
    }
    
    fn parse_vector_search(&self, sql: &str) -> Result<ParsedSQL> {
        let mut parsed = ParsedSQL {
            statement_type: StatementType::VectorSearch,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        // Extract vector search parameters
        if let Some(limit_index) = sql_lower.find(" limit ") {
            let limit_part = &sql[limit_index + 7..];
            if let Ok(limit_val) = limit_part.trim().parse::<usize>() {
                parsed.limit = Some(limit_val);
            }
        }
        
        parsed.vector_operations.push(VectorOperation {
            operation_type: VectorOperationType::Search,
            column: "embedding".to_string(),
            query_vector: vec![],
            distance_metric: DistanceMetric::Cosine,
            limit: parsed.limit,
        });
        
        Ok(parsed)
    }
    
    fn parse_column_definitions(&self, columns_part: &str) -> Result<Vec<ColumnDefinition>> {
        let mut columns = Vec::new();
        let column_parts: Vec<&str> = columns_part.split(',').collect();
        
        for part in column_parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            
            let name = parts[0].to_string();
            let data_type = self.parse_data_type(parts[1])?;
            let constraints = self.parse_constraints(&parts[2..]);
            
            columns.push(ColumnDefinition {
                name,
                data_type,
                constraints,
            });
        }
        
        Ok(columns)
    }
    
    fn parse_data_type(&self, type_str: &str) -> Result<DataType> {
        match type_str.to_lowercase().as_str() {
            "int" | "integer" => Ok(DataType::Integer),
            "float" | "real" => Ok(DataType::Float),
            "text" | "varchar" | "string" => Ok(DataType::Text),
            "bool" | "boolean" => Ok(DataType::Boolean),
            "timestamp" => Ok(DataType::Timestamp),
            "json" => Ok(DataType::Json),
            s if s.starts_with("vector(") && s.ends_with(")") => {
                let dimension_str = &s[7..s.len()-1];
                if let Ok(dimension) = dimension_str.parse::<usize>() {
                    Ok(DataType::Vector(dimension))
                } else {
                    Err(anyhow::anyhow!("Invalid vector dimension: {}", dimension_str))
                }
            }
            _ => Ok(DataType::Custom(type_str.to_string())),
        }
    }
    
    fn parse_constraints(&self, parts: &[&str]) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        
        for part in parts {
            match part.to_lowercase().as_str() {
                "primary" | "primarykey" => constraints.push(Constraint::PrimaryKey),
                "not" | "notnull" => constraints.push(Constraint::NotNull),
                "unique" => constraints.push(Constraint::Unique),
                s if s.starts_with("default(") && s.ends_with(")") => {
                    let default_value = &s[8..s.len()-1];
                    constraints.push(Constraint::Default(default_value.to_string()));
                }
                _ => {}
            }
        }
        
        constraints
    }
    
    pub fn validate_sql(&self, parsed: &ParsedSQL) -> Result<()> {
        match parsed.statement_type {
            StatementType::Create => {
                if parsed.table_name.is_none() {
                    return Err(anyhow::anyhow!("CREATE statement must specify table name"));
                }
                if parsed.columns.is_empty() {
                    return Err(anyhow::anyhow!("CREATE statement must specify at least one column"));
                }
            }
            StatementType::Insert | StatementType::Update | StatementType::Delete => {
                if parsed.table_name.is_none() {
                    return Err(anyhow::anyhow!("Statement must specify table name"));
                }
            }
            StatementType::Select => {
                if parsed.table_name.is_none() {
                    return Err(anyhow::anyhow!("SELECT statement must specify table name"));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    pub fn extract_vector_operations(&self, sql: &str) -> Vec<VectorOperation> {
        let mut operations = Vec::new();
        
        if sql.to_lowercase().contains("<->") {
            operations.push(VectorOperation {
                operation_type: VectorOperationType::Search,
                column: "embedding".to_string(),
                query_vector: vec![],
                distance_metric: DistanceMetric::Cosine,
                limit: None,
            });
        }
        
        if sql.to_lowercase().contains("ai_embedding(") {
            operations.push(VectorOperation {
                operation_type: VectorOperationType::Insert,
                column: "embedding".to_string(),
                query_vector: vec![],
                distance_metric: DistanceMetric::Cosine,
                limit: None,
            });
        }
        
        operations
    }
}

impl Default for SQLParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_select() {
        let parser = SQLParser::new();
        let sql = "SELECT * FROM users WHERE age > 18 LIMIT 10";
        let parsed = parser.parse(sql).unwrap();
        
        assert_eq!(parsed.statement_type, StatementType::Select);
        assert_eq!(parsed.table_name, Some("users".to_string()));
        assert_eq!(parsed.limit, Some(10));
    }
    
    #[test]
    fn test_parse_create() {
        let parser = SQLParser::new();
        let sql = "CREATE TABLE docs (id INT PRIMARY KEY, content TEXT, embedding VECTOR(384))";
        let parsed = parser.parse(sql).unwrap();
        
        assert_eq!(parsed.statement_type, StatementType::Create);
        assert_eq!(parsed.table_name, Some("docs".to_string()));
        assert_eq!(parsed.columns.len(), 3);
        
        let vector_col = parsed.columns.iter().find(|c| c.name == "embedding").unwrap();
        match &vector_col.data_type {
            DataType::Vector(dim) => assert_eq!(*dim, 384),
            _ => panic!("Expected Vector data type"),
        }
    }
    
    #[test]
    fn test_parse_vector_search() {
        let parser = SQLParser::new();
        let sql = "SELECT * FROM docs ORDER BY embedding <-> ai_embedding('query') LIMIT 5";
        let parsed = parser.parse(sql).unwrap();
        
        assert_eq!(parsed.statement_type, StatementType::Select);
        assert!(!parsed.vector_operations.is_empty());
        assert_eq!(parsed.limit, Some(5));
    }
    
    #[test]
    fn test_validate_sql() {
        let parser = SQLParser::new();
        let mut parsed = ParsedSQL {
            statement_type: StatementType::Create,
            table_name: None,
            columns: vec![],
            conditions: vec![],
            vector_operations: vec![],
            limit: None,
            offset: None,
        };
        
        assert!(parser.validate_sql(&parsed).is_err());
        
        parsed.table_name = Some("test".to_string());
        parsed.columns.push(ColumnDefinition {
            name: "id".to_string(),
            data_type: DataType::Integer,
            constraints: vec![],
        });
        
        assert!(parser.validate_sql(&parsed).is_ok());
    }
}
