import { createFileRoute } from '@tanstack/solid-router'
import { createEffect, createSignal, onMount } from 'solid-js'
import { ChatInput } from '../components/ChatInput'
import { ChatWindow } from '../components/ChatWindow'
import { SessionList } from '../components/SessionList'
import { MenuIcon, RefreshIcon, MoreIcon } from '../components/icons'
import { useChat } from '~/hooks/chat'
import { useChatManager } from '~/hooks/chat_manager'

export const Route = createFileRoute('/chat')({
    component: ChatRoute,
})

function ChatRoute() {
    const [sidebarOpen, setSidebarOpen] = createSignal(false)
    const [activeSessionId, setActiveSessionId] = createSignal<string | null>(null)

    const chat = useChat()
    const chatManager = useChatManager()

    // 切换侧边栏
    const toggleSidebar = () => {
        setSidebarOpen(!sidebarOpen())
    }

    // 加载会话数据和初始化
    onMount(async () => {
        chat.connect(await chatManager.createSession())
    })

    // 处理创建新会话
    const handleCreateNewSession = async (category?: string) => {
        // 在移动端创建新会话后关闭侧边栏
        setSidebarOpen(false)
    }

    // 处理选择会话
    const handleSelectSession = async (sessionId: string) => {
        // 在移动端选择会话后关闭侧边栏
        setSidebarOpen(false)
        setActiveSessionId(sessionId)
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
                    sessions={chatManager.sessionHistory()}
                    activeSessionId={activeSessionId()}
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
                    <button class="btn btn-sm btn-square btn-ghost">
                        <RefreshIcon />
                    </button>

                    {/* 更多按钮 */}
                    <button class="btn btn-sm btn-square btn-ghost">
                        <MoreIcon />
                    </button>
                </div>

                {/* 聊天窗口 */}
                <ChatWindow messages={chat.messages} loading={chat.loading()} />

                {/* 聊天输入框 */}
                <ChatInput onSendMessage={chat.sendMessage} disabled={chat.loading()} />
            </div>
        </div>
    )
}
