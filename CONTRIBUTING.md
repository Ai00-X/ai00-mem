# 贡献指南

感谢您对 AI00-MEM 项目的关注！我们欢迎各种形式的贡献，包括但不限于：

- 🐛 报告问题
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码修复
- ⚡ 性能优化
- 🧪 添加测试
- 🌐 翻译

## 📋 目录

- [开发环境设置](#开发环境设置)
- [项目结构](#项目结构)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [测试指南](#测试指南)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)
- [问题报告](#问题报告)
- [功能建议](#功能建议)

## 🛠️ 开发环境设置

### 前置要求

- **Rust**: 1.70.0 或更高版本
- **Git**: 用于版本控制
- **数据库** (可选):
  - SQLite (默认，无需额外安装)
  - PostgreSQL (如果需要测试PostgreSQL功能)
  - MySQL (如果需要测试MySQL功能)

### 克隆项目

```bash
git clone https://github.com/your-org/ai00-mem.git
cd ai00-mem
```

### 安装依赖

```bash
# 安装Rust依赖
cargo build

# 安装开发工具
cargo install cargo-watch
cargo install cargo-tarpaulin  # 代码覆盖率
cargo install cargo-audit      # 安全审计
cargo install cargo-outdated   # 依赖更新检查
```

### 环境配置

```bash
# 复制环境配置模板
cp .env.example .env

# 编辑配置文件
# 根据需要修改数据库连接等配置
```

### 验证安装

```bash
# 运行测试
cargo test

# 运行示例
cargo run --example quick_test

# 运行基准测试
cargo bench
```

## 📁 项目结构

```
ai00-mem/
├── src/                    # 源代码
│   ├── lib.rs             # 库入口
│   ├── core.rs            # 核心数据结构
│   ├── config.rs          # 配置管理
│   ├── database.rs        # 数据库操作
│   ├── memory.rs          # 记忆管理
│   ├── retrieval.rs       # 检索算法
│   ├── learning.rs        # 学习引擎
│   ├── utils.rs           # 工具函数
│   ├── examples.rs        # 示例代码
│   └── error.rs           # 错误处理
├── examples/              # 示例程序
│   ├── basic_usage.rs     # 基本使用
│   ├── quick_test.rs      # 快速测试
│   ├── benchmark.rs       # 性能测试
│   └── interactive_demo.rs # 交互演示
├── tests/                 # 集成测试
│   └── integration_tests.rs
├── benches/               # 性能基准
│   └── memory_benchmarks.rs
├── docs/                  # 文档
├── Cargo.toml            # 项目配置
├── README.md             # 项目说明
├── CHANGELOG.md          # 变更日志
└── CONTRIBUTING.md       # 贡献指南
```

## 🔄 开发流程

### 1. 创建分支

```bash
# 从main分支创建新分支
git checkout main
git pull origin main
git checkout -b feature/your-feature-name

# 或者修复bug
git checkout -b fix/issue-number-description
```

### 2. 开发

```bash
# 使用cargo-watch进行自动重新编译
cargo watch -x check -x test

# 或者手动运行
cargo check  # 快速检查
cargo test   # 运行测试
```

### 3. 提交代码

```bash
# 添加文件
git add .

# 提交（遵循提交规范）
git commit -m "feat: add new memory retrieval algorithm"

# 推送到远程分支
git push origin feature/your-feature-name
```

### 4. 创建 Pull Request

在GitHub上创建Pull Request，详细描述您的更改。

## 📝 代码规范

### Rust代码风格

我们遵循标准的Rust代码风格：

```bash
# 格式化代码
cargo fmt

# 检查代码风格
cargo clippy -- -D warnings
```

### 命名规范

- **函数和变量**: `snake_case`
- **类型和trait**: `PascalCase`
- **常量**: `SCREAMING_SNAKE_CASE`
- **模块**: `snake_case`

### 文档规范

```rust
/// 简短的一行描述
/// 
/// 详细的描述，解释函数的用途、参数和返回值。
/// 
/// # 参数
/// 
/// * `param1` - 参数1的描述
/// * `param2` - 参数2的描述
/// 
/// # 返回值
/// 
/// 返回值的描述
/// 
/// # 错误
/// 
/// 可能出现的错误情况
/// 
/// # 示例
/// 
/// ```rust
/// use ai00_mem::prelude::*;
/// 
/// let result = function_name(param1, param2)?;
/// assert_eq!(result, expected_value);
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // 实现
}
```

### 错误处理

- 使用 `Result<T, MemoryError>` 进行错误处理
- 提供有意义的错误信息
- 使用 `?` 操作符传播错误

```rust
use crate::error::{MemoryError, Result};

pub fn example_function() -> Result<String> {
    let data = some_operation()
        .map_err(|e| MemoryError::Database(format!("Failed to read data: {}", e)))?;
    
    Ok(data)
}
```

## 🧪 测试指南

### 测试类型

1. **单元测试**: 在每个模块中测试单个函数
2. **集成测试**: 在 `tests/` 目录中测试模块间交互
3. **文档测试**: 在文档注释中的示例代码
4. **基准测试**: 在 `benches/` 目录中的性能测试

### 编写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_memory_creation() {
        let config = Config::default();
        let db = VectorGraphDB::new(config).await.unwrap();
        
        let memory = Memory {
            id: "test_id".to_string(),
            content: "test content".to_string(),
            // ... 其他字段
        };
        
        let result = db.create_memory(&memory).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_utility_function() {
        let input = "test input";
        let expected = "expected output";
        let result = utility_function(input);
        assert_eq!(result, expected);
    }
}
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_memory_creation

# 运行集成测试
cargo test --test integration_tests

# 运行基准测试
cargo bench

# 生成代码覆盖率报告
cargo tarpaulin --out Html
```

## 📋 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

### 提交格式

```
<类型>[可选的作用域]: <描述>

[可选的正文]

[可选的脚注]
```

### 提交类型

- `feat`: 新功能
- `fix`: 修复bug
- `docs`: 文档更新
- `style`: 代码格式化（不影响功能）
- `refactor`: 重构代码
- `perf`: 性能优化
- `test`: 添加或修改测试
- `chore`: 构建过程或辅助工具的变动
- `ci`: CI配置文件和脚本的变动

### 示例

```bash
# 新功能
git commit -m "feat(retrieval): add HippoRAG algorithm implementation"

# 修复bug
git commit -m "fix(database): resolve connection pool deadlock issue"

# 文档更新
git commit -m "docs: update API documentation for memory management"

# 性能优化
git commit -m "perf(cache): optimize memory cache lookup performance"
```

## 🔀 Pull Request 流程

### PR标题

使用与提交信息相同的格式：

```
feat(retrieval): add HippoRAG algorithm implementation
```

### PR描述模板

```markdown
## 📝 描述

简要描述此PR的目的和内容。

## 🔧 更改类型

- [ ] 新功能
- [ ] Bug修复
- [ ] 文档更新
- [ ] 性能优化
- [ ] 重构
- [ ] 测试

## 🧪 测试

- [ ] 添加了新的测试
- [ ] 所有现有测试通过
- [ ] 手动测试通过

## 📋 检查清单

- [ ] 代码遵循项目规范
- [ ] 自我审查了代码
- [ ] 添加了必要的注释
- [ ] 更新了相关文档
- [ ] 没有引入新的警告
- [ ] 添加了适当的测试
- [ ] 所有测试通过

## 🔗 相关Issue

Closes #issue_number

## 📸 截图（如适用）

## 📝 额外说明

任何需要审查者注意的特殊说明。
```

### 审查流程

1. **自动检查**: CI会自动运行测试和代码检查
2. **代码审查**: 至少需要一个维护者的审查
3. **测试验证**: 确保所有测试通过
4. **文档检查**: 确保文档是最新的
5. **合并**: 审查通过后合并到main分支

## 🐛 问题报告

### 报告Bug

使用GitHub Issues报告bug，请包含：

1. **环境信息**:
   - Rust版本
   - 操作系统
   - ai00-mem版本
   - 数据库类型和版本

2. **问题描述**:
   - 期望的行为
   - 实际的行为
   - 重现步骤

3. **代码示例**:
   - 最小可重现示例
   - 错误信息和堆栈跟踪

### Bug报告模板

```markdown
## 🐛 Bug描述

简要描述bug的现象。

## 🔄 重现步骤

1. 执行 '...'
2. 点击 '....'
3. 滚动到 '....'
4. 看到错误

## 🎯 期望行为

描述您期望发生的行为。

## 📸 截图

如果适用，添加截图来帮助解释您的问题。

## 🖥️ 环境信息

- OS: [e.g. Windows 11, macOS 13, Ubuntu 22.04]
- Rust版本: [e.g. 1.70.0]
- ai00-mem版本: [e.g. 0.1.0]
- 数据库: [e.g. SQLite, PostgreSQL 15]

## 📝 额外上下文

添加任何其他关于问题的上下文信息。
```

## 💡 功能建议

### 提出新功能

使用GitHub Issues提出功能建议，请包含：

1. **功能描述**: 详细描述建议的功能
2. **使用场景**: 解释为什么需要这个功能
3. **实现建议**: 如果有想法，描述可能的实现方式
4. **替代方案**: 考虑过的其他解决方案

### 功能建议模板

```markdown
## 🚀 功能建议

简要描述您建议的功能。

## 🎯 问题描述

描述您遇到的问题或需求。

## 💡 建议的解决方案

描述您希望实现的功能。

## 🔄 替代方案

描述您考虑过的任何替代解决方案或功能。

## 📝 额外上下文

添加任何其他关于功能请求的上下文或截图。
```

## 🏆 贡献者认可

我们重视每一个贡献，无论大小。贡献者将被添加到：

- README.md的贡献者列表
- 发布说明中的感谢名单
- 项目的贡献者页面

## 📞 联系方式

如果您有任何问题或需要帮助：

- 📧 邮箱: ai00-mem@example.com
- 💬 GitHub Discussions: [项目讨论区](https://github.com/your-org/ai00-mem/discussions)
- 🐛 问题报告: [GitHub Issues](https://github.com/your-org/ai00-mem/issues)

## 📄 许可证

通过贡献代码，您同意您的贡献将在与项目相同的[MIT许可证](LICENSE)下授权。

---

再次感谢您对AI00-MEM项目的贡献！🎉