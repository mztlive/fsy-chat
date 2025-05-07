import { For, Show } from 'solid-js'
import ImageSkelon from '../common/ImageSkelon'
import { ImageActionButtons } from '.'
import { GeneratedImage } from '~/api/types'
import { Motion } from 'solid-motionone'
import GeneratedImageComponent from './GeneratedImage'

export interface GeneratedImageSet {
    images: GeneratedImage[]
    onImageSelect: (imageId: string) => void
    loading: boolean
}

export default function GeneratedImageSet(props: GeneratedImageSet) {
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
                                        {url => <GeneratedImageComponent url={url} />}
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
