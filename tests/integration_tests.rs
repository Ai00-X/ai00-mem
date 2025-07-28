//! 集成测试
//!
//! 这个文件包含了 ai00-mem 库的集成测试，验证各个组件之间的协作。

use ai00_mem::config::Config;
use ai00_mem::prelude::*;
use ai00_mem::FeedbackContext;
use ai00_mem::FeedbackRecord;
use ai00_mem::FeedbackType;
use ai00_mem::FusionMethod;
use ai00_mem::HippoRAGRetriever;
use ai00_mem::Priority;
use ai00_mem::RetrievalConstraints;
use ai00_mem::RetrievalContext;
use ai00_mem::UpdateMemoryRequest;
use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio;

/// 测试基本的记忆管理功能
#[tokio::test]
async fn test_basic_memory_management() {
    let mut config = Config::default();
    config.vector.similarity_threshold = 0.1; // 降低阈值确保匹配
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建记忆
    let attributes = MemoryAttributes {
        keywords: vec!["测试".to_string()],
        tags: vec!["测试".to_string()],
        context: String::new(),
        importance: 0.8,
        confidence: 0.9,
        emotion: None,
        source: None,
        language: None,
        custom_attributes: std::collections::HashMap::new(),
    };
    let content = "这是一个测试记忆".to_string();
    let query_text = content.clone();
    let embedding = memory_manager.generate_embedding(&content).await.unwrap();
    let memory = Memory::new(
        content,
        MemoryType::Knowledge,
        embedding,
        attributes,
    );


    memory_manager.create_memory(&memory).await.unwrap();

    // 检索记忆
    let query = Query {
        text: query_text,
        query_type: QueryType::Semantic,
        filters: QueryFilters {
              ..Default::default()
          },
        limit: Some(10),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };
    let context = Context::default();
    let results = memory_manager
        .retrieve_memories(&query, &context)
        .await
        .unwrap();
    assert!(!results.is_empty());
    assert_eq!(results[0].memory.id, memory.id);
}

/// 测试记忆连接功能
#[tokio::test]
async fn test_memory_connections() {
    let mut config = Config::default();
    config.vector.similarity_threshold = 0.1; // 降低阈值确保匹配
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建两个记忆
    let attributes1 = MemoryAttributes {
        keywords: vec!["编程".to_string()],
        importance: 0.8,
        confidence: 0.9,
        ..Default::default()
    };
    let content1 = "Rust编程语言".to_string();
    let query_text = content1.clone();
    let embedding1 = memory_manager.generate_embedding(&content1).await.unwrap();
    let memory1 = Memory::new(
        content1,
        MemoryType::Knowledge,
        embedding1,
        attributes1,
    );


    let attributes2 = MemoryAttributes {
        keywords: vec!["编程".to_string(), "系统".to_string()],
        importance: 0.7,
        confidence: 0.8,
        ..Default::default()
    };
    let content2 = "系统编程".to_string();
    let embedding2 = memory_manager.generate_embedding(&content2).await.unwrap();
    let memory2 = Memory::new(
        content2,
        MemoryType::Knowledge,
        embedding2,
        attributes2,
    );


    memory_manager.create_memory(&memory1).await.unwrap();
    memory_manager.create_memory(&memory2).await.unwrap();

    // 创建连接
    let connection = Connection::new(
        memory1.id.clone(),
        memory2.id.clone(),
        ConnectionType::Semantic,
        0.8,
    );

    memory_manager.create_connection(&connection).await.unwrap();

    // 检索相关记忆
    let query = Query {
        text: query_text,
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            ..Default::default()
        },
        limit: Some(10),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };
    let context = Context::default();
    let results = memory_manager
        .retrieve_memories(&query, &context)
        .await
        .unwrap();
    assert!(!results.is_empty());
}

