import { createSignal, Match, Switch } from 'solid-js'
import AspectRatioSelector, { AspectRatio } from './AspectRatioSelector'
import GenerateButton from './GenerateButton'
import { TrashIcon } from '../icons'

export interface PromptInputProps {
    prompt: string
    onPromptChange: (prompt: string) => void
    onGenerate: () => void
    disabled: boolean
}

export default function PromptInput(props: PromptInputProps) {
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
            <div class="relative bg-white/60 rounded-lg border border-gray-200 shadow-sm">
                <textarea
                    class="w-full p-4 rounded resize-none text-sm focus:outline-none bg-transparent"
                    style="min-height: 80px"
                    placeholder="详细描述您想要的图像，越具体越好..."
                    value={props.prompt}
                    onInput={e => props.onPromptChange(e.target.value)}
                    onKeyPress={handleKeyPress}
                    disabled={props.disabled}
                />

                <div class="p-3 border-t border-gray-100 flex justify-between items-center">
                    <div class="flex items-center gap-2">
                        <div class="text-xs text-gray-400">
                            <span class={props.prompt.length > 900 ? 'text-orange-500' : ''}>
                                {props.prompt.length}
                            </span>{' '}
                            / 1000
                        </div>
                        <AspectRatioSelector onRatioSelect={setSelectedRatio} />
                    </div>

                    <div class="flex space-x-2">
                        <button
                            class="px-3 py-1.5 rounded-lg text-gray-600 hover:bg-gray-100 transition disabled:opacity-50"
                            onClick={() => props.onPromptChange('')}
                            disabled={props.disabled || !props.prompt}
                            title="清空输入"
                        >
                            <TrashIcon />
                        </button>

                        <GenerateButton disabled={props.disabled} onGenerate={props.onGenerate} />
                    </div>
                </div>
            </div>
        </div>
    )
}
