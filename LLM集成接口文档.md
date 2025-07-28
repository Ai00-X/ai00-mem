# AI00-Mem 记忆系统 LLM 集成接口文档

本文档详细说明了AI00-Mem记忆系统中需要LLM配合的各个接口，包括输入输出格式和示例。

## 1. 文本嵌入生成接口

### 1.1 记忆内容嵌入生成

**接口位置**: `src/memory.rs:generate_embedding()`

**功能**: 将记忆文本内容转换为高维语义嵌入向量

**输入格式**:
```rust
struct EmbeddingRequest {
    text: String,           // 需要生成嵌入的文本
    model: String,          // 使用的嵌入模型名称
    dimension: usize,       // 嵌入向量维度
}
```

**期望输出**:
```rust
struct EmbeddingResponse {
    embedding: Vec<f32>,    // 嵌入向量
    model_used: String,     // 实际使用的模型
    token_count: usize,     // 处理的token数量
}
```

**输入示例**:
```json
{
    "text": "机器学习是人工智能的一个重要分支，它使计算机能够从数据中学习并做出预测。",
    "model": "text-embedding-ada-002",
    "dimension": 1536
}
```

**期望输出示例**:
```json
{
    "embedding": [0.123, -0.456, 0.789, ...],
    "model_used": "text-embedding-ada-002",
    "token_count": 28
}
```

### 1.2 查询嵌入生成

**接口位置**: `src/retrieval.rs:generate_query_embedding()`

**功能**: 将用户查询转换为嵌入向量用于相似度检索

**输入格式**:
```rust
struct QueryEmbeddingRequest {
    query: String,          // 用户查询文本
    context: Option<String>, // 查询上下文
    language: Option<String>, // 查询语言
}
```

**期望输出**:
```rust
struct QueryEmbeddingResponse {
    embedding: Vec<f32>,    // 查询嵌入向量
    processed_query: String, // 预处理后的查询
}
```

**输入示例**:
```json
{
    "query": "深度学习算法有哪些？",
    "context": "用户正在学习AI相关知识",
    "language": "zh"
}
```

**期望输出示例**:
```json
{
    "embedding": [0.234, -0.567, 0.890, ...],
    "processed_query": "深度学习算法"
}
```

## 2. 智能属性提取接口

### 2.1 记忆属性提取

**接口位置**: `src/memory.rs:extract_attributes()`

**功能**: 从记忆内容中提取关键词、标签、情感等属性

**输入格式**:
```rust
struct AttributeExtractionRequest {
    content: String,        // 记忆内容
    context: Context,       // 上下文信息
    extraction_types: Vec<AttributeType>, // 需要提取的属性类型
}

enum AttributeType {
    Keywords,
    Tags,
    Emotion,
    Importance,
    Language,
    Theme,
}
```

**期望输出**:
```rust
struct AttributeExtractionResponse {
    keywords: Vec<String>,
    tags: Vec<String>,
    emotion: Option<String>,
    importance: f32,
    confidence: f32,
    language: Option<String>,
    themes: Vec<String>,
}
```

**输入示例**:
```json
{
    "content": "今天学习了卷积神经网络(CNN)的基本原理，它在图像识别领域表现出色。通过多层卷积和池化操作，CNN能够自动提取图像特征。",
    "extraction_types": ["Keywords", "Tags", "Emotion", "Importance"]
}
```

**期望输出示例**:
```json
{
    "keywords": ["卷积神经网络", "CNN", "图像识别", "卷积", "池化", "特征提取"],
    "tags": ["深度学习", "神经网络", "计算机视觉", "机器学习"],
    "emotion": "积极学习",
    "importance": 0.8,
    "confidence": 0.9,
    "language": "zh",
    "themes": ["人工智能", "技术学习"]
}
```

### 2.2 关键词提取

**接口位置**: `src/utils.rs:extract_keywords()`

**功能**: 从文本中提取关键词

**输入格式**:
```rust
struct KeywordExtractionRequest {
    text: String,
    max_keywords: usize,
    language: Option<String>,
    domain: Option<String>,  // 领域信息，如"技术"、"医学"等
}
```

