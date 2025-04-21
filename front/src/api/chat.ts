import { BASE_URL, get, post } from './request'
import {
    ApiResponse,
    ChatRequest,
    MessageHistoryResponse,
    NewSSEQuery,
    NewSSEResponse,
    SessionHistory,
    SSEMessage,
} from './types'

// 创建聊天会话
export function createChatSession(params?: NewSSEQuery): Promise<ApiResponse<NewSSEResponse>> {
    return get<NewSSEResponse>('/chat/create', params)
}

// 发送聊天消息
export function sendChatMessage(sessionId: string, message: string): Promise<ApiResponse<void>> {
    return post<void>(`/chat/message/${sessionId}`, { message } as ChatRequest)
}

// 获取所有文档类别
export function getAllDocumentCategories(): Promise<ApiResponse<string[]>> {
    return get<string[]>('/all/document/category')
}

// 创建SSE连接
export function createSSEConnection(sessionId: string): EventSource {
    return new EventSource(`${BASE_URL}/chat/sse/${sessionId}`)
}

// SSE消息事件监听器类型
export type SSEMessageListener = (message: SSEMessage) => void

// 添加SSE消息监听
export function addSSEMessageListener(source: EventSource, listener: SSEMessageListener): void {
    source.addEventListener('new-message', (event: MessageEvent) => {
        try {
            // 将接收到的JSON字符串解析为对象
            const data = JSON.parse(event.data)
            listener(data)
        } catch (error) {
            console.error('解析SSE消息失败:', error)
        }
    })
}

export function addSSEErrorListener(source: EventSource, listener: SSEMessageListener): void {
    source.addEventListener('error', (event: MessageEvent) => {
        try {
            // 将接收到的JSON字符串解析为对象
            const data = JSON.parse(event.data)
            listener(data)
        } catch (error) {
            console.error('解析SSE消息失败:', error)
        }
    })
}

// 关闭SSE连接
export function closeSSEConnection(source: EventSource): void {
    source.close()
}

// 获取会话历史
export function getSessionHistory(): Promise<ApiResponse<SessionHistory[]>> {
    return get<SessionHistory[]>('/session/history')
}

// 获取会话消息历史
export function getMessageHistory(
    sessionId: string
): Promise<ApiResponse<MessageHistoryResponse[]>> {
    return get<MessageHistoryResponse[]>(`/message/history/${sessionId}`)
}