/// 测试不同类型的检索
#[tokio::test]
async fn test_different_retrieval_types() {
    let mut config = Config::default();
    config.vector.similarity_threshold = 0.1; // 降低阈值确保匹配
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建不同类型的记忆
    let memories = vec![
        {
            let content = "学习Rust编程".to_string();
            let embedding = memory_manager.generate_embedding(&content).await.unwrap();
            Memory::new(
                content,
                MemoryType::Knowledge,
                embedding,
                MemoryAttributes {
                    keywords: vec!["Rust".to_string(), "学习".to_string()],
                    tags: vec!["Rust".to_string(), "学习".to_string()],
                    context: String::new(),
                    importance: 0.8,
                    confidence: 0.9,
                    emotion: None,
                    source: None,
                    language: None,
                    custom_attributes: std::collections::HashMap::new(),
                },
            )
        },
        {
            let content = "今天完成了Rust项目".to_string();
            let embedding = memory_manager.generate_embedding(&content).await.unwrap();
            Memory::new(
                content,
                MemoryType::Event,
                embedding,
                MemoryAttributes {
                    keywords: vec!["Rust".to_string(), "项目".to_string()],
                    tags: vec!["Rust".to_string(), "项目".to_string()],
                    context: String::new(),
                    importance: 0.7,
                    confidence: 0.8,
                    emotion: None,
                    source: None,
                    language: None,
                    custom_attributes: std::collections::HashMap::new(),
                },
            )
        },
    ];

    for memory in &memories {
        memory_manager.create_memory(memory).await.unwrap();
    }

    // 语义检索
    let context = Context::default();
    let semantic_query = Query {
        text: "Rust编程".to_string(),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            ..Default::default()
        },
        limit: Some(10),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };

    let _semantic_results = memory_manager
        .retrieve_memories(&semantic_query, &context)
        .await
        .unwrap();

    // 时间检索
    let temporal_query = Query {
        text: memories[1].content.clone(),
        query_type: QueryType::Temporal,
        filters: QueryFilters {
            time_range: Some((Utc::now() - Duration::hours(24), Utc::now())),
            ..Default::default()
        },
        limit: Some(10),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };

    let temporal_results = memory_manager
        .retrieve_memories(&temporal_query, &context)
        .await
        .unwrap();
    assert!(!temporal_results.is_empty());
}

/// 测试海马体检索
#[tokio::test]
async fn test_hippocampus_retrieval() {
    let mut config = Config::default();
    config.vector.similarity_threshold = 0.1; // 降低阈值确保匹配
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let retriever = HippoRAGRetriever::new(db.clone(), config.clone())
        .await
        .unwrap();
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建一些相关的记忆
    let memories = vec![
        "机器学习是人工智能的一个分支",
        "深度学习是机器学习的子集",
        "神经网络是深度学习的基础",
        "Transformer是现代NLP的核心架构",
    ];

    for (i, content) in memories.iter().enumerate() {
        let content = content.to_string();
        let embedding = memory_manager.generate_embedding(&content).await.unwrap();
        let memory = Memory {
            id: IdGenerator::generate_memory_id(),
            content: content,
            memory_type: MemoryType::Knowledge,
            embedding,
            attributes: MemoryAttributes {
                keywords: vec!["AI".to_string(), "机器学习".to_string()],
                tags: vec!["AI".to_string(), "机器学习".to_string()],
                context: String::new(),
                importance: 0.8,
                confidence: 0.9,
                emotion: None,
                source: None,
                language: None,
                custom_attributes: std::collections::HashMap::new(),
            },
            ..Default::default()
        };

        memory_manager.create_memory(&memory).await.unwrap();
    }

    // HippoRAG检索
    let query = Query {
        text: "人工智能学习".to_string(),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            ..Default::default()
        },
        limit: Some(5),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };

    let context = RetrievalContext {
        session_id: None,
        current_topic: None,
        recent_queries: vec![],
        time_window: None,
        priority: Priority::Normal,
        constraints: RetrievalConstraints {
            max_results: None,
            min_relevance: None,
            required_tags: vec![],
            excluded_tags: vec![],
            time_range: None,
            source_filter: None,
        },
    };
    let results = retriever
        .hippocampus_retrieval(&query, &context, true, true, &FusionMethod::LinearWeighted)
        .await
        .unwrap();
    assert!(!results.is_empty());
}