**期望输出**:
```rust
struct KeywordExtractionResponse {
    keywords: Vec<KeywordInfo>,
}

struct KeywordInfo {
    word: String,
    score: f32,     // 关键词重要性评分
    category: Option<String>, // 关键词类别
}
```

**输入示例**:
```json
{
    "text": "Transformer架构彻底改变了自然语言处理领域，其自注意力机制能够捕捉长距离依赖关系。",
    "max_keywords": 5,
    "language": "zh",
    "domain": "技术"
}
```

**期望输出示例**:
```json
{
    "keywords": [
        {"word": "Transformer", "score": 0.95, "category": "技术术语"},
        {"word": "自然语言处理", "score": 0.90, "category": "技术领域"},
        {"word": "自注意力机制", "score": 0.85, "category": "技术概念"},
        {"word": "长距离依赖", "score": 0.75, "category": "技术特性"},
        {"word": "架构", "score": 0.70, "category": "技术概念"}
    ]
}
```

## 3. 关系推理接口

### 3.1 因果关系分析

**接口位置**: `src/memory.rs:extract_causal_keywords()`

**功能**: 分析记忆之间的因果关系

**输入格式**:
```rust
struct CausalAnalysisRequest {
    source_memory: Memory,
    target_memory: Memory,
    context: Vec<Memory>,   // 相关上下文记忆
}
```

**期望输出**:
```rust
struct CausalAnalysisResponse {
    has_causal_relation: bool,
    relation_type: CausalRelationType,
    confidence: f32,
    explanation: String,
    causal_keywords: Vec<String>,
}

enum CausalRelationType {
    Cause,      // 源记忆是目标记忆的原因
    Effect,     // 源记忆是目标记忆的结果
    Correlation, // 相关但非因果
    None,       // 无关系
}
```

**输入示例**:
```json
{
    "source_memory": {
        "content": "用户反馈系统响应速度太慢",
        "timestamp": "2024-01-15T10:00:00Z"
    },
    "target_memory": {
        "content": "优化了数据库查询，添加了索引",
        "timestamp": "2024-01-16T14:30:00Z"
    },
    "context": []
}
```

**期望输出示例**:
```json
{
    "has_causal_relation": true,
    "relation_type": "Cause",
    "confidence": 0.85,
    "explanation": "用户反馈响应速度慢导致了数据库优化行为",
    "causal_keywords": ["反馈", "响应速度", "优化", "数据库"]
}
```

### 3.2 语义相似性分析

**接口位置**: `src/retrieval.rs:semantic_retrieval()`

**功能**: 分析记忆之间的语义相似性

**输入格式**:
```rust
struct SemanticSimilarityRequest {
    memory1: Memory,
    memory2: Memory,
    analysis_depth: AnalysisDepth,
}

enum AnalysisDepth {
    Surface,    // 表面语义
    Deep,       // 深层语义
    Conceptual, // 概念层面
}
```

**期望输出**:
```rust
struct SemanticSimilarityResponse {
    similarity_score: f32,
    similarity_aspects: Vec<SimilarityAspect>,
    shared_concepts: Vec<String>,
    explanation: String,
}

struct SimilarityAspect {
    aspect: String,
    score: f32,
}
```

**输入示例**:
```json
{
    "memory1": {
        "content": "机器学习模型需要大量数据进行训练"
    },
    "memory2": {
        "content": "深度神经网络的训练依赖于海量数据集"
    },
    "analysis_depth": "Deep"
}
```

**期望输出示例**:
```json
{
    "similarity_score": 0.87,
    "similarity_aspects": [
        {"aspect": "主题相似性", "score": 0.90},
        {"aspect": "概念重叠", "score": 0.85},
        {"aspect": "语义结构", "score": 0.80}
    ],
    "shared_concepts": ["机器学习", "训练", "数据", "模型"],
    "explanation": "两个记忆都涉及机器学习模型训练对数据的依赖性"
}
```

## 4. 内容理解接口

### 4.1 重要性评估

