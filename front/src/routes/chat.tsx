import { createFileRoute, useNavigate, useSearch } from '@tanstack/solid-router'
import { createEffect, createMemo, createSignal, For, untrack } from 'solid-js'
import { SessionList } from '../components/SessionList'
import { useChat } from '~/hooks/chat'
import { useChatManager } from '~/hooks/chat_manager'
import ChatContent from '~/components/ChatContent'

export const Route = createFileRoute('/chat')({
    validateSearch: search =>
        search as {
            session_id?: string
        },

    component: ChatRoute,
})

function ChatRoute() {
    const [sidebarOpen, setSidebarOpen] = createSignal(false)
    const [showCategories, setShowCategories] = createSignal(false)

    const chat = useChat()
    const chatManager = useChatManager()
    const searchParams = useSearch({ from: '/chat' })
    const navigate = useNavigate({ from: '/chat' })

    const activeSessionId = createMemo(() => searchParams().session_id)

    // 切换侧边栏
    const toggleSidebar = () => {
        setSidebarOpen(!sidebarOpen())
    }

    // 监听会话ID变化并连接会话
    createEffect(() => {
        const sessionId = activeSessionId()
        if (sessionId) {
            // 不知道为什么，不佳untrack会导致createEffect一直触发。可能是内部的响应式状态更新了，导致createEffect一直触发。
            untrack(() => {
                chat.connect(sessionId)
                chat.loadMessageHistory()
            })
        }
    })

    // 处理创建新会话
    const handleCreateNewSession = async (category?: string) => {
        setSidebarOpen(false)
        setShowCategories(true)
    }

    const handleSelectCategory = async (category: string) => {
        setShowCategories(false)
        await chatManager.createSession.mutateAsync(category)
        navigate({ search: { session_id: chatManager.createSession.data } })
    }

    // 处理选择会话
    const handleSelectSession = async (sessionId: string) => {
        // 在移动端选择会话后关闭侧边栏
        setSidebarOpen(false)
        navigate({ search: { session_id: sessionId } })
    }

    // 处理删除会话
    const handleDeleteSession = async (sessionId: string) => {
        await chatManager.removeSession.mutateAsync(sessionId)

        if (sessionId === activeSessionId()) {
            chat.cleanup()
            navigate({ search: { session_id: undefined } })
        }

        chatManager.sessionHistory.refetch()
    }

    return (
        <div class="h-screen grid grid-cols-1 md:grid-cols-[16rem_1fr] bg-base-100 text-base-content overflow-hidden">
            {/* 侧边栏遮罩 - 仅在移动端显示 */}
            <div
                class={`fixed inset-0 bg-black/50 z-10 md:hidden ${sidebarOpen() ? 'block' : 'hidden'}`}
                onClick={() => setSidebarOpen(false)}
            ></div>

            {/* 侧边栏 */}
            <div
                class={`
                bg-base-100 border-r border-base-300/30 h-screen
                fixed md:static z-20 w-64 transition-transform 
                ${sidebarOpen() ? 'translate-x-0' : '-translate-x-full md:translate-x-0'}
                `}
            >
                <SessionList
                    sessions={chatManager.sessionHistory.data}
                    activeSessionId={activeSessionId() ?? null}
                    onSelectSession={handleSelectSession}
                    onCreateNewSession={handleCreateNewSession}
                    onDeleteSession={handleDeleteSession}
                    categories={[]}
                />
            </div>

            {/* 主要内容区域 */}
            <ChatContent
                messages={chat.messages}
                loading={chat.loading()}
                sseError={chat.sseError()}
                isCreatingSession={chatManager.createSession.isPending}
                activeSessionId={activeSessionId()}
                onSendMessage={chat.sendMessage}
                onStartChat={() => setShowCategories(true)}
                onToggleSidebar={toggleSidebar}
            />

            {/* 选择助手类型 - 使用dialog作为模态框 */}
            <dialog id="categories" class="modal" open={showCategories()}>
                <div class="modal-box">
                    <h3 class="text-lg font-bold">请选择助手类型</h3>
                    <div class="flex flex-col gap-4 mt-4 max-w-md">
                        <For each={chatManager.categories()}>
                            {category => (
                                <button
                                    class="btn btn-outline w-full"
                                    onClick={() => handleSelectCategory(category)}
                                >
                                    {category}
                                </button>
                            )}
                        </For>
                    </div>
                    <div class="modal-action">
                        <button class="btn" onClick={() => setShowCategories(false)}>
                            关闭
                        </button>
                    </div>
                </div>
            </dialog>
        </div>
    )
}
