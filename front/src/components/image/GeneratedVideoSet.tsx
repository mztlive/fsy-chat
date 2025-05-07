import { For, Show } from 'solid-js'
import ImageSkelon from '../common/ImageSkelon'
import { ImageActionButtons } from '.'
import { GeneratedVideo } from '~/api/types'
import { Motion } from 'solid-motionone'
import GeneratedVideoComponent from './GeneratedVideo'

export interface GeneratedVideoSet {
    videos: GeneratedVideo[]
    onVideoSelect: (videoId: string) => void
    loading: boolean
}

export default function GeneratedVideoSet(props: GeneratedVideoSet) {
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
            <Show when={props.videos.length > 0}>
                <div class="flex flex-col gap-6">
                    <For each={props.videos}>
                        {video => (
                            <div class="flex flex-col gap-2 w-full border-b border-gray-200 pb-4">
                                {/* 第一行：Prompt */}
                                <div class="text-xs text-gray-400">
                                    {video.actual_prompt || '无提示词'}
                                </div>

                                {/* 第二行：图片网格 */}
                                <GeneratedVideoComponent url={video.url} />
                            </div>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    )
}
