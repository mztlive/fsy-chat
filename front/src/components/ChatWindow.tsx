import { For, Show, createEffect, onMount } from 'solid-js'
import { Message } from '../types/chat'
import { ChatMessage } from './ChatMessage'
import { DocumentIcon, PencilIcon, QuestionIcon, AlignIcon } from './icons'
import { Motion } from 'solid-motionone'
import WindowErrorAlert from './WindowErrorAlert'

interface ChatWindowProps {
    messages: Message[]
    waiting_reply?: boolean
    sseError: string
    isCreatingSession: boolean
}

export function ChatWindow(props: ChatWindowProps) {
    let messagesEndRef: HTMLDivElement | undefined

    // 滚动到最新消息
    const scrollToBottom = () => {
        messagesEndRef?.scrollIntoView({ behavior: 'smooth' })
    }

    // 当消息列表变化或加载状态变化时，滚动到底部
    createEffect(() => {
        if (props.messages.length || props.waiting_reply) {
            setTimeout(scrollToBottom, 100)
        }
    })

    // 组件挂载时滚动到底部
    onMount(() => {
        scrollToBottom()
    })

    return (
        <div class="flex-1 overflow-y-auto overflow-x-hidden">
            <Show when={props.sseError}>
                <WindowErrorAlert sseError={props.sseError} />
            </Show>

            <Show when={props.isCreatingSession}>
                <Motion.div class="fixed inset-0 bg-base-100/50 backdrop-blur-sm z-10 flex items-center justify-center">
                    <div role="alert" class="alert bg-base-100 shadow-lg max-w-md mx-auto">
                        <span class="loading loading-spinner loading-md"></span>
                        <span>正在创建会话...</span>
                    </div>
                </Motion.div>
            </Show>
            <Show
                when={props.messages.length > 0 && !props.isCreatingSession}
                fallback={
                    <Motion.div
                        class="flex flex-col items-center justify-center h-full text-center p-4"
                        animate={{
                            opacity: [0, 1],
                            transform: ['translateY(10px)', 'translateY(0)'],
                        }}
                        transition={{ duration: 1, easing: 'ease-in-out' }}
                    >
                        <div class="text-2xl font-medium mb-8">我可以帮你</div>
                        <div class="grid grid-cols-2 gap-3 max-w-md w-full">
                            <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                                <span class="text-primary flex-shrink-0">
                                    <DocumentIcon size={24} />
                                </span>
                                <span class="text-sm">总结文本</span>
                            </button>
                            <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                                <span class="text-secondary flex-shrink-0">
                                    <PencilIcon size={24} />
                                </span>
                                <span class="text-sm">写文章</span>
                            </button>
                            <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                                <span class="text-info flex-shrink-0">
                                    <QuestionIcon size={24} />
                                </span>
                                <span class="text-sm">提供建议</span>
                            </button>
                            <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                                <span class="text-accent flex-shrink-0">
                                    <AlignIcon size={24} />
                                </span>
                                <span class="text-sm">了解公司业务</span>
                            </button>
                        </div>
                    </Motion.div>
                }
            >
                <div class="pb-32 pt-4">
                    <For each={props.messages}>
                        {message => (
                            <Motion.div
                                animate={{
                                    opacity: [0, 1],
                                    transform: ['translateX(-10px)', 'translateY(0)'],
                                }}
                                transition={{ duration: 0.5, easing: 'ease-in-out' }}
                                class={`py-4 ${message.role === 'assistant' ? 'bg-base-100' : 'bg-base-100'}`}
                            >
                                <div class="max-w-3xl mx-auto px-4 md:px-8 w-full overflow-hidden">
                                    <ChatMessage message={message} />
                                </div>
                            </Motion.div>
                        )}
                    </For>

                    <Show when={props.waiting_reply}>
                        <div id="loading" class="py-4 bg-base-100">
                            <div class="max-w-3xl mx-auto px-4 md:px-8 w-full overflow-hidden">
                                <div class="chat chat-start">
                                    <div class="chat-bubble bg-base-200 text-base-content">
                                        <span class="loading loading-dots loading-md"></span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Show>

                    <div ref={messagesEndRef}></div>
                </div>
            </Show>
        </div>
    )
}
