import { createSignal, ParentProps, Show } from 'solid-js'
import { SendIcon, TrashIcon, XIcon } from '../icons'
import { Motion } from 'solid-motionone'

export interface PromptInputProps extends ParentProps {
    onGenerate: (prompt: string) => void
    loading: boolean
}

export default function PromptInput(props: PromptInputProps) {
    const [prompt, setPrompt] = createSignal('')

    // 处理按键事件，Enter键触发生成
    const handleKeyPress = (e: KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault()
            props.onGenerate(prompt())
        }
    }

    return (
        <div class="w-full relative">
            <Motion.div
                class="card bg-base-100/90 shadow-sm border border-base-200"
                animate={{
                    width: props.loading ? '4rem' : '100%',
                    height: props.loading ? '4rem' : 'auto',
                    borderRadius: props.loading ? '9999px' : '0.5rem',
                    x: props.loading ? 'calc(50% - 2rem)' : '0%',
                    margin: props.loading ? '0' : '0',
                }}
                transition={{
                    duration: 0.4,
                    easing: 'ease-out',
                    opacity: { duration: 0.3 },
                }}
                style={{
                    'transform-origin': 'center',
                    'will-change': 'transform, width, height, border-radius',
                }}
            >
                <Show when={!props.loading}>
                    <textarea
                        class="textarea w-full p-4 resize-none text-sm bg-transparent border-none outline-none focus:outline-none focus:border-none focus:ring-0"
                        style="min-height: 80px"
                        placeholder="输入您的任意想法，越详细越好..."
                        value={prompt()}
                        onInput={e => setPrompt(e.target.value)}
                        onKeyPress={handleKeyPress}
                        disabled={props.loading}
                    />

                    <div class="card-actions p-3 border-t border-base-200 flex justify-between items-center">
                        <div class="flex items-center gap-2">
                            <div class="text-xs text-base-content/60">
                                <span class={prompt().length > 900 ? 'text-warning' : ''}>
                                    {prompt().length}
                                </span>{' '}
                                / 1000
                            </div>

                            {props.children}
                        </div>

                        <div class="flex space-x-2">
                            <button
                                class="btn btn-sm btn-ghost btn-square"
                                onClick={() => setPrompt('')}
                                disabled={props.loading || !prompt()}
                                title="清空输入"
                            >
                                <TrashIcon />
                            </button>

                            <button
                                class="btn btn-circle btn-sm btn-primary"
                                disabled={props.loading}
                                title="发送消息"
                                onClick={() => props.onGenerate(prompt())}
                            >
                                <SendIcon />
                            </button>
                        </div>
                    </div>
                </Show>

                <Show when={props.loading}>
                    <Motion.div
                        class="relative w-full h-full flex items-center justify-center"
                        initial={{ scale: 0, opacity: 0 }}
                        animate={{
                            scale: 1,
                            opacity: 1,
                            rotate: 360,
                        }}
                        transition={{
                            duration: 0.8,
                            rotate: {
                                repeat: Infinity,
                                duration: 1,
                                easing: 'linear',
                            },
                            scale: {
                                delay: 0.2,
                                duration: 0.2,
                            },
                            opacity: {
                                delay: 0.2,
                                duration: 0.2,
                            },
                        }}
                    >
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
                        <button class="absolute inset-0 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity duration-200">
                            <div class="bg-base-300/80 rounded-full p-1">
                                <XIcon class="w-5 h-5 text-base-content" />
                            </div>
                        </button>
                    </Motion.div>
                </Show>
            </Motion.div>
        </div>
    )
}
