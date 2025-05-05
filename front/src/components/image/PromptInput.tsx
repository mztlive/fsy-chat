import { createSignal } from 'solid-js'

export interface PromptInputProps {
    prompt: string
    onPromptChange: (prompt: string) => void
    onGenerate: () => void
    disabled: boolean
}

export default function PromptInput(props: PromptInputProps) {
    // 处理按键事件，Enter键触发生成
    const handleKeyPress = (e: KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault()
            props.onGenerate()
        }
    }

    return (
        <div class="w-full mb-4">
            <div class="relative bg-white rounded-lg shadow-sm border border-gray-200">
                <textarea
                    class="w-full p-4 rounded-lg resize-none text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                    style="min-height: 80px"
                    placeholder="描述您想创建的图像，越详细越好..."
                    value={props.prompt}
                    onInput={e => props.onPromptChange(e.target.value)}
                    onKeyPress={handleKeyPress}
                    disabled={props.disabled}
                />

                <div class="p-2 border-t border-gray-100 flex justify-between items-center">
                    <div class="text-xs text-gray-400">{props.prompt.length} / 1000</div>

                    <div class="flex space-x-2">
                        <button
                            class="px-3 py-1.5 rounded-lg text-gray-600 hover:bg-gray-100 transition"
                            onClick={() => props.onPromptChange('')}
                            disabled={props.disabled || !props.prompt}
                        >
                            <svg
                                class="w-4 h-4"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                />
                            </svg>
                        </button>

                        <button
                            class={`px-4 py-1.5 rounded-lg font-medium text-white flex items-center ${
                                props.disabled
                                    ? 'bg-blue-300 cursor-not-allowed'
                                    : 'bg-blue-500 hover:bg-blue-600'
                            }`}
                            onClick={props.onGenerate}
                            disabled={props.disabled}
                        >
                            {props.disabled ? (
                                <>
                                    <div class="mr-2 animate-spin w-4 h-4 border-2 border-white/20 border-t-white rounded-full"></div>
                                    生成中...
                                </>
                            ) : (
                                <>立即生成</>
                            )}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}