**接口位置**: `src/learning.rs:calculate_importance()`

**功能**: 评估记忆内容的重要性

**输入格式**:
```rust
struct ImportanceAssessmentRequest {
    memory: Memory,
    user_context: UserContext,
    historical_interactions: Vec<Interaction>,
}

struct UserContext {
    user_id: String,
    interests: Vec<String>,
    expertise_level: ExpertiseLevel,
    current_goals: Vec<String>,
}
```

**期望输出**:
```rust
struct ImportanceAssessmentResponse {
    importance_score: f32,
    reasoning: String,
    factors: Vec<ImportanceFactor>,
}

struct ImportanceFactor {
    factor: String,
    weight: f32,
    contribution: f32,
}
```

**输入示例**:
```json
{
    "memory": {
        "content": "学会了使用Git进行版本控制",
        "tags": ["编程", "工具"]
    },
    "user_context": {
        "user_id": "dev_001",
        "interests": ["软件开发", "编程"],
        "expertise_level": "Intermediate",
        "current_goals": ["提高开发效率"]
    },
    "historical_interactions": []
}
```

**期望输出示例**:
```json
{
    "importance_score": 0.85,
    "reasoning": "Git是软件开发的基础工具，与用户的兴趣和目标高度匹配",
    "factors": [
        {"factor": "与用户兴趣匹配", "weight": 0.3, "contribution": 0.9},
        {"factor": "实用性", "weight": 0.25, "contribution": 0.95},
        {"factor": "基础重要性", "weight": 0.2, "contribution": 0.8},
        {"factor": "学习价值", "weight": 0.25, "contribution": 0.75}
    ]
}
```

### 4.2 情感分析

**接口位置**: `src/memory.rs:extract_attributes()` (emotion部分)

**功能**: 分析记忆内容的情感倾向

**输入格式**:
```rust
struct EmotionAnalysisRequest {
    text: String,
    context: Option<String>,
    language: Option<String>,
}
```

**期望输出**:
```rust
struct EmotionAnalysisResponse {
    primary_emotion: String,
    emotion_scores: HashMap<String, f32>,
    sentiment: Sentiment,
    confidence: f32,
}

enum Sentiment {
    Positive,
    Negative,
    Neutral,
}
```

**输入示例**:
```json
{
    "text": "今天的项目演示非常成功，客户对我们的方案很满意！",
    "context": "工作汇报",
    "language": "zh"
}
```

**期望输出示例**:
```json
{
    "primary_emotion": "喜悦",
    "emotion_scores": {
        "喜悦": 0.85,
        "满足": 0.75,
        "自豪": 0.60,
        "兴奋": 0.45
    },
    "sentiment": "Positive",
    "confidence": 0.90
}
```

## 5. 个性化学习接口

### 5.1 用户偏好分析

**接口位置**: `src/learning.rs:PreferenceLearner`

**功能**: 分析用户的兴趣偏好和学习模式

**输入格式**:
```rust
struct PreferenceAnalysisRequest {
    user_id: String,
    interaction_history: Vec<Interaction>,
    memory_access_patterns: Vec<MemoryAccess>,
    feedback_records: Vec<FeedbackRecord>,
}
```

**期望输出**:
```rust
struct PreferenceAnalysisResponse {
    topic_preferences: HashMap<String, f32>,
    learning_style: LearningStyle,
    interaction_patterns: InteractionPatterns,
    recommendations: Vec<String>,
}
```

**输入示例**:
```json
{
    "user_id": "user_001",
    "interaction_history": [
        {"query": "机器学习算法", "timestamp": "2024-01-15T10:00:00Z"},
        {"query": "深度学习框架", "timestamp": "2024-01-15T11:00:00Z"}
    ],
    "memory_access_patterns": [],
    "feedback_records": []
}
```

**期望输出示例**:
```json
{
    "topic_preferences": {
        "机器学习": 0.90,
        "深度学习": 0.85,
        "人工智能": 0.80,
        "编程": 0.75
    },
    "learning_style": "Visual",
    "interaction_patterns": {
        "preferred_time": "morning",
        "session_length": "medium",
        "query_complexity": "intermediate"
    },
    "recommendations": [
        "推荐更多机器学习相关内容",
        "建议学习TensorFlow框架"
    ]
}
```

