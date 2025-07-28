//! 演示 model2vec-rs 集成的示例程序
//!
//! 这个示例展示了如何使用集成的 potion-multilingual-128M 模型
//! 进行文本嵌入生成。

use ai00_mem::{
    config::Config,
    core::{Memory, MemoryAttributes, MemoryConnections, MemoryMetadata, MemoryType},
    database::VectorGraphDB,
    error::Result,
    memory::MemoryManager,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("AI00-Mem 嵌入向量演示");
    println!("使用 model2vec-rs 和 potion-multilingual-128M 模型\n");

    // 初始化配置
    let mut config = Config::default();
    // 使用内存数据库避免文件权限问题
    config.database.url = "sqlite::memory:".to_string();
    // 设置向量维度为 256
    config.vector.dimension = 256;
    println!("✓ 配置初始化完成，向量维度: {}", config.vector.dimension);
    println!("  数据库类型: 内存数据库");

    // 创建数据库和内存管理器
    let db = Arc::new(VectorGraphDB::new(config.clone()).await?);
    let memory_manager = MemoryManager::new(db, config.clone()).await?;
    println!("✓ 内存管理器创建完成");

    // 测试文本
    let test_texts = vec![
        "今天天气很好，适合出去散步。",
        "我正在学习人工智能和机器学习。",
        "The weather is nice today, perfect for a walk.",
        "I am learning artificial intelligence and machine learning.",
    ];

    println!("\n开始测试内存存储功能...");

    for (i, text) in test_texts.iter().enumerate() {
        println!("\n--- 测试 {} ---", i + 1);
        println!("文本: {}", text);

        // 创建内存对象
        let memory = Memory {
            id: format!("demo_memory_{}", i + 1),
            content: text.to_string(),
            memory_type: MemoryType::Knowledge,
            embedding: Vec::new(), // 将由系统自动生成
            attributes: MemoryAttributes {
                keywords: vec!["演示".to_string(), "测试".to_string()],
                tags: vec!["演示".to_string(), "测试".to_string()],
                context: format!("演示文本 {}", i + 1),
                importance: 0.7,
                emotion: None,
                source: Some("demo".to_string()),
                confidence: 0.9,
                language: Some("zh".to_string()),
                custom_attributes: HashMap::new(),
            },
            connections: MemoryConnections::default(),
            metadata: MemoryMetadata {
                created_at: Utc::now(),
                updated_at: Utc::now(),
                access_count: 0,
                last_accessed: Utc::now(),
                version: 1,
                is_deleted: false,
                deleted_at: None,
                custom_metadata: HashMap::new(),
            },
        };

        // 存储内存（这会触发嵌入向量生成）
        match memory_manager.create_memory(&memory).await {
            Ok(_memory_id) => {
                println!("✓ 内存存储成功");
                println!("  这意味着嵌入向量生成也成功了！");
            }
            Err(e) => {
                println!("✗ 内存存储失败: {}", e);
                println!("  注意: 这可能是由于网络连接问题或模型下载失败");
            }
        }
    }

    println!("\n=== 演示完成 ===");
    println!("\n说明:");
    println!("- 如果看到内存存储成功，说明 model2vec-rs 集成正常工作");
    println!("- potion-multilingual-128M 模型需要从 Hugging Face 下载");
    println!("- 模型支持中文和英文等多种语言");
    println!("- 生成的向量维度为 256，提供更高精度的语义表示");

    Ok(())
}
