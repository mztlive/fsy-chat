import { For, Show } from 'solid-js'
import ImageSkelon from '../common/ImageSkelon'
import { ImageActionButtons } from './'
import { GeneratedImage } from '~/api/types'
import { Motion } from 'solid-motionone'

export interface GeneratedImagesProps {
    images: GeneratedImage[]
    onImageSelect: (imageId: string) => void
    loading: boolean
}

export default function GeneratedImages(props: GeneratedImagesProps) {
    return (
        <div>
            {/* 加载状态下显示骨架屏 */}
            <Show when={props.loading}>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 mb-8">
                    {Array(4)
                        .fill(0)
                        .map(() => (
                            <ImageSkelon />
                        ))}
                </div>
            </Show>

            {/* 显示现有图片 */}
            <Show when={props.images.length > 0}>
                <div class="flex flex-col gap-6">
                    <For each={props.images}>
                        {image => (
                            <div class="flex flex-col gap-2 w-full border-b border-gray-200 pb-4">
                                {/* 第一行：Prompt */}
                                <div class="text-xs text-gray-400">
                                    {image.actual_prompt || '无提示词'}
                                </div>

                                {/* 第二行：图片网格 */}
                                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                                    <For each={image.urls}>
                                        {url => (
                                            <Motion.div
                                                class="relative aspect-square overflow-hidden rounded-lg cursor-pointer hover:shadow-lg transition group"
                                                initial={{ scale: 0.95, opacity: 0.5 }}
                                                animate={{ scale: 1, opacity: 1 }}
                                                transition={{ duration: 0.3 }}
                                            >
                                                <img
                                                    src={url}
                                                    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                                                    loading="lazy"
                                                />
                                                <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-all duration-200">
                                                    <div class="absolute bottom-2 right-2 flex space-x-1">
                                                        <ImageActionButtons
                                                            onDownload={() =>
                                                                console.log('下载图片')
                                                            }
                                                            onCopy={() => console.log('复制图片')}
                                                        />
                                                    </div>
                                                </div>
                                            </Motion.div>
                                        )}
                                    </For>
                                </div>

                                {/* 第三行：时间戳 */}
                                <div class="text-xs text-gray-500 mt-1">
                                    {new Date(image.timestamp).toLocaleDateString()}
                                </div>
                            </div>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    )
}