## 6. 查询理解接口

### 6.1 记忆检索接口

**接口位置**: `src/retrieval.rs:MemoryManager::retrieve_memories`

**功能**: 根据LLM的查询需求检索相关记忆

**输入格式**:
```rust
struct Query {
    text: String,
    memory_type: Option<MemoryType>,
    limit: usize,
    offset: usize,
    sort_by: SortBy,
    weights: RetrievalWeights,
}

struct RetrievalWeights {
    semantic: f32,
    temporal: f32,
    importance: f32,
    personalization: f32,
}
```

**期望输出**:
```rust
struct RetrievalResult {
    memory: Memory,
    relevance_score: f32,
    explanation: RetrievalExplanation,
}

struct RetrievalExplanation {
    semantic_score: f32,
    temporal_score: f32,
    importance_score: f32,
    personalization_score: f32,
    connection_paths: Vec<Vec<String>>,
    reasoning: String,
}
```

**输入示例**:
```json
{
  "query": {
    "text": "Rust编程最佳实践",
    "memory_type": "knowledge",
    "limit": 10,
    "offset": 0,
    "sort_by": "relevance",
    "weights": {
      "semantic": 0.4,
      "temporal": 0.2,
      "importance": 0.3,
      "personalization": 0.1
    }
  },
  "context": {
    "session_id": "session_123",
    "current_topic": "Rust学习",
    "user_preferences": {
      "programming": 0.9,
      "rust": 0.8
    }
  }
}
```

**期望输出示例**:
```json
{
  "success": true,
  "results": [
    {
      "memory": {
        "id": "mem_abc123",
        "content": "Rust的所有权系统保证了内存安全",
        "memory_type": "knowledge",
        "importance": 0.9,
        "created_at": "2024-01-15T10:30:00Z",
        "tags": ["Rust", "内存管理"],
        "keywords": ["所有权", "内存安全", "借用检查器"],
        "confidence": 0.95
      },
      "relevance_score": 0.92,
      "explanation": {
        "semantic_score": 0.90,
        "temporal_score": 0.85,
        "importance_score": 0.9,
        "personalization_score": 0.88,
        "connection_paths": [["mem_xyz789", "mem_def456"]],
        "reasoning": "与用户查询高度相关，且与用户偏好匹配"
      }
    }
  ],
  "total_count": 15
}
```

## 7. 实现建议

### 7.1 API集成方式

1. **OpenAI API集成**:
   ```rust
   use openai_api_rs::v1::api::OpenAIClient;
   use openai_api_rs::v1::embedding::EmbeddingRequest;
   ```

2. **本地模型集成**:
   ```rust
   use candle_core::Device;
   use candle_nn::VarBuilder;
   use candle_transformers::models::bert::BertModel;
   ```

3. **异步处理**:
   ```rust
   #[async_trait]
   pub trait LLMProvider {
       async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>>;
       async fn extract_attributes(&self, text: &str) -> Result<MemoryAttributes>;
       async fn analyze_sentiment(&self, text: &str) -> Result<EmotionAnalysis>;
   }
   ```

### 7.2 记忆存储接口

#### 接口描述
将LLM处理后的内容存储到记忆系统中

#### 输入格式
```json
{
  "content": "文本内容",
  "type": "knowledge|event|task|conversation|reflection|goal|habit|emotion",
  "context": {
    "session_id": "会话ID",
    "current_topic": "当前主题",
    "user_preferences": {},
    "environment": {}
  },
  "attributes": {
    "keywords": ["关键词1", "关键词2"],
    "tags": ["标签1", "标签2"],
    "importance": 0.8,
    "confidence": 0.9,
    "emotion": "neutral",
    "source": "llm_response",
    "language": "zh"
  }
}
```

