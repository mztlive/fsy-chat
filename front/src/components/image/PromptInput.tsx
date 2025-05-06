import { createSignal, Match, Switch } from 'solid-js'
import AspectRatioSelector, { AspectRatio } from './AspectRatioSelector'
import GenerateButton from './GenerateButton'
import { TrashIcon } from '../icons'

export interface PromptInputProps {
    prompt: string
    onPromptChange: (prompt: string) => void
    onGenerate: () => void
    loading: boolean
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
            <div class="card bg-base-100/80 shadow-sm border border-base-200">
                <textarea
                    class="textarea w-full p-4 resize-none text-sm bg-transparent border-none outline-none focus:outline-none focus:border-none focus:ring-0"
                    style="min-height: 80px"
                    placeholder="详细描述您想要的图像，越具体越好..."
                    value={props.prompt}
                    onInput={e => props.onPromptChange(e.target.value)}
                    onKeyPress={handleKeyPress}
                    disabled={props.loading}
                />

                <div class="card-actions p-3 border-t border-base-200 flex justify-between items-center">
                    <div class="flex items-center gap-2">
                        <div class="text-xs text-base-content/60">
                            <span class={props.prompt.length > 900 ? 'text-warning' : ''}>
                                {props.prompt.length}
                            </span>{' '}
                            / 1000
                        </div>
                        <AspectRatioSelector onRatioSelect={setSelectedRatio} />
                    </div>

                    <div class="flex space-x-2">
                        <button
                            class="btn btn-sm btn-ghost btn-square"
                            onClick={() => props.onPromptChange('')}
                            disabled={props.loading || !props.prompt}
                            title="清空输入"
                        >
                            <TrashIcon />
                        </button>

                        <GenerateButton disabled={props.loading} onGenerate={props.onGenerate} />
                    </div>
                </div>
            </div>
        </div>
    )
}
