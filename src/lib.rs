//! # AI00-Mem: 智能记忆系统
//!
//! AI00-Mem是一个为个人AI助手设计的智能记忆系统，基于A-Mem和HippoRAG论文的研究成果。
//! 该系统提供动态记忆组织、神经生物学启发的检索机制和持续学习能力。
//!
//! ## 核心特性
//!
//! - **动态记忆组织**: 基于Zettelkasten方法的原子化记忆管理
//! - **智能检索**: HippoRAG风格的个性化PageRank检索算法
//! - **记忆演化**: 自适应的记忆重要性调整和连接更新
//! - **多模式存储**: 向量数据库和图数据库的混合存储
//! - **持续学习**: 基于用户反馈的个性化适应机制
//!
//! ## 快速开始
//!
//! ```rust
//! use ai00_mem::prelude::*;
//! use ai00_mem::config::Config;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() -> ai00_mem::Result<()> {
//!     // 创建配置
//!     let mut config = Config::default();
//!     config.database.url = "sqlite::memory:".to_string();
//!     
//!     // 创建数据库和记忆管理器
//!     let db = std::sync::Arc::new(VectorGraphDB::new(config.clone()).await?);
//!     let memory_manager = MemoryManager::new(db.clone(), config.clone()).await?;
//!     
//!     // 创建记忆
//!     let attributes = MemoryAttributes {
//!         keywords: vec!["机器学习".to_string(), "AI".to_string()],
//!         tags: vec!["学习".to_string()],
//!         importance: 0.8,
//!         ..Default::default()
//!     };
//!     let content = "用户询问了关于机器学习的基础概念".to_string();
//!     let embedding = memory_manager.generate_embedding(&content).await?;
//!     let memory = Memory::new(content, MemoryType::Knowledge, embedding, attributes);
//!     
//!     memory_manager.create_memory(&memory).await?;
//!     
//!     // 检索记忆
//!     let query = Query {
//!         text: "机器学习".to_string(),
//!         query_type: QueryType::Semantic,
//!         filters: QueryFilters::default(),
//!         limit: Some(5),
//!         offset: None,
//!         sort_by: None,
//!         weights: QueryWeights::default(),
//!     };
//!     
//!     let context = Context::default();
//!     let results = memory_manager.retrieve_memories(&query, &context).await?;
//!     println!("找到 {} 条相关记忆", results.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## 模块结构
//!
//! - `core`: 核心数据结构和类型定义
//! - `storage`: 存储层实现，包括向量数据库和图数据库
//! - `memory`: 记忆管理模块，包括创建、链接和演化
//! - `retrieval`: 检索引擎，支持多种检索策略
//! - `learning`: 学习模块，实现持续学习和适应
//! - `utils`: 工具函数和辅助模块
//! - `examples`: 使用示例和演示代码
//!
//! ## 配置
//!
//! 系统支持通过配置文件或环境变量进行配置：
//!
//! ```toml
//! [memory]
//! database_url = "sqlite:memory.db"
//! cache_size = 1000
//!
//! [retrieval]
//! semantic_threshold = 0.7
//! max_results = 50
//!
//! [learning]
//! learning_rate = 0.01
//! decay_rate = 0.001
//! ```

pub mod config;
pub mod core;
pub mod database;
pub mod error;
pub mod learning;
pub mod memory;
pub mod retrieval;
pub mod utils;

#[cfg(feature = "examples")]
pub mod examples;

// 重新导出核心类型
pub use core::*;
pub use database::*;
pub use error::*;
pub use learning::*;
pub use memory::*;
pub use retrieval::*;

/// 预导入模块，包含最常用的类型和函数
pub mod prelude {
    pub use crate::core::*;
    pub use crate::core::{Memory, MemoryAttributes, MemoryConnections, MemoryMetadata};
    pub use crate::core::{Query, QueryFilters, QueryType};
    pub use crate::database::VectorGraphDB;
    pub use crate::error::{MemoryError, Result};
    pub use crate::learning::{LearningEngine, LearningResult};
    pub use crate::memory::MemoryManager;
    pub use crate::utils::*;

    // 重新导出常用的外部类型
    pub use anyhow;
    pub use chrono::{DateTime, Utc};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
    pub use uuid::Uuid;
}

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 库描述
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// 获取库的完整版本信息
pub fn version_info() -> String {
    format!("{} v{} - {}", NAME, VERSION, DESCRIPTION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let info = version_info();
        assert!(info.contains("ai00-mem"));
        assert!(info.contains("0.1.0"));
    }

    #[tokio::test]
    async fn test_basic_functionality() {
        use crate::prelude::*;

        // 测试基本功能是否正常，使用内存数据库
        let mut config = crate::config::Config::default();
        config.database.url = "sqlite::memory:".to_string();
        let result = VectorGraphDB::new(config).await;
        assert!(result.is_ok());
    }
}
