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
        <div class="flex flex-wrap gap-2">
            <For each={props.ratios}>
                {ratio => (
                    <button
                        class={`flex flex-col items-center rounded-lg transition ${
                            props.selectedRatio === ratio.id
                                ? 'bg-blue-50 border-2 border-blue-500 text-blue-700'
                                : 'bg-gray-50 border border-gray-200 text-gray-600 hover:border-gray-300'
                        } px-4 py-2`}
                        onClick={() => props.onRatioSelect(ratio.id)}
                    >
                        <div class="flex items-center gap-2">
                            <div class="w-6 h-6 bg-gray-200 rounded flex items-center justify-center">
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
                            <span class="text-sm font-medium">{ratio.label}</span>
                        </div>
                    </button>
                )}
            </For>
        </div>
    )
}
