import { For, Show } from 'solid-js'

export interface GeneratedImage {
    id: string
    url: any
    timestamp: number
}

export interface GeneratedImagesProps {
    images: GeneratedImage[]
    onImageSelect: (imageId: string) => void
    loading: boolean
}

export default function GeneratedImages(props: GeneratedImagesProps) {
    return (
        <div>
            <Show
                when={!props.loading}
                fallback={
                    <div class="flex justify-center p-10">
                        <div class="animate-spin w-10 h-10 border-4 border-blue-500 border-t-transparent rounded-full"></div>
                    </div>
                }
            >
                <div class="grid grid-cols-4 gap-4">
                    <For each={props.images}>
                        {image => (
                            <div
                                class="relative overflow-hidden rounded-lg cursor-pointer hover:shadow-lg transition"
                                onClick={() => props.onImageSelect(image.id)}
                            >
                                <img
                                    src={image.url}
                                    alt="AI生成的图像"
                                    class="w-full h-full object-cover"
                                    loading="lazy"
                                />
                                <div class="absolute inset-0 bg-black/0 hover:bg-black/20 transition-all duration-200"></div>
                            </div>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    )
}
