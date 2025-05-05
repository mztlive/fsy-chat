import { createSignal } from 'solid-js'
import AspectRatioSelector, { AspectRatio } from './AspectRatioSelector'

export interface PromptInputProps {
    prompt: string
    onPromptChange: (prompt: string) => void
    onGenerate: () => void
    disabled: boolean
}

export default function PromptInput(props: PromptInputProps) {
    // 图片比例列表和选中状态
    const [aspectRatios] = createSignal<AspectRatio[]>([
        { id: '1:1', label: '1:1', ratio: '1:1' },
        { id: '16:9', label: '16:9', ratio: '16:9' },
        { id: '3:2', label: '3:2', ratio: '3:2' },
        { id: '4:3', label: '4:3', ratio: '4:3' },
        { id: '3:4', label: '3:4', ratio: '3:4' },
        { id: '2:3', label: '2:3', ratio: '2:3' },
        { id: '9:16', label: '9:16', ratio: '9:16' },
    ])
    const [selectedRatio, setSelectedRatio] = createSignal('1:1')

    // 处理按键事件，Enter键触发生成
    const handleKeyPress = (e: KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault()
            props.onGenerate()
        }
    }

    return (
        <div class="w-full">
            <div class="relative bg-white/60 rounded-lg border border-gray-200 shadow-sm backdrop-blur-md focus-within:border-blue-300 focus-within:ring-1 focus-within:ring-blue-300">
                <textarea
                    class="w-full p-4 rounded-lg resize-none text-sm focus:outline-none bg-transparent"
                    style="min-height: 80px"
                    placeholder="详细描述您想要的图像，越具体越好..."
                    value={props.prompt}
                    onInput={e => props.onPromptChange(e.target.value)}
                    onKeyPress={handleKeyPress}
                    disabled={props.disabled}
                />

                <div class="p-3 border-t border-gray-100 flex justify-between items-center">
                    <div class="text-xs text-gray-400">
                        <span class={props.prompt.length > 900 ? 'text-orange-500' : ''}>
                            {props.prompt.length}
                        </span>{' '}
                        / 1000
                    </div>
                    <AspectRatioSelector
                        ratios={aspectRatios()}
                        selectedRatio={selectedRatio()}
                        onRatioSelect={setSelectedRatio}
                    />

                    <div class="flex space-x-2">
                        <button
                            class="px-3 py-1.5 rounded-lg text-gray-600 hover:bg-gray-100 transition disabled:opacity-50"
                            onClick={() => props.onPromptChange('')}
                            disabled={props.disabled || !props.prompt}
                            title="清空输入"
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
                            class={`px-5 py-1.5 rounded-lg font-medium text-white flex items-center ${
                                props.disabled
                                    ? 'bg-blue-400 cursor-not-allowed'
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
                                <>
                                    <svg
                                        class="w-4 h-4 mr-1"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M13 10V3L4 14h7v7l9-11h-7z"
                                        />
                                    </svg>
                                    立即生成
                                </>
                            )}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}
