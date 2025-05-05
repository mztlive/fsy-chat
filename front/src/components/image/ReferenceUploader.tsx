import { createSignal, Show } from 'solid-js'

export interface ReferenceUploaderProps {
    onImageUpload: (file: File) => void
}

export default function ReferenceUploader(props: ReferenceUploaderProps) {
    const [previewUrl, setPreviewUrl] = createSignal<string | null>(null)

    const handleFileChange = (e: Event) => {
        const input = e.target as HTMLInputElement
        if (input.files && input.files[0]) {
            const file = input.files[0]

            const reader = new FileReader()
            reader.onload = e => {
                setPreviewUrl(e.target?.result as string)
            }
            reader.readAsDataURL(file)

            props.onImageUpload(file)
        }
    }

    const handleDragOver = (e: DragEvent) => {
        e.preventDefault()
    }

    const handleDrop = (e: DragEvent) => {
        e.preventDefault()
        if (e.dataTransfer?.files && e.dataTransfer.files[0]) {
            const file = e.dataTransfer.files[0]

            const reader = new FileReader()
            reader.onload = e => {
                setPreviewUrl(e.target?.result as string)
            }
            reader.readAsDataURL(file)

            props.onImageUpload(file)
        }
    }

    return (
        <div class="mb-6">
            <h3 class="text-sm font-medium mb-3">导入参考图</h3>

            <Show
                when={previewUrl()}
                fallback={
                    <div
                        class="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center cursor-pointer hover:border-blue-500 transition"
                        onDragOver={handleDragOver}
                        onDrop={handleDrop}
                    >
                        <input
                            type="file"
                            accept="image/*"
                            class="hidden"
                            id="reference-image"
                            onChange={handleFileChange}
                        />
                        <label for="reference-image" class="cursor-pointer block">
                            <svg
                                class="w-10 h-10 mx-auto text-gray-400 mb-2"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.5"
                                    d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                />
                            </svg>
                            <span class="text-sm text-gray-500">点击或拖拽上传参考图</span>
                        </label>
                    </div>
                }
            >
                <div class="relative">
                    <img
                        src={previewUrl() || ''}
                        class="w-full h-auto rounded-lg object-cover"
                        style={{ 'max-height': '200px' }}
                    />
                    <button
                        class="absolute top-2 right-2 bg-black/50 hover:bg-black/70 text-white p-1 rounded-full"
                        onClick={() => setPreviewUrl(null)}
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                </div>
            </Show>
        </div>
    )
}
