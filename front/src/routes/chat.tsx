import { createFileRoute, useNavigate, useSearch } from '@tanstack/solid-router'
import { createEffect, createMemo, createSignal, For, onMount, untrack } from 'solid-js'
import { ChatInput } from '../components/ChatInput'
import { ChatWindow } from '../components/ChatWindow'
import { SessionList } from '../components/SessionList'
import { MenuIcon, RefreshIcon, MoreIcon, PlusIcon } from '../components/icons'
import { useChat } from '~/hooks/chat'
import { useChatManager } from '~/hooks/chat_manager'

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

    return (
        <div class="h-screen flex bg-base-100 text-base-content overflow-hidden">
            {/* 侧边栏遮罩 - 仅在移动端显示 */}
            <div
                class={`fixed inset-0 bg-black/50 z-10 md:hidden ${sidebarOpen() ? 'block' : 'hidden'}`}
                onClick={() => setSidebarOpen(false)}
            ></div>

            {/* 侧边栏 */}
            <div
                class={`
          w-64 bg-base-100 border-r border-base-300/30 h-screen flex-shrink-0 
          fixed md:static z-20 transition-transform ${sidebarOpen() ? 'translate-x-0' : '-translate-x-full md:translate-x-0'}
        `}
            >
                <SessionList
                    sessions={chatManager.sessionHistory.data}
                    activeSessionId={activeSessionId() ?? null}
                    onSelectSession={handleSelectSession}
                    onCreateNewSession={handleCreateNewSession}
                    categories={[]}
                />
            </div>

            {/* 主要内容区域 */}
            <div class="flex-1 flex flex-col relative h-screen overflow-hidden w-full md:w-[calc(100%-16rem)]">
                {/* 顶部栏 */}
                <div class="h-14 border-b border-base-300/30 flex items-center px-4 gap-4">
                    {/* 移动端菜单按钮 */}
                    <button
                        class="md:hidden btn btn-sm btn-square btn-ghost"
                        onClick={toggleSidebar}
                    >
                        <MenuIcon />
                    </button>

                    {/* 标题 */}
                    <h1 class="text-xl font-medium">Fsy-Chat</h1>

                    <div class="flex-1"></div>

                    {/* 刷新按钮 */}
                    <button
                        class="btn btn-sm btn-square btn-ghost"
                        onClick={() => {
                            window.location.reload()
                        }}
                    >
                        <RefreshIcon />
                    </button>
                </div>

                {/* 聊天窗口 */}
                <ChatWindow
                    messages={chat.messages}
                    waiting_reply={chat.loading() || chatManager.createSession.isPending}
                    sseError={chat.sseError()}
                    isCreatingSession={chatManager.createSession.isPending}
                />

                {/* 聊天输入框 */}
                <ChatInput
                    onSendMessage={chat.sendMessage}
                    disabled={chat.loading() || chatManager.createSession.isPending}
                    activeSessionId={activeSessionId() ?? ''}
                    onStartChat={() => {
                        setShowCategories(true)
                    }}
                />
            </div>

            {/* 选择助手类型 */}
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
