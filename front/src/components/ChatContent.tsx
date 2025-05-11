import { Message } from '~/types/chat'
import { ChatWindow } from './ChatWindow'
import { RefreshIcon } from './icons'
import { MenuIcon } from './icons'
import { PromptInput } from './image'
import { Show } from 'solid-js'

// 头部组件接口
interface ChatHeaderProps {
    onToggleSidebar: () => void
}

// 头部组件
const ChatHeader = (props: ChatHeaderProps) => {
    return (
        <header class="h-14 border-b border-base-300/30 flex items-center px-4 gap-4">
            {/* 移动端菜单按钮 */}
            <button
                class="md:hidden btn btn-sm btn-square btn-ghost"
                onClick={props.onToggleSidebar}
            >
                <MenuIcon />
            </button>

            {/* 标题 */}
            <h1 class="text-xl font-medium">Fsy AI Assistant</h1>

            <div class="flex-1"></div>

            {/* 刷新按钮 */}
            {/* <button
                class="btn btn-sm btn-square btn-ghost"
                onClick={() => {
                    window.location.reload()
                }}
            >
                <RefreshIcon />
            </button> */}
        </header>
    )
}

// 底部输入组件接口
interface ChatFooterProps {
    activeSessionId?: string
    onSendMessage: (text: string) => void
    loading: boolean
    onStartChat: () => void
}

// 底部输入组件
const ChatFooter = (props: ChatFooterProps) => {
    return (
        <footer class="p-4">
            <Show when={props.activeSessionId}>
                <PromptInput onGenerate={props.onSendMessage} loading={props.loading} />
            </Show>

            <Show when={!props.activeSessionId}>
                <button
                    class="btn btn-primary w-full max-w-3xl mx-auto block"
                    onClick={props.onStartChat}
                >
                    开始聊天
                </button>
            </Show>
        </footer>
    )
}

// 主内容区域组件接口
interface ChatContentProps {
    messages: Message[]
    loading: boolean
    sseError: string | null
    isCreatingSession: boolean
    activeSessionId?: string
    onSendMessage: (text: string) => void
    onStartChat: () => void
    onToggleSidebar: () => void
}

// 主内容区域组件
const ChatContent = (props: ChatContentProps) => {
    return (
        <div class="bg-base-100 grid grid-rows-[auto_1fr_auto] h-screen overflow-hidden">
            <ChatHeader onToggleSidebar={props.onToggleSidebar} />

            {/* 聊天窗口 */}
            <main class="overflow-auto">
                <ChatWindow
                    messages={props.messages}
                    waiting_reply={props.loading || props.isCreatingSession}
                    sseError={props.sseError}
                    isCreatingSession={props.isCreatingSession}
                />
            </main>

            <ChatFooter
                activeSessionId={props.activeSessionId}
                onSendMessage={props.onSendMessage}
                loading={props.loading}
                onStartChat={props.onStartChat}
            />
        </div>
    )
}

export default ChatContent
