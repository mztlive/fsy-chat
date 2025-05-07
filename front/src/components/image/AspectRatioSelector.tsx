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
}

const AspectRatioSelector = (props: AspectRatioSelectorProps) => {
    // 图片比例列表和选中状态
    const [aspectRatios] = createSignal<AspectRatio[]>([
        { id: '1:1', label: '1:1', ratio: '1:1', width: 1024, height: 1024 },
        { id: '16:9', label: '16:9', ratio: '16:9', width: 1440, height: 810 },
        { id: '3:2', label: '3:2', ratio: '3:2', width: 1440, height: 960 },
        { id: '4:3', label: '4:3', ratio: '4:3', width: 1440, height: 1080 },
        { id: '3:4', label: '3:4', ratio: '3:4', width: 1080, height: 1440 },
        { id: '2:3', label: '2:3', ratio: '2:3', width: 960, height: 1440 },
        { id: '9:16', label: '9:16', ratio: '9:16', width: 810, height: 1440 },
    ])
    const [selectedRatio, setSelectedRatio] = createSignal('1:1')

    const [isOpen, setIsOpen] = createSignal(false)

    const toggleMenu = () => setIsOpen(!isOpen())

    const handleSelect = (ratioId: string) => {
        setSelectedRatio(ratioId)
        props.onRatioSelect(aspectRatios().find(ratio => ratio.id === ratioId)!)
        setIsOpen(false)
    }

    return (
        <div class="dropdown dropdown-top">
            <button
                class="btn btn-sm bg-white text-base-content border border-gray-200 hover:bg-gray-100 normal-case flex items-center gap-1"
                onClick={toggleMenu}
            >
                <Show when={selectedRatio()}>
                    <div class="w-4 h-4 bg-gray-100 rounded-sm flex items-center justify-center">
                        <div
                            class="bg-gray-600"
                            style={{
                                width: selectedRatio().split(':')[0] + 'px',
                                height: selectedRatio().split(':')[1] + 'px',
                                'max-width': '80%',
                                'max-height': '80%',
                            }}
                        ></div>
                    </div>
                    <span>{selectedRatio()}</span>
                </Show>
                <UpIcon />
            </button>

            <Show when={isOpen()}>
                <div class="dropdown-content z-10 menu p-1 shadow-lg bg-white rounded-md w-52 border border-gray-200 mb-1">
                    <For each={aspectRatios()}>
                        {ratio => (
                            <li>
                                <button
                                    class={`${selectedRatio() === ratio.id ? 'bg-blue-50 text-blue-700' : 'text-gray-700 hover:bg-gray-50'}`}
                                    onClick={() => handleSelect(ratio.id)}
                                >
                                    <div class="w-4 h-4 bg-gray-100 rounded-sm flex items-center justify-center">
                                        <div
                                            class={`${selectedRatio() === ratio.id ? 'bg-blue-700' : 'bg-gray-600'}`}
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