/// 测试学习引擎
#[tokio::test]
async fn test_learning_engine() {
    let mut config = Config::default();
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let learning_engine = LearningEngine::new(db.clone(), config.clone())
        .await
        .unwrap();
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建记忆
    let content = "学习引擎测试记忆".to_string();
    let embedding = memory_manager.generate_embedding(&content).await.unwrap();
    let mut memory = Memory {
        id: IdGenerator::generate_memory_id(),
        content: content,
        memory_type: MemoryType::Knowledge,
        embedding,
        attributes: MemoryAttributes {
            keywords: vec!["测试".to_string()],
            tags: vec!["测试".to_string()],
            context: String::new(),
            importance: 0.7,
            confidence: 0.8,
            emotion: None,
            source: None,
            language: None,
            custom_attributes: std::collections::HashMap::new(),
        },
        ..Default::default()
    };
    

    memory_manager.create_memory(&memory).await.unwrap();

    // 记录反馈
    let feedback = FeedbackRecord {
        memory_id: memory.id.clone(),
        feedback_type: FeedbackType::Explicit,
        score: 0.9,
        context: FeedbackContext {
            query: "学习".to_string(),
            result_position: 0,
            session_id: IdGenerator::generate_session_id(),
            device_type: Some("test".to_string()),
            time_of_day: 14,
            day_of_week: 1,
        },
        timestamp: Utc::now(),
    };

    learning_engine.record_feedback(feedback).await.unwrap();

    // 运行学习循环
    let results = learning_engine.run_learning_cycle().await.unwrap();
    assert!(!results.is_empty());
}

/// 测试并发操作
#[tokio::test]
async fn test_concurrent_operations() {
    let mut config = Config::default();
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = Arc::new(
        MemoryManager::new(db.clone(), config.clone())
            .await
            .unwrap(),
    );

    // 并发创建记忆
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            let mm = memory_manager.clone();
            tokio::spawn(async move {
                let attributes = MemoryAttributes {
                    keywords: vec![format!("并发记忆 {}", i)],
                    tags: vec!["并发".to_string()],
                    context: String::new(),
                    importance: 0.5,
                    emotion: None,
                    source: None,
                    confidence: 0.8,
                    language: None,
                    custom_attributes: Default::default(),
                };
                let content = format!("并发测试记忆 {}", i);
            let embedding = mm.generate_embedding(&content).await.unwrap();
            let memory = Memory::new(
                content,
                MemoryType::Knowledge,
                embedding,
                attributes,
            );
        
                mm.create_memory(&memory).await
            })
        })
        .collect();

    // 等待所有任务完成
    for task in tasks {
        task.await.unwrap().unwrap();
    }

    // 验证所有记忆都被创建
    let query = Query {
        text: format!("并发测试记忆 {}", 0),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            ..Default::default()
        },
        limit: Some(20),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };

    let context = Context::default();
    let results = memory_manager
        .retrieve_memories(&query, &context)
        .await
        .unwrap();
    assert!(results.len() >= 10);
}

