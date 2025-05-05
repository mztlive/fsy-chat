import { For } from 'solid-js'

export interface AspectRatio {
    id: string
    label: string
    ratio: string
}

export interface AspectRatioSelectorProps {
    ratios: AspectRatio[]
    selectedRatio: string
    onRatioSelect: (ratioId: string) => void
}

const AspectRatioSelector = (props: AspectRatioSelectorProps) => {
    return (
        <div class="flex flex-wrap gap-1">
            <For each={props.ratios}>
                {ratio => (
                    <button
                        class={`flex flex-col items-center rounded-md transition ${
                            props.selectedRatio === ratio.id
                                ? 'bg-blue-50 border border-blue-500 text-blue-700'
                                : 'bg-gray-50 border border-gray-200 text-gray-600 hover:border-gray-300'
                        } px-2 py-1`}
                        onClick={() => props.onRatioSelect(ratio.id)}
                    >
                        <div class="flex items-center gap-1">
                            <div class="w-4 h-4 bg-gray-200 rounded flex items-center justify-center">
                                <div
                                    class="bg-gray-500"
                                    style={{
                                        width: ratio.ratio.split(':')[0] + 'px',
                                        height: ratio.ratio.split(':')[1] + 'px',
                                        'max-width': '80%',
                                        'max-height': '80%',
                                    }}
                                ></div>
                            </div>
                            <span class="text-xs font-medium">{ratio.label}</span>
                        </div>
                    </button>
                )}
            </For>
        </div>
    )
}

export default AspectRatioSelector
