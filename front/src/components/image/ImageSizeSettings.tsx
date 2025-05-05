import { createSignal } from 'solid-js'

export interface Resolution {
    id: string
    label: string
}

export interface ImageSizeSettingsProps {
    resolutions: Resolution[]
    selectedResolution: string
    width: number
    height: number
    onResolutionSelect: (resolutionId: string) => void
    onSizeChange: (width: number, height: number) => void
}

export default function ImageSizeSettings(props: ImageSizeSettingsProps) {
    return (
        <div class="mb-6">
            <h3 class="text-sm font-medium mb-3">图片尺寸</h3>

            <div class="flex space-x-2 mb-4">
                {props.resolutions.map(resolution => (
                    <button
                        class={`px-3 py-1 text-sm border rounded transition ${
                            props.selectedResolution === resolution.id
                                ? 'border-blue-500 bg-blue-50 text-blue-600'
                                : 'border-gray-200 hover:border-gray-300'
                        }`}
                        onClick={() => props.onResolutionSelect(resolution.id)}
                    >
                        {resolution.label}
                    </button>
                ))}
            </div>

            <div class="flex items-center space-x-2">
                <div class="flex-1">
                    <label class="text-xs text-gray-500 mb-1 block">宽度 (W)</label>
                    <div class="flex items-center">
                        <input
                            type="number"
                            class="w-full p-2 border border-gray-200 rounded text-sm"
                            value={props.width}
                            onInput={e =>
                                props.onSizeChange(parseInt(e.target.value), props.height)
                            }
                        />
                    </div>
                </div>

                <div class="flex items-center justify-center h-8">
                    <div class="w-5 h-px bg-gray-300"></div>
                </div>

                <div class="flex-1">
                    <label class="text-xs text-gray-500 mb-1 block">高度 (H)</label>
                    <div class="flex items-center">
                        <input
                            type="number"
                            class="w-full p-2 border border-gray-200 rounded text-sm"
                            value={props.height}
                            onInput={e => props.onSizeChange(props.width, parseInt(e.target.value))}
                        />
                    </div>
                </div>
            </div>
        </div>
    )
}
