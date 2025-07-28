use ai00_mem::database::{VectorGraphDB, Vector, GraphNode, GraphEdge, VectorQueryRequest, GraphQueryRequest};
use ai00_mem::config::{Config, DatabaseConfig, DatabaseType};
use std::collections::HashMap;
use chrono::Utc;
use serde_json::json;

#[tokio::test]
async fn test_memory_and_sqlite_storage() {
    println!("开始测试内存和SQLite存储功能...");
    
    // 创建测试配置 - 使用内存数据库
    let mut config = Config::default();
    config.database.database_type = DatabaseType::SQLite;
    config.database.url = "sqlite://:memory:".to_string();
    config.database.table_prefix = "test_".to_string();
    
    // 创建数据库实例
    let db = VectorGraphDB::new(config).await.expect("创建数据库失败");
    
    // 测试1: 向量存储和检索
    println!("测试向量存储和检索...");
    let vector = Vector {
        id: "test_vector_1".to_string(),
        embedding: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("type".to_string(), json!("test"));
            meta.insert("content".to_string(), json!("测试文本"));
            meta
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    db.insert_vector(&vector).await.expect("存储向量失败");
    
    // 查询向量
    let query_request = VectorQueryRequest {
        query_vector: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        limit: Some(5),
        threshold: Some(0.0),
        filters: None,
    };
    
    let vector_results = db.query_vectors(&query_request).await.expect("查询向量失败");
    assert!(!vector_results.is_empty(), "应该能找到存储的向量");
    assert_eq!(vector_results[0].vector.id, "test_vector_1");
    println!("✓ 向量存储和检索成功");
    
    // 测试2: 图节点存储和检索
    println!("测试图节点存储和检索...");
    let node1 = GraphNode {
        id: "test_node_1".to_string(),
        node_type: "concept".to_string(),
        properties: {
            let mut props = HashMap::new();
            props.insert("name".to_string(), json!("测试概念1"));
            props.insert("description".to_string(), json!("这是一个测试概念"));
            props
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let node2 = GraphNode {
        id: "test_node_2".to_string(),
        node_type: "concept".to_string(),
        properties: {
            let mut props = HashMap::new();
            props.insert("name".to_string(), json!("测试概念2"));
            props
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    db.insert_node(&node1).await.expect("存储节点1失败");
    db.insert_node(&node2).await.expect("存储节点2失败");
    
    // 测试3: 图边存储和检索
    println!("测试图边存储和检索...");
    let edge = GraphEdge {
        id: "test_edge_1".to_string(),
        from_node: "test_node_1".to_string(),
        to_node: "test_node_2".to_string(),
        edge_type: "connects".to_string(),
        weight: 0.8,
        properties: {
            let mut props = HashMap::new();
            props.insert("relationship".to_string(), json!("关联"));
            props
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    db.insert_edge(&edge).await.expect("存储边失败");
    
    // 查询图结构
    let graph_query = GraphQueryRequest {
        start_nodes: vec!["test_node_1".to_string()],
        edge_types: Some(vec!["connects".to_string()]),
        max_depth: Some(2),
        limit: Some(10),
        filters: None,
    };
    
    let graph_results = db.query_graph(&graph_query).await.expect("查询图失败");
    assert!(!graph_results.nodes.is_empty(), "应该能找到节点");
    assert!(!graph_results.edges.is_empty(), "应该能找到边");
    println!("✓ 图存储和检索成功");
    
    // 测试4: 数据库统计信息
    println!("测试数据库统计...");
    let (vector_count, node_count, edge_count, cache_stats) = db.get_stats().await.expect("获取统计失败");
    
    println!("数据库统计:");
    println!("  向量数量: {}", vector_count);
    println!("  节点数量: {}", node_count);
    println!("  边数量: {}", edge_count);
    println!("缓存统计:");
    println!("  缓存向量: {}", cache_stats.0);
    println!("  缓存节点: {}", cache_stats.1);
    println!("  缓存边: {}", cache_stats.2);
    
    assert!(vector_count >= 1, "应该至少有一个向量");
    assert!(node_count >= 2, "应该至少有两个节点");
    assert!(edge_count >= 1, "应该至少有一条边");
    println!("✓ 统计信息测试成功");
    
    println!("内存和SQLite存储测试全部通过!");
}

#[tokio::test]
async fn test_data_persistence() {
    println!("开始测试数据持久化...");
    
    let db_url = "sqlite:C:\\work\\ai00.art\\ai00-mem\\test_persistence.db?mode=rwc";
    
    let db_file = "C:\\work\\ai00.art\\ai00-mem\\test_persistence.db";
    
    // 确保清理旧文件
    let _ = std::fs::remove_file(db_file);
    
    // 第一次运行：创建数据库并存储数据
    {
        let mut config = Config::default();
        config.database.database_type = DatabaseType::SQLite;
        config.database.url = db_url.to_string();
        config.database.table_prefix = "persist_".to_string();
        
        let db = VectorGraphDB::new(config).await.expect("创建数据库失败");
        
        // 存储测试数据
        let vector = Vector {
            id: "persistent_vec".to_string(),
            embedding: vec![10.0, 20.0, 30.0, 40.0, 50.0],
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("persistent".to_string(), json!(true));
                meta.insert("description".to_string(), json!("持久化测试向量"));
                meta
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        db.insert_vector(&vector).await.expect("存储持久化向量失败");
        
        let node = GraphNode {
            id: "persistent_node".to_string(),
            node_type: "persistent_type".to_string(),
            properties: {
                let mut props = HashMap::new();
                props.insert("name".to_string(), json!("持久化节点"));
                props
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        db.insert_node(&node).await.expect("存储持久化节点失败");
        
        println!("✓ 第一次运行：数据存储完成");
    }
    
    // 第二次运行：重新打开数据库，验证数据仍然存在
    {
        let mut config = Config::default();
        config.database.database_type = DatabaseType::SQLite;
        config.database.url = db_url.to_string();
        config.database.table_prefix = "persist_".to_string();
        
        let db = VectorGraphDB::new(config).await.expect("重新打开数据库失败");
        
        // 验证向量仍然存在
        let query_request = VectorQueryRequest {
            query_vector: vec![10.0, 20.0, 30.0, 40.0, 50.0],
            limit: Some(5),
            threshold: Some(0.0),
            filters: None,
        };
        
        let vector_results = db.query_vectors(&query_request).await.expect("查询持久化向量失败");
        assert!(!vector_results.is_empty(), "持久化向量应该仍然存在");
        
        let found_vector = &vector_results[0].vector;
        assert_eq!(found_vector.id, "persistent_vec");
        assert_eq!(found_vector.embedding, vec![10.0, 20.0, 30.0, 40.0, 50.0]);
        assert_eq!(found_vector.metadata.get("persistent"), Some(&json!(true)));
        
        // 验证节点仍然存在
        let graph_query = GraphQueryRequest {
            start_nodes: vec!["persistent_node".to_string()],
            edge_types: None,
            max_depth: Some(1),
            limit: Some(5),
            filters: None,
        };
        
        let graph_results = db.query_graph(&graph_query).await.expect("查询持久化节点失败");
        assert!(!graph_results.nodes.is_empty(), "持久化节点应该仍然存在");
        
        println!("✓ 第二次运行：数据验证完成");
    }
    
    // 清理测试文件
    let _ = std::fs::remove_file("./test_persistence.db");
    
    println!("数据持久化测试通过!");
}

#[tokio::test]
async fn test_memory_cache_functionality() {
    println!("开始测试内存缓存功能...");
    
    let mut config = Config::default();
    config.database.database_type = DatabaseType::SQLite;
    config.database.url = "sqlite://:memory:".to_string();
    config.database.table_prefix = "cache_".to_string();
    
    let db = VectorGraphDB::new(config).await.expect("创建数据库失败");
    
    // 存储多个向量
    for i in 0..5 {
        let vector = Vector {
            id: format!("cache_vec_{}", i),
            embedding: vec![i as f32, (i+1) as f32, (i+2) as f32],
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        db.insert_vector(&vector).await.expect("存储向量失败");
    }
    
    // 获取统计信息
    let (vector_count, node_count, edge_count, cache_stats) = db.get_stats().await.expect("获取统计失败");
    
    println!("缓存测试统计:");
    println!("  数据库向量: {}", vector_count);
    println!("  缓存向量: {}", cache_stats.0);
    println!("  缓存节点: {}", cache_stats.1);
    println!("  缓存边: {}", cache_stats.2);
    
    assert!(vector_count >= 5, "应该存储了5个向量");
    
    println!("内存缓存测试通过!");
}