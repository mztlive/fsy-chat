import { For, createSignal, Show } from 'solid-js'
import { UpIcon } from '../icons'

export interface AspectRatio {
    id: string
    label: string
    ratio: string
    width: number
    height: number
    mode?: 'image' | 'video'
}

export interface AspectRatioSelectorProps {
    onRatioSelect: (ratio: AspectRatio) => void
    mode?: 'image' | 'video'
}

const AspectRatioSelector = (props: AspectRatioSelectorProps) => {
    // 图片比例列表和选中状态
    const [aspectRatios] = createSignal<AspectRatio[]>([
        // 图片模式
        { id: '1:1', label: '1:1', ratio: '1:1', width: 1024, height: 1024, mode: 'image' },
        { id: '16:9', label: '16:9', ratio: '16:9', width: 1440, height: 810, mode: 'image' },
        { id: '3:2', label: '3:2', ratio: '3:2', width: 1440, height: 960, mode: 'image' },
        { id: '4:3', label: '4:3', ratio: '4:3', width: 1440, height: 1080, mode: 'image' },
        { id: '3:4', label: '3:4', ratio: '3:4', width: 1080, height: 1440, mode: 'image' },
        { id: '2:3', label: '2:3', ratio: '2:3', width: 960, height: 1440, mode: 'image' },
        { id: '9:16', label: '9:16', ratio: '9:16', width: 810, height: 1440, mode: 'image' },
        // 视频模式
        {
            id: '1280x720',
            label: '1280x720',
            ratio: '16:9',
            width: 1280,
            height: 720,
            mode: 'video',
        },
        { id: '960x960', label: '960x960', ratio: '1:1', width: 960, height: 960, mode: 'video' },
        {
            id: '720x1280',
            label: '720x1280',
            ratio: '9:16',
            width: 720,
            height: 1280,
            mode: 'video',
        },
        {
            id: '1088x832',
            label: '1088x832',
            ratio: '4:3',
            width: 1088,
            height: 832,
            mode: 'video',
        },
        {
            id: '832x1088',
            label: '832x1088',
            ratio: '3:4',
            width: 832,
            height: 1088,
            mode: 'video',
        },
        { id: '832x480', label: '832x480', ratio: '16:9', width: 832, height: 480, mode: 'video' },
        { id: '624x624', label: '624x624', ratio: '1:1', width: 624, height: 624, mode: 'video' },
        { id: '480x832', label: '480x832', ratio: '9:16', width: 480, height: 832, mode: 'video' },
    ])
    const [selectedRatio, setSelectedRatio] = createSignal(
        props.mode === 'video' ? '1280x720' : '1:1'
    )

    const [isOpen, setIsOpen] = createSignal(false)

    const toggleMenu = () => setIsOpen(!isOpen())

    const handleSelect = (ratioId: string) => {
        setSelectedRatio(ratioId)
        props.onRatioSelect(aspectRatios().find(ratio => ratio.id === ratioId)!)
        setIsOpen(false)
    }

    // 根据mode过滤出对应模式的比例选项
    const filteredRatios = () => {
        return aspectRatios().filter(ratio => !props.mode || ratio.mode === props.mode)
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
                                width: selectedRatio().includes(':')
                                    ? selectedRatio().split(':')[0] + 'px'
                                    : '2px',
                                height: selectedRatio().includes(':')
                                    ? selectedRatio().split(':')[1] + 'px'
                                    : '2px',
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
                    <For each={filteredRatios()}>
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
