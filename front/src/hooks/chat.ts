import { createSignal, createEffect, onCleanup, createResource } from 'solid-js'
import {
    createChatSession,
    sendChatMessage,
    createSSEConnection,
    addSSEMessageListener,
    closeSSEConnection,
    getAllDocumentCategories,
} from '~/api/chat'
import type { ChatSession, Message } from '~/types/chat'
import { createMessage, createNewSession } from '~/utils/chatUtils'

export const useChat = () => {
    const [sessions, setSessions] = createSignal<ChatSession[]>([])
    const [activeSessionId, setActiveSessionId] = createSignal<string | null>(null)

    const [loading, setLoading] = createSignal(false)
    const [backendSessionId, setBackendSessionId] = createSignal<string | null>(null)

    const [eventSource, setEventSource] = createSignal<EventSource | null>(null)
    const [categories] = createResource(async () => {
        const response = await getAllDocumentCategories()
        if (response.code === 0) {
            return response.data
        }
        return []
    })

    // 当前活跃会话
    const activeSession = () => {
        if (!activeSessionId()) return null
        return sessions().find(s => s.id === activeSessionId()) || null
    }

    // 当前会话的消息
    const activeMessages = () => activeSession()?.messages || []

    // 建立SSE连接
    const setupSSEConnection = (sessionId: string) => {
        // 关闭之前的连接
        if (eventSource()) {
            closeSSEConnection(eventSource()!)
        }

        const source = createSSEConnection(sessionId)
        setEventSource(source)

        // 添加消息监听
        addSSEMessageListener(source, messageText => {
            // 处理收到的消息
            const aiMessage = createMessage('assistant', messageText)

            setSessions(prev => {
                return prev.map(session => {
                    if (session.id === activeSessionId()) {
                        return {
                            ...session,
                            messages: [...session.messages, aiMessage],
                            updatedAt: Date.now(),
                        }
                    }
                    return session
                })
            })

            setLoading(false)
        })

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

            setSessions([newSession, ...sessions()])
            setActiveSessionId(newSession.id)

            // 触发界面更新
            return newSession.id
        }
        return null
    }

    // 选择会话
    const selectSession = async (sessionId: string) => {
        const session = sessions().find(s => s.id === sessionId)

        if (session) {
            // 获取后端会话ID
            const backendId = session.metadata?.backendSessionId

            if (backendId) {
                // 设置当前活跃的后端会话ID
                setBackendSessionId(backendId)
                // 建立SSE连接
                setupSSEConnection(backendId)
            } else {
                // 如果没有后端会话ID，创建一个新的
                const newBackendId = await createBackendSession()
                if (newBackendId) {
                    // 更新会话的metadata
                    setSessions(prev =>
                        prev.map(s =>
                            s.id === sessionId
                                ? {
                                      ...s,
                                      metadata: { ...s.metadata, backendSessionId: newBackendId },
                                  }
                                : s
                        )
                    )
                }
            }
        }

        setActiveSessionId(sessionId)
    }

    // 发送消息
    const sendMessage = async (content: string) => {
        if (!activeSessionId()) {
            const newSessionId = await createSession()
            if (!newSessionId) return // 如果创建失败，直接返回
            // 等待状态更新后再继续
            await new Promise(resolve => setTimeout(resolve, 100))
        }

        // 创建用户消息
        const userMessage = createMessage('user', content)

        // 更新会话
        setSessions(prev => {
            return prev.map(session => {
                if (session.id === activeSessionId()) {
                    return {
                        ...session,
                        messages: [...session.messages, userMessage],
                        title:
                            session.messages.length === 0
                                ? content.slice(0, 30) + (content.length > 30 ? '...' : '')
                                : session.title,
                        updatedAt: Date.now(),
                    }
                }
                return session
            })
        })

        // 设置加载状态
        setLoading(true)

        // 获取当前会话的后端会话ID
        const currentSession = sessions().find(s => s.id === activeSessionId())
        const currentBackendId = currentSession?.metadata?.backendSessionId || backendSessionId()

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
            if (response.code !== 0) {
                throw new Error(response.message || '发送消息失败')
            }
        } catch (error) {
            console.error('发送消息失败:', error)
            setLoading(false)

            // 添加错误消息
            const errorMessage = createMessage('system', '消息发送失败，请检查网络连接并重试。')

            setSessions(prev => {
                return prev.map(session => {
                    if (session.id === activeSessionId()) {
                        return {
                            ...session,
                            messages: [...session.messages, errorMessage],
                            updatedAt: Date.now(),
                        }
                    }
                    return session
                })
            })
        }
    }

    // 保存/加载会话
    const loadSessions = (loadedSessions: ChatSession[]) => {
        setSessions(loadedSessions)
    }

    // 注册清理函数
    onCleanup(cleanup)

    return {
        // 状态
        sessions,
        activeSessionId,
        loading,
        categories,
        activeSession,
        activeMessages,

        // 方法
        createSession,
        selectSession,
        sendMessage,
        loadSessions,
        setSessions,
        setActiveSessionId,
    }
}
