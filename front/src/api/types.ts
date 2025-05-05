// 聊天请求结构
export interface ChatRequest {
    message: string
}

// 聊天响应结构
export interface ChatResponse {
    session_id: string
    message: string
}

// 创建SSE查询参数
export interface NewSSEQuery {
    category?: string
}

// 创建SSE响应
export interface NewSSEResponse {
    session_id: string
}

export interface SSEMessage {
    id: string
    content: string
}

// API响应包装类型
export interface ApiResponse<T> {
    status: number
    message: string
    data: T
}

// 文档类别列表响应
export type DocumentCategoryResponse = ApiResponse<string[]>

export interface SessionHistory {
    session_id: string
    title: string
}

export interface MessageHistoryResponse {
    role: 'user' | 'assistant'
    content?: Array<{
        type?: string
        text: string
    }>
}

export type MessageHistoryResponseList = ApiResponse<MessageHistoryResponse[]>

export interface ImageGenerationRequest {
    prompt: string
}
