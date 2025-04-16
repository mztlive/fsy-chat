import { createSignal, createEffect, onCleanup, createResource } from 'solid-js'
import { createStore } from 'solid-js/store'
import {
    createChatSession,
    sendChatMessage,
    createSSEConnection,
    addSSEMessageListener,
    closeSSEConnection,
    getAllDocumentCategories,
} from '~/api/chat'
import { SSEMessage } from '~/api/types'
import type { ChatSession, Message } from '~/types/chat'
import { createMessage, createNewSession } from '~/utils/chatUtils'

export const useChat = () => {
    const [currentSession, setCurrentSession] = createSignal<ChatSession | null>(null)
    const [loading, setLoading] = createSignal(false)
    const [backendSessionId, setBackendSessionId] = createSignal<string | null>(null)

    const [eventSource, setEventSource] = createSignal<EventSource | null>(null)
    const [categories] = createResource(async () => {
        const response = await getAllDocumentCategories()
        if (response.status === 200) {
            return response.data
        }
        return []
    })

    const [currentMessage, setCurrentMessage] = createStore<Message>({
        id: '',
        role: 'assistant',
        content: '',
        timestamp: 0,
    })

    // 当前会话的消息
    const activeMessages = () => currentSession()?.messages || []

    // 处理SSE消息
    const handleSSEMessage = (message: SSEMessage) => {
        // 获取消息ID
        const messageId = message.id
        if (currentMessage.id == messageId) {
            setCurrentMessage('content', prev => prev + message.content)
            return
        }

        setCurrentMessage({
            id: messageId,
            role: 'assistant',
            content: message.content,
            timestamp: Date.now(),
        })

        if (currentSession()) {
            setCurrentSession({
                ...currentSession()!,
                messages: [...currentSession()!.messages, currentMessage],
                updatedAt: Date.now(),
            })
        }

        setLoading(false)
    }

    // 建立SSE连接
    const setupSSEConnection = (sessionId: string) => {
        // 关闭之前的连接
        if (eventSource()) {
            closeSSEConnection(eventSource()!)
        }

        const source = createSSEConnection(sessionId)
        setEventSource(source)

        // 添加消息监听
        addSSEMessageListener(source, handleSSEMessage)

        // 错误处理
        source.onerror = error => {
            console.error('SSE连接错误:', error)
            source.close()
            setEventSource(null)
        }
    }

    // 创建后端会话
    const createBackendSession = async (category?: string) => {
        const response = await createChatSession({ category })
        setBackendSessionId(response.data.session_id)
        // 建立SSE连接
        setupSSEConnection(response.data.session_id)
        return response.data.session_id
    }

    // 清理连接
    const cleanup = () => {
        if (eventSource()) {
            closeSSEConnection(eventSource()!)
        }
    }

    // 创建新会话
    const createSession = async (category?: string) => {
        // 创建后端会话
        const backendId = await createBackendSession(category)

        if (backendId) {
            const newSession = createNewSession()
            // 将后端会话ID保存到前端会话的metadata中
            newSession.metadata = { backendSessionId: backendId }

            setCurrentSession(newSession)

            // 触发界面更新
            return newSession.id
        }
        return null
    }

    // 发送消息
    const sendMessage = async (content: string) => {
        if (!currentSession()) {
            const newSessionId = await createSession()
            if (!newSessionId) return // 如果创建失败，直接返回
            // 等待状态更新后再继续
            await new Promise(resolve => setTimeout(resolve, 100))
        }

        // 创建用户消息
        const userMessage = createMessage('user', content)

        // 更新会话
        if (currentSession()) {
            setCurrentSession({
                ...currentSession()!,
                messages: [...currentSession()!.messages, userMessage],
                title:
                    currentSession()!.messages.length === 0
                        ? content.slice(0, 30) + (content.length > 30 ? '...' : '')
                        : currentSession()!.title,
                updatedAt: Date.now(),
            })
        }

        // 设置加载状态
        setLoading(true)

        // 获取当前会话的后端会话ID
        const currentBackendId = currentSession()?.metadata?.backendSessionId || backendSessionId()

        if (!currentBackendId) {
            console.error('未找到有效的后端会话ID')
            setLoading(false)
            return
        }

        // 发送消息到后端
        try {
            // 确保使用正确的sendChatMessage接口
            const response = await sendChatMessage(currentBackendId, content)

            // 如果发送失败，显示错误信息
            if (response.status !== 200) {
                throw new Error(response.message || '发送消息失败')
            }

            setCurrentMessage('content', '')
            setCurrentMessage('id', '')
        } catch (error) {
            console.error('发送消息失败:', error)
            setLoading(false)

            // 添加错误消息
            const errorMessage = createMessage('system', '消息发送失败，请检查网络连接并重试。')

            if (currentSession()) {
                setCurrentSession({
                    ...currentSession()!,
                    messages: [...currentSession()!.messages, errorMessage],
                    updatedAt: Date.now(),
                })
            }
        }
    }

    // 注册清理函数
    onCleanup(cleanup)

    return {
        // 状态
        currentSession,
        loading,
        categories,
        activeMessages,

        // 方法
        createSession,
        sendMessage,
        setCurrentSession,
    }
}
