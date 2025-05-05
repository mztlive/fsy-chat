import { For, Show } from 'solid-js'
import ImageSkelon from '../common/ImageSkelon'

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
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {/* 加载状态下显示骨架屏 */}
                <Show when={props.loading}>
                    {Array(4)
                        .fill(0)
                        .map(() => (
                            <ImageSkelon />
                        ))}
                </Show>

                {/* 显示现有图片 */}
                <Show when={props.images.length > 0}>
                    <For each={props.images}>
                        {image => (
                            <div
                                class="relative aspect-square overflow-hidden rounded-lg cursor-pointer hover:shadow-lg transition group"
                                onClick={() => props.onImageSelect(image.id)}
                            >
                                <img
                                    src={image.url}
                                    alt="AI生成的图像"
                                    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                                    loading="lazy"
                                />
                                <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-all duration-200">
                                    <div class="absolute bottom-2 right-2 flex space-x-1">
                                        <button class="p-1.5 bg-white/90 rounded-full text-gray-700 hover:bg-white">
                                            <svg
                                                class="w-4 h-4"
                                                fill="none"
                                                stroke="currentColor"
                                                viewBox="0 0 24 24"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                                                />
                                            </svg>
                                        </button>
                                        <button class="p-1.5 bg-white/90 rounded-full text-gray-700 hover:bg-white">
                                            <svg
                                                class="w-4 h-4"
                                                fill="none"
                                                stroke="currentColor"
                                                viewBox="0 0 24 24"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                                                />
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                                <div class="absolute top-2 right-2 bg-black/40 backdrop-blur-sm text-white text-xs px-2 py-1 rounded-full">
                                    {new Date(image.timestamp).toLocaleDateString()}
                                </div>
                            </div>
                        )}
                    </For>
                </Show>
            </div>
        </div>
    )
}
