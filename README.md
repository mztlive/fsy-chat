# FSY AI 聊天应用

FSY AI 聊天是一个基于 Rust 编写的智能聊天应用，支持与大语言模型进行对话，并具有文档检索和向量搜索功能。应用采用模块化设计，易于扩展和维护。

## 主要功能

- 基于大语言模型的智能对话
- 支持多个聊天会话管理
- 文档加载和向量检索
- 会话持久化存储
- Web 界面接口
- 支持文档类别和上下文相关问答

## 系统架构

系统主要由以下模块组成：

- **核心模块 (Kernel)**: 负责协调各个组件，管理会话和代理创建
- **聊天模块 (Chat)**: 处理与 AI 模型的对话，管理聊天历史
- **会话管理 (Session Manager)**: 负责多用户、多会话的存储和管理
- **文档加载器 (Document Loader)**: 加载和管理文档库
- **向量存储 (Vector Store)**: 将文档转换为向量并支持语义搜索
- **配置模块 (Config)**: 管理应用程序配置
- **存储模块 (Storages)**: 提供会话持久化功能
- **Web 服务 (Web)**: 提供 HTTP API 接口

## 快速开始

### 配置

创建`config.toml`文件，包含以下内容：

```toml
[agent]
api_key = "你的API密钥"
preamble = "你是一个智能助手，请回答问题"
chat_model = "qwen-max"

[embedding]
model = "text-embedding-v1"

[document]
[[document.categories]]
name = "default"
directory = "./data"
```

### 运行

```bash
cargo run -- --config config.toml --port 3000
```

## API 接口

应用提供以下主要 API 端点：

- `POST /api/chat`: 发送聊天消息
- `GET /api/sessions`: 获取所有会话列表
- `POST /api/sessions`: 创建新会话
- `DELETE /api/sessions/{id}`: 删除会话
- `GET /api/categories`: 获取文档类别列表

## 开发指南

### 项目结构

```
src/
├── chat.rs           # 聊天会话模块
├── kernel.rs         # 核心协调模块
├── config.rs         # 配置处理
├── document_loader.rs # 文档加载器
├── vector_store.rs   # 向量存储和检索
├── session_manager/  # 会话管理
├── storages/         # 持久化存储
├── web/              # Web服务器
└── main.rs           # 程序入口
```

### 扩展指南

#### 添加新的文档类别

1. 在配置文件中添加新类别
2. 在相应目录中添加 JSON 格式的文档

#### 自定义大语言模型

修改`config.toml`文件中的`agent.chat_model`字段。

#### 实现新的存储后端

1. 实现`storage.rs`中的`Storage` trait
2. 在`kernel.rs`中集成新的存储实现

## 许可证

[MIT License](LICENSE)
