import { For, createSignal, Show } from 'solid-js'
import { UpIcon } from '../icons'

export interface AspectRatio {
    id: string
    label: string
    ratio: string
    width: number
    height: number
}

export interface AspectRatioSelectorProps {
    onRatioSelect: (ratio: AspectRatio) => void
    aspectRatios: AspectRatio[]
    selectedRatio: AspectRatio
}

const AspectRatioSelector = (props: AspectRatioSelectorProps) => {
    const [isOpen, setIsOpen] = createSignal(false)

    const toggleMenu = () => setIsOpen(!isOpen())

    const handleSelect = (ratioId: string) => {
        props.onRatioSelect(props.aspectRatios.find(ratio => ratio.id === ratioId)!)
        setIsOpen(false)
    }

    return (
        <div class="dropdown dropdown-top">
            <button
                class="btn btn-sm bg-white text-base-content border border-gray-200 hover:bg-gray-100 normal-case flex items-center gap-1"
                onClick={toggleMenu}
            >
                <Show when={props.selectedRatio}>
                    <div class="w-4 h-4 bg-gray-100 rounded-sm flex items-center justify-center">
                        <div
                            class="bg-gray-600"
                            style={{
                                width: props.selectedRatio.ratio.includes(':')
                                    ? props.selectedRatio.ratio.split(':')[0] + 'px'
                                    : '2px',
                                height: props.selectedRatio.ratio.includes(':')
                                    ? props.selectedRatio.ratio.split(':')[1] + 'px'
                                    : '2px',
                                'max-width': '80%',
                                'max-height': '80%',
                            }}
                        ></div>
                    </div>
                    <span>{props.selectedRatio.label}</span>
                </Show>
                <UpIcon />
            </button>

            <Show when={isOpen()}>
                <div class="dropdown-content z-10 menu p-1 shadow-lg bg-white rounded-md w-52 border border-gray-200 mb-1">
                    <For each={props.aspectRatios}>
                        {ratio => (
                            <li>
                                <button
                                    class={`${props.selectedRatio.id === ratio.id ? 'bg-blue-50 text-blue-700' : 'text-gray-700 hover:bg-gray-50'}`}
                                    onClick={() => handleSelect(ratio.id)}
                                >
                                    <div class="w-4 h-4 bg-gray-100 rounded-sm flex items-center justify-center">
                                        <div
                                            class={`${props.selectedRatio.id === ratio.id ? 'bg-blue-700' : 'bg-gray-600'}`}
                                            style={{
                                                width: ratio.ratio.split(':')[0] + 'px',
                                                height: ratio.ratio.split(':')[1] + 'px',
                                                'max-width': '80%',
                                                'max-height': '80%',
                                            }}
                                        ></div>
                                    </div>
                                    <span>{ratio.label}</span>
                                </button>
                            </li>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    )
}

export default AspectRatioSelector