#### 输出格式
```json
{
  "success": true,
  "memory": {
    "id": "记忆唯一ID",
    "content": "文本内容",
    "memory_type": "knowledge",
    "importance": 0.8,
    "created_at": "2024-01-01T12:00:00Z"
  },
  "embedding_generated": true,
  "connections_established": 3,
  "similar_memories_found": 5
}
```

#### Rust实现示例
```rust
use ai00_mem::{MemoryManager, CreateMemoryRequest, Context, MemoryType, MemoryAttributes};

async fn store_llm_content(
    memory_manager: &MemoryManager,
    content: &str,
    memory_type: MemoryType,
    session_id: &str
) -> Result<String> {
    let request = CreateMemoryRequest {
        content: content.to_string(),
        context: Context {
            session_id: Some(session_id.to_string()),
            ..Default::default()
        },
        attributes: Some(MemoryAttributes {
            tags: vec!["llm_response".to_string()],
            importance: 0.8,
            source: Some("llm_response".to_string()),
            ..Default::default()
        }),
        force_connections: None,
    };
    
    let memory = memory_manager.create_memory_from_request(request).await?;
    Ok(memory.id)
}
```

### 7.2 配置管理

#### 代码配置示例

```rust
use ai00_mem::config::{Config, DatabaseConfig, VectorConfig, CacheConfig, 
                        DatabaseType, VectorIndexType, DistanceMetric, CacheType};

// 创建完整配置
let config = Config {
    database: DatabaseConfig {
        database_type: DatabaseType::SQLite,
        url: "sqlite://memory.db".to_string(),
        max_connections: 100,
        min_connections: 10,
        database_name: "ai00_mem".to_string(),
        table_prefix: "mem_".to_string(),
        auto_migrate: true,
        ..Default::default()
    },
    vector: VectorConfig {
        dimension: 384,
        similarity_threshold: 0.7,
        index_type: VectorIndexType::HNSW,
        distance_metric: DistanceMetric::Cosine,
        ..Default::default()
    },
    cache: CacheConfig {
        enabled: true,
        cache_type: CacheType::Memory,
        max_size_mb: 512,
        ttl_seconds: 3600,
        lru_capacity: 1000,
        ..Default::default()
    },
    ..Default::default()
};

// LLM集成配置
#[derive(Debug, Clone)]
pub struct LLMIntegrationConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub timeout_seconds: u64,
}
```

### 7.3 错误处理

```rust
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("API调用失败: {0}")]
    ApiError(String),
    #[error("模型加载失败: {0}")]
    ModelLoadError(String),
    #[error("输入格式错误: {0}")]
    InputFormatError(String),
    #[error("超时错误")]
    TimeoutError,
    #[error("配额不足")]
    QuotaExceededError,
}
```

### 7.4 缓存策略

```rust
pub struct LLMCache {
    embedding_cache: Arc<RwLock<LruCache<String, Vec<f32>>>>,
    attribute_cache: Arc<RwLock<LruCache<String, MemoryAttributes>>>,
    ttl: Duration,
}
```

## 8. 测试示例

### 8.1 单元测试

```rust
#[tokio::test]
async fn test_embedding_generation() {
    let llm_provider = MockLLMProvider::new();
    let text = "测试文本";
    
    let embedding = llm_provider.generate_embedding(text).await.unwrap();
    
    assert_eq!(embedding.len(), 1536);
    assert!(embedding.iter().all(|&x| x >= -1.0 && x <= 1.0));
}
```

### 8.2 集成测试

```rust
#[tokio::test]
async fn test_full_memory_pipeline() {
    let memory_manager = MemoryManager::new_with_llm(llm_provider).await;
    
    let memory_id = memory_manager.create_memory(
        "深度学习是机器学习的子集",
        &context
    ).await.unwrap();
    
    let results = memory_manager.retrieve_memories(
        &Query::new("什么是深度学习？")
    ).await.unwrap();
    
    assert!(!results.is_empty());
}
```

---

**注意**: 本文档中的接口设计是基于当前代码结构的建议，实际实现时可能需要根据具体的LLM服务提供商API进行调整。建议优先实现嵌入生成和属性提取接口，然后逐步扩展其他功能。