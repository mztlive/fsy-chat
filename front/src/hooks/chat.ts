import { createSignal, createEffect, onCleanup, createResource } from 'solid-js'
import { createStore, SetStoreFunction } from 'solid-js/store'
import {
    createChatSession,
    sendChatMessage,
    createSSEConnection,
    addSSEMessageListener,
    closeSSEConnection,
    getAllDocumentCategories,
    getMessageHistory,
} from '~/api/chat'
import { SSEMessage } from '~/api/types'
import type { ChatSession, Message } from '~/types/chat'
import { createMessage, createNewSession } from '~/utils/chatUtils'

export const useChat = () => {
    const [loading, setLoading] = createSignal(false)
    const [eventSource, setEventSource] = createSignal<EventSource | null>(null)
    const [messages, setMessages] = createStore<Message[]>([])
    const [currentMessageId, setCurrentMessageId] = createSignal<string>('')
    const [sessionId, setSessionId] = createSignal<string>('')

    // 追加消息内容到指定下标
    const appendMessage = (index: number, message: SSEMessage) => {
        if (index != -1) {
            setMessages(index, 'content', prev => prev + message.content)
        }
    }

    // 追加消息到当前消息
    const appendMessageToCurrentMessage = (message: SSEMessage) => {
        appendMessage(
            messages.findIndex(m => m.id == currentMessageId()),
            message
        )
    }

    const loadMessageHistory = async () => {
        const response = await getMessageHistory(sessionId())
        response.data.forEach((message, index) => {
            setMessages([
                ...messages,
                {
                    id: index.toString(),
                    role: message.role,
                    content: message.content?.[0]?.text ?? '',
                    timestamp: Date.now(),
                },
            ])
        })
    }

    // 处理SSE消息
    const handleSSEMessage = (message: SSEMessage) => {
        if (currentMessageId() == message.id) {
            appendMessageToCurrentMessage(message)
            return
        }

        // 意味着是新消息
        setCurrentMessageId(message.id)
        setMessages([
            ...messages,
            {
                id: message.id,
                role: 'assistant',
                content: message.content,
                timestamp: Date.now(),
            },
        ])

        setLoading(false)
    }

    // 建立SSE连接
    const connect = (_sessionId: string) => {
        // 关闭之前的连接
        if (eventSource()) {
            closeSSEConnection(eventSource()!)
        }

        const source = createSSEConnection(_sessionId)
        setEventSource(source)

        // 添加消息监听
        addSSEMessageListener(source, handleSSEMessage)

        // 错误处理
        source.onerror = error => {
            console.error('SSE连接错误:', error)
            source.close()
            setEventSource(null)
        }

        setSessionId(_sessionId)
        console.log('连接成功:', sessionId())
    }

    // 清理连接
    const cleanup = () => {
        if (eventSource()) {
            closeSSEConnection(eventSource()!)
        }
    }

    // 发送消息
    const sendMessage = async (content: string) => {
        // 设置加载状态
        setLoading(true)

        // 创建用户消息
        const userMessage = createMessage('user', content)

        setMessages([...messages, userMessage])

        // 发送消息到后端
        try {
            if (sessionId() == '') {
                throw new Error('未找到会话ID')
            }

            await sendChatMessage(sessionId(), content)
        } catch (error) {
            console.error('发送消息失败:', error)
            setLoading(false)

            // 添加错误消息
            const errorMessage = createMessage('system', '消息发送失败，请检查网络连接并重试。')
            setMessages([...messages, errorMessage])
        }
    }

    // 注册清理函数
    onCleanup(cleanup)

    return {
        loading,
        messages,
        // 方法
        sendMessage,
        connect,
        loadMessageHistory,
    }
}
