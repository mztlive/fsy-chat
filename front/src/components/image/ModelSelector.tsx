import { For, createSignal } from 'solid-js'

export interface Model {
    id: string
    name: string
    version?: string
}

export interface ModelSelectorProps {
    models: Model[]
    selectedModel: string
    onModelSelect: (modelId: string) => void
}

export default function ModelSelector(props: ModelSelectorProps) {
    return (
        <div class="mb-6">
            <h3 class="text-sm font-medium mb-3">模型</h3>
            <div class="space-y-2">
                <For each={props.models}>
                    {model => (
                        <button
                            class={`w-full px-3 py-2 text-left rounded-lg transition ${
                                props.selectedModel === model.id
                                    ? 'bg-blue-50 text-blue-600 border border-blue-200'
                                    : 'hover:bg-gray-100 border border-transparent'
                            }`}
                            onClick={() => props.onModelSelect(model.id)}
                        >
                            <div class="flex items-center">
                                <span class="text-sm font-medium">{model.name}</span>
                                {model.version && (
                                    <span class="ml-2 text-xs text-gray-500">{model.version}</span>
                                )}
                            </div>
                        </button>
                    )}
                </For>
            </div>
        </div>
    )
}
