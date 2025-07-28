# AI00-MEM: 个人AI助手记忆系统

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://github.com/Ai00-X/ai00-mem)
[![GitHub](https://img.shields.io/badge/github-repo-blue.svg)](https://github.com/Ai00-X/ai00-mem)

一个基于A-Mem和HippoRAG论文设计的高级记忆系统，为个人AI助手提供智能的记忆存储、检索和学习能力。

## 🌟 核心特性

### 🧠 智能记忆管理
- **动态记忆组织**: 基于Zettelkasten方法的记忆网络
- **多类型记忆**: 支持知识、事件、任务、对话等多种记忆类型
- **自动连接发现**: 智能识别记忆间的语义、时间、因果关系
- **记忆演化**: 根据访问模式和用户反馈动态调整记忆重要性

### 🔍 先进检索算法
- **HippoRAG检索**: 神经生物学启发的检索算法
- **个性化PageRank**: 基于用户偏好的个性化排序
- **多模态检索**: 语义、结构化、时间、混合检索
- **上下文感知**: 考虑用户状态、设备、时间等上下文信息

### 🎯 自适应学习
- **用户偏好学习**: 自动学习用户的查询和反馈模式
- **重要性调整**: 基于访问频率和反馈动态调整记忆重要性
- **连接演化**: 自动强化或弱化记忆间的连接
- **模式检测**: 识别用户行为和记忆访问模式

### ⚡ 高性能设计
- **异步处理**: 全异步架构，支持高并发
- **智能缓存**: 多层缓存策略，优化检索性能
- **批处理**: 支持批量操作和并行处理
- **内存优化**: 高效的向量存储和压缩算法

### 🌍 多语言向量嵌入
- **内置多语言支持**: 原生支持中文、英文、日文、韩文等多种语言
- **Model2Vec集成**: 利用model2vec-rs实现高效的多语言嵌入
- **跨语言检索**: 无缝跨语言搜索记忆内容
- **语言感知处理**: 自动语言检测和优化编码
- **轻量级模型**: 为资源受限环境优化的嵌入模型

### 🔧 灵活配置
- **数据库支持**: 专注于SQLite，轻量级且功能强大
- **模块化设计**: 可插拔的组件架构
- **代码配置**: 支持代码方式配置
- **扩展性**: 易于扩展新的检索算法和学习策略

## 🚀 快速开始

### 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ai00-mem = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 基本使用

```rust
use ai00_mem::{MemoryManager, Config, CreateMemoryRequest, Context, Query};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建配置
    let mut config = Config::default();
    config.vector.similarity_threshold = 0.7;
    
    // 2. 创建数据库连接
    let db = Arc::new(ai00_mem::database::VectorGraphDB::new(&config).await?);
    
    // 3. 创建记忆管理器
    let memory_manager = MemoryManager::new(db, config).await?;
    
    // 4. 创建记忆请求
    let request = CreateMemoryRequest {
        content: "Rust是一种系统编程语言，注重安全性、并发性和性能。".to_string(),
        context: Context {
            session_id: Some("session_001".to_string()),
            current_topic: Some("编程语言".to_string()),
            ..Default::default()
        },
        attributes: None,
        force_connections: None,
    };
    
    // 5. 创建记忆
    let memory = memory_manager.create_memory_from_request(request).await?;
    println!("创建记忆: {}", memory.id);
    
    // 6. 查询相关记忆
    let query = Query {
        text: "Rust编程语言".to_string(),
        memory_type: None,
        limit: 10,
        offset: 0,
        sort_by: ai00_mem::core::SortBy::Relevance,
        weights: Default::default(),
    };
    
    let results = memory_manager.retrieve_memories(query).await?;
    println!("找到 {} 个相关记忆", results.len());
    
    for result in results {
        println!("- {} (相关性: {:.2})", 
            result.memory.content, 
            result.relevance_score);
    }
    
    Ok(())
}
```

### 高级功能示例

```rust
use ai00_mem::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await?);
    
    // 初始化完整系统
    let memory_manager = MemoryManager::new(db.clone(), config.clone()).await?;
    let retriever = HippoRAGRetriever::new(db.clone(), config.clone()).await?;
    let learning_engine = LearningEngine::new(db.clone(), config.clone()).await?;
    
    // 创建带上下文的查询
    let query = Query {
        text: "机器学习算法".to_string(),
        user_id: "user_123".to_string(),
        context: Some(Context {
            session_id: "session_456".to_string(),
            timestamp: Utc::now(),
            user_state: UserState::Active,
            task_context: Some("学习".to_string()),
            emotional_state: Some(EmotionalState::Curious),
            attention_level: Some(AttentionLevel::High),
            ..Default::default()
        }),
        filters: vec![
            Filter::MemoryType(MemoryType::Knowledge),
            Filter::TimeRange(Utc::now() - Duration::days(30), Utc::now()),
        ],
        max_results: 5,
        min_relevance: 0.6,
    };
    
    // 使用HippoRAG检索
    let results = retriever.hippocampus_retrieval(&query).await?;
    
    // 记录用户反馈
    let feedback = FeedbackRecord {
        memory_id: results[0].memory.id.clone(),
        user_id: "user_123".to_string(),
        feedback_type: FeedbackType::Explicit,
        score: 0.9,
        context: FeedbackContext {
            query: query.text.clone(),
            result_position: 0,
            session_id: "session_456".to_string(),
            device_type: Some("desktop".to_string()),
            time_of_day: 14,
            day_of_week: 1,
        },
        timestamp: Utc::now(),
    };
    
    learning_engine.record_feedback(feedback).await?;
    
    // 运行学习循环
    let learning_results = learning_engine.run_learning_cycle().await?;
    println!("学习循环完成，执行了 {} 个任务", learning_results.len());
    
    Ok(())
}
```

## 📖 详细文档

### 核心API

#### 记忆管理器

- `MemoryManager::new(db: Arc<VectorGraphDB>, config: Config) -> Result<Self>`: 创建新的记忆管理器
- `create_memory_from_request(&self, request: CreateMemoryRequest) -> Result<Memory>`: 从请求创建新记忆
- `retrieve_memories(&self, query: Query) -> Result<Vec<RetrievalResult>>`: 检索相关记忆
- `update_memory(&self, request: UpdateMemoryRequest) -> Result<Memory>`: 更新记忆
- `delete_memory(&self, memory_id: &str) -> Result<()>:`: 删除记忆
- `generate_embedding(&self, text: &str) -> Result<Vec<f32>>`: 生成文本嵌入向量

#### 请求结构

- `CreateMemoryRequest`: 记忆创建请求
  - `content: String`: 记忆内容
  - `context: Context`: 上下文信息
  - `attributes: Option<MemoryAttributes>`: 记忆属性（可选）
  - `force_connections: Option<Vec<MemoryId>>`: 强制连接的记忆ID（可选）

- `UpdateMemoryRequest`: 记忆更新请求
  - `memory_id: MemoryId`: 记忆ID
  - `updates: Vec<UpdateType>`: 更新操作列表

## 🙏 致谢

- 灵感来源于[A-Mem](https://arxiv.org/abs/2312.00001)和[HippoRAG](https://arxiv.org/abs/2405.14813)
- 基于[Rust](https://www.rust-lang.org)和[SQLite](https://www.sqlite.org)构建
- 多语言嵌入由[model2vec-rs](https://github.com/anonymous-ai/model2vec-rs)提供支持
- 感谢开源社区的启发和支持