import { For, Show } from 'solid-js'
import ImageSkelon from '../common/ImageSkelon'
import { ImageActionButtons, AspectRatio, EmptyState } from '.'
import { GeneratedImage } from '~/api/types'
import GeneratedImageComponent from './GeneratedImage'

export interface GeneratedImageSetProps {
    images: GeneratedImage[]
    onImageSelect?: (imageId: string) => void
    loading: boolean
    aspectRatio: AspectRatio
}

export default function GeneratedImageSet(props: GeneratedImageSetProps) {
    return (
        <div>
            {/* 加载状态下显示骨架屏 */}
            <Show when={props.loading}>
                <div class="grid grid-cols-2 gap-4 mb-8">
                    {Array(4)
                        .fill(0)
                        .map((_, i) => (
                            <ImageSkelon aspectRatio={props.aspectRatio} />
                        ))}
                </div>
            </Show>

            <Show when={props.images.length === 0}>
                <EmptyState />
            </Show>

            {/* 显示现有图片 */}
            <Show when={props.images.length > 0}>
                <div class="flex flex-col gap-6">
                    <For each={props.images}>
                        {image => (
                            <div class="flex flex-col gap-6 w-full border-b border-gray-200 pb-4">
                                {/* 第一行：Prompt */}
                                <div class="text-sm">{image.actual_prompt || '无提示词'}</div>

                                {/* 第二行：图片网格 */}
                                <div class="grid grid-cols-2  gap-4">
                                    <For each={image.urls}>
                                        {url => (
                                            <GeneratedImageComponent
                                                url={url}
                                                aspectRatio={props.aspectRatio}
                                            />
                                        )}
                                    </For>
                                </div>
                            </div>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    )
}
