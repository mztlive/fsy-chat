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

export default function AspectRatioSelector(props: AspectRatioSelectorProps) {
    return (
        <div class="mb-6">
            <h3 class="text-sm font-medium mb-3">图片比例</h3>
            <div class="grid grid-cols-4 gap-2">
                <For each={props.ratios}>
                    {ratio => (
                        <button
                            class={`p-2 flex flex-col items-center justify-center border rounded transition ${
                                props.selectedRatio === ratio.id
                                    ? 'border-blue-500 bg-blue-50 text-blue-600'
                                    : 'border-gray-200 hover:border-gray-300'
                            }`}
                            onClick={() => props.onRatioSelect(ratio.id)}
                        >
                            <div class="w-10 h-8 bg-gray-200 rounded flex items-center justify-center mb-1">
                                <div
                                    class="bg-gray-400"
                                    style={{
                                        width: ratio.ratio.split(':')[0] + 'px',
                                        height: ratio.ratio.split(':')[1] + 'px',
                                        'max-width': '90%',
                                        'max-height': '90%',
                                    }}
                                ></div>
                            </div>
                            <span class="text-xs">{ratio.label}</span>
                        </button>
                    )}
                </For>
            </div>
        </div>
    )
}
