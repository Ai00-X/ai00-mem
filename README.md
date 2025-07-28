# AI00-MEM: Personal AI Assistant Memory System

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://github.com/Ai00-X/ai00-mem)
[![GitHub](https://img.shields.io/badge/github-repo-blue.svg)](https://github.com/Ai00-X/ai00-mem)

**English** | [中文](README_CN.md)

An advanced memory system based on A-Mem and HippoRAG papers, designed for personal AI assistants to provide intelligent memory storage, retrieval, and learning capabilities.

## 🌟 Core Features

### 🧠 Intelligent Memory Management
- **Dynamic Memory Organization**: Memory networks based on Zettelkasten methodology
- **Multi-type Memory**: Support for knowledge, events, tasks, conversations, and more
- **Automatic Connection Discovery**: Intelligent identification of semantic, temporal, and causal relationships between memories
- **Memory Evolution**: Dynamically adjust memory importance based on access patterns and user feedback

### 🔍 Advanced Retrieval Algorithms
- **HippoRAG Retrieval**: Neurobiologically-inspired retrieval algorithm
- **Personalized PageRank**: Personalized ranking based on user preferences
- **Multi-modal Retrieval**: Semantic, structured, temporal, and hybrid retrieval
- **Context Awareness**: Consider user state, device, time, and other contextual information

### 🎯 Adaptive Learning
- **User Preference Learning**: Automatically learn user query and feedback patterns
- **Importance Adjustment**: Dynamically adjust memory importance based on access frequency and feedback
- **Connection Evolution**: Automatically strengthen or weaken connections between memories
- **Pattern Detection**: Identify user behavior and memory access patterns

### ⚡ High-Performance Design
- **Async Processing**: Full async architecture supporting high concurrency
- **Smart Caching**: Multi-layer caching strategy for optimized retrieval performance
- **Batch Processing**: Support for batch operations and parallel processing
- **Memory Optimization**: Efficient vector storage and compression algorithms

### 🔧 Flexible Configuration
- **Database Support**: Focused on SQLite - lightweight yet powerful
- **Modular Design**: Pluggable component architecture
- **Code Configuration**: Support for code-based configuration
- **Extensibility**: Easy to extend new retrieval algorithms and learning strategies

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ai00-mem = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use ai00_mem::{MemoryManager, Config, CreateMemoryRequest, Context, Query};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create configuration
    let mut config = Config::default();
    config.vector.similarity_threshold = 0.7;
    
    // 2. Create database connection
    let db = Arc::new(ai00_mem::database::VectorGraphDB::new(&config).await?);
    
    // 3. Create memory manager
    let memory_manager = MemoryManager::new(db, config).await?;
    
    // 4. Create memory request
    let request = CreateMemoryRequest {
        content: "Rust is a systems programming language focused on safety, concurrency, and performance.".to_string(),
        context: Context {
            session_id: Some("session_001".to_string()),
            current_topic: Some("programming languages".to_string()),
            ..Default::default()
        },
        attributes: None,
        force_connections: None,
    };
    
    // 5. Create memory
    let memory = memory_manager.create_memory_from_request(request).await?;
    println!("Created memory: {}", memory.id);
    
    // 6. Query related memories
    let query = Query {
        text: "Rust programming language".to_string(),
        memory_type: None,
        limit: 10,
        offset: 0,
        sort_by: ai00_mem::core::SortBy::Relevance,
        weights: Default::default(),
    };
    
    let results = memory_manager.retrieve_memories(query).await?;
    println!("Found {} related memories", results.len());
    
    for result in results {
        println!("- {} (relevance: {:.2})", 
            result.memory.content, 
            result.relevance_score);
    }
    
    Ok(())
}
```

### Advanced Features Example

```rust
use ai00_mem::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await?);
    
    // Initialize complete system
    let memory_manager = MemoryManager::new(db.clone(), config.clone()).await?;
    let retriever = HippoRAGRetriever::new(db.clone(), config.clone()).await?;
    let learning_engine = LearningEngine::new(db.clone(), config.clone()).await?;
    
    // Create contextual query
    let query = Query {
        text: "machine learning algorithms".to_string(),
        user_id: "user_123".to_string(),
        context: Some(Context {
            session_id: "session_456".to_string(),
            timestamp: Utc::now(),
            user_state: UserState::Active,
            task_context: Some("learning".to_string()),
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
    
    // Use HippoRAG retrieval
    let results = retriever.hippocampus_retrieval(&query).await?;
    
    // Record user feedback
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
    
    // Run learning cycle
    let learning_results = learning_engine.run_learning_cycle().await?;
    println!("Learning cycle completed, executed {} tasks", learning_results.len());
    
    Ok(())
}
```

## 📖 Detailed Documentation

### Core APIs

#### Memory Manager

- `MemoryManager::new(db: Arc<VectorGraphDB>, config: Config) -> Result<Self>`: Create new memory manager
- `create_memory_from_request(&self, request: CreateMemoryRequest) -> Result<Memory>`: Create new memory from request
- `retrieve_memories(&self, query: Query) -> Result<Vec<RetrievalResult>>`: Retrieve relevant memories
- `update_memory(&self, request: UpdateMemoryRequest) -> Result<Memory>`: Update memory
- `delete_memory(&self, memory_id: &str) -> Result<()>`: Delete memory
- `generate_embedding(&self, text: &str) -> Result<Vec<f32>>`: Generate text embedding vector

#### Request Structures

- `CreateMemoryRequest`: Memory creation request
  - `content: String`: Memory content
  - `context: Context`: Context information
  - `attributes: Option<MemoryAttributes>`: Memory attributes (optional)
  - `force_connections: Option<Vec<MemoryId>>`: Force connection memory IDs (optional)

- `UpdateMemoryRequest`: Memory update request
  - `memory_id: MemoryId`: Memory ID
  - `updates: Vec<UpdateType>`: List of update operations

## 🏗️ Architecture

### Vector-Graph Hybrid Storage
- **Vector Storage**: High-dimensional embeddings for semantic search
- **Graph Storage**: Relationship networks for contextual understanding
- **Hybrid Index**: Combines vector and graph indices for optimal performance

### Learning Engine
- **Online Learning**: Continuous learning from user interactions
- **Reinforcement Learning**: Rewards based on user feedback
- **Graph Neural Networks**: Advanced relationship modeling

### Retrieval Pipeline
1. **Preprocessing**: Query understanding and expansion
2. **Vector Search**: Semantic similarity matching
3. **Graph Traversal**: Relationship-based retrieval
4. **Ranking**: Personalized relevance scoring
5. **Post-processing**: Contextual filtering and re-ranking

## 🔬 Research Background

Based on cutting-edge research:
- **A-Mem**: Adaptive memory systems for AI assistants
- **HippoRAG**: Neurobiologically-inspired retrieval algorithms
- **Zettelkasten**: Knowledge management methodology
- **Personalized PageRank**: User-specific ranking algorithms

## 📊 Performance

- **Memory Capacity**: Supports millions of memories
- **Retrieval Speed**: Sub-100ms for complex queries
- **Learning Efficiency**: Real-time adaptation
- **Resource Usage**: Optimized for personal devices

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [A-Mem](https://arxiv.org/abs/2312.00001) and [HippoRAG](https://arxiv.org/abs/2405.14813)
- Built with [Rust](https://www.rust-lang.org) and [SQLite](https://www.sqlite.org)
- Thanks to the open-source community for inspiration and support