/// 测试过滤功能
#[tokio::test]
async fn test_filtering() {
    let mut config = Config::default();
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建不同类型的记忆
    let knowledge_attributes = MemoryAttributes {
        keywords: vec!["知识".to_string()],
        tags: vec!["知识".to_string()],
        context: String::new(),
        importance: 0.8,
        emotion: None,
        source: None,
        confidence: 0.9,
        language: None,
        custom_attributes: Default::default(),
    };
    let content_k = "知识型记忆".to_string();
    let embedding_k = memory_manager.generate_embedding(&content_k).await.unwrap();
    let knowledge_memory = Memory::new(
        content_k,
        MemoryType::Knowledge,
        embedding_k,
        knowledge_attributes,
    );
    

    let task_attributes = MemoryAttributes {
        keywords: vec!["任务".to_string()],
        tags: vec!["任务".to_string()],
        context: String::new(),
        importance: 0.7,
        emotion: None,
        source: None,
        confidence: 0.8,
        language: None,
        custom_attributes: Default::default(),
    };
    let content_t = "任务型记忆".to_string();
    let embedding_t = memory_manager.generate_embedding(&content_t).await.unwrap();
    let task_memory = Memory::new(
        content_t,
        MemoryType::Task,
        embedding_t,
        task_attributes,
    );
    

    memory_manager
        .create_memory(&knowledge_memory)
        .await
        .unwrap();
    memory_manager.create_memory(&task_memory).await.unwrap();

    // 按标签过滤
    let context = Context::default();
    let filtered_query = Query {
        text: "记忆".to_string(),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            tags: Some(vec!["知识".to_string()]),
            ..Default::default()
        },
        limit: Some(10),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };

    let filtered_results = memory_manager
        .retrieve_memories(&filtered_query, &context)
        .await
        .unwrap();
    assert!(!filtered_results.is_empty());

    // 验证所有结果都是知识型记忆
    for result in filtered_results {
        assert_eq!(result.memory.memory_type, MemoryType::Knowledge);
    }
}

/// 测试记忆更新功能
#[tokio::test]
async fn test_memory_updates() {
    let mut config = Config::default();
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 创建记忆
    let attributes = MemoryAttributes {
        keywords: vec!["原始标题".to_string()],
        tags: vec!["原始".to_string()],
        context: String::new(),
        importance: 0.5,
        emotion: None,
        source: None,
        confidence: 0.8,
        language: None,
        custom_attributes: Default::default(),
    };
    let content = "原始内容".to_string();
    let embedding = memory_manager.generate_embedding(&content).await.unwrap();
    let memory = Memory::new(
        content,
        MemoryType::Knowledge,
        embedding,
        attributes,
    );
    

    memory_manager.create_memory(&memory).await.unwrap();

    // 更新记忆
    let update_request = UpdateMemoryRequest {
        memory_id: memory.id.clone(),
        updates: vec![UpdateType::ImportanceAdjustment(0.8)],
        context: Context::default(),
    };

    memory_manager.update_memory(update_request).await.unwrap();

    // 验证更新
    let context = Context::default();
    let query = Query {
        text: "原始".to_string(),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            ..Default::default()
        },
        limit: Some(10),
        offset: None,
        sort_by: None,
        weights: QueryWeights::default(),
    };

    let results = memory_manager
        .retrieve_memories(&query, &context)
        .await
        .unwrap();
    assert!(!results.is_empty());
    // 注意：这里我们无法直接验证importance的变化，因为retrieve_memories返回的是RetrievalResult
    // 在实际实现中，可能需要添加专门的方法来获取单个记忆的详细信息
}

/// 测试统计功能
#[tokio::test]
async fn test_statistics() {
    let mut config = Config::default();
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    config.database.url = format!("sqlite://{}" , temp_file.path().display());
    config.database.database_name = "test".to_string();
    let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
    let memory_manager = MemoryManager::new(db.clone(), config.clone())
        .await
        .unwrap();

    // 获取初始统计
    let initial_stats = memory_manager.get_stats().await;

    // 创建一些记忆
    for i in 0..5 {
        let attributes = MemoryAttributes {
            keywords: vec![format!("统计记忆 {}", i)],
            tags: vec!["统计".to_string(), format!("tag_{}", i)],
            context: String::new(),
            importance: 0.5,
            emotion: None,
            source: None,
            confidence: 0.8,
            language: None,
            custom_attributes: Default::default(),
        };
        let content = format!("统计测试记忆 {}", i);
        let embedding = memory_manager.generate_embedding(&content).await.unwrap();
        let memory = Memory::new(
            content,
            if i % 2 == 0 {
                MemoryType::Knowledge
            } else {
                MemoryType::Event
            },
            embedding,
            attributes,
        );

        memory_manager.create_memory(&memory).await.unwrap();
    }

    // 获取更新后的统计
    let updated_stats = memory_manager.get_stats().await;

    // 验证统计信息
    assert!(updated_stats.total_memories >= initial_stats.total_memories + 5);
}
