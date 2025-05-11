import { For, Show, createSignal } from 'solid-js'
import { GeneratedVideo } from '~/api/types'
import { Motion } from 'solid-motionone'
import GeneratedVideoComponent from './GeneratedVideo'
import { VideoIcon } from '../icons'
import VideoActionMenu from './VideoActionMenu'

export interface GeneratedVideoSet {
    videos: GeneratedVideo[]
    onVideoSelect: (videoId: string) => void
    loading: boolean
}

export default function GeneratedVideoSet(props: GeneratedVideoSet) {
    // 处理删除视频
    const handleDeleteVideo = (videoUrl: string) => {
        console.log('删除视频:', videoUrl)
        // 实际删除逻辑可以在这里实现
    }

    // 生成骨架屏数组
    const skeletonItems = () => Array(1).fill(0)

    return (
        <div class="w-full">
            {/* 显示视频内容或骨架屏 */}
            <Show when={props.videos.length > 0 || props.loading}>
                <div class="columns-1 sm:columns-2 lg:columns-3 gap-4 md:gap-6 space-y-4 md:space-y-6">
                    {/* 显示已有的视频列表 - 无论是否正在加载都显示 */}
                    <For each={props.videos}>
                        {video => (
                            <Motion.div
                                initial={{ opacity: 0, y: 20 }}
                                animate={{ opacity: 1, y: 0 }}
                                transition={{ duration: 0.3 }}
                                class="break-inside-avoid mb-4 md:mb-6"
                            >
                                <div class="card bg-base-100 shadow-sm hover:shadow-md transition-all duration-200 overflow-hidden h-auto relative">
                                    {/* 视频内容 */}
                                    <div class="w-full bg-base-200/30">
                                        <GeneratedVideoComponent url={video.url} />
                                    </div>

                                    {/* 操作按钮 - 位置调整到右上角，提高层级确保可点击 */}
                                    <div
                                        class="absolute top-2 right-2 z-30"
                                        onClick={e => e.stopPropagation()}
                                    >
                                        <VideoActionMenu
                                            url={video.url}
                                            onDelete={() => handleDeleteVideo(video.url)}
                                        />
                                    </div>

                                    {/* 视频信息 */}
                                    <div class="p-3">
                                        <div class="flex justify-between items-center mb-1">
                                            <p class="text-xs opacity-70 line-clamp-1 flex-1">
                                                {video.actual_prompt || '无提示词'}
                                            </p>
                                            <span class="badge badge-sm ml-2">
                                                {new Date(video.timestamp).toLocaleDateString()}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            </Motion.div>
                        )}
                    </For>

                    {/* 仅在loading时才显示骨架屏，作为新内容的占位 */}
                    <Show when={props.loading}>
                        <For each={skeletonItems()}>
                            {(_, index) => (
                                <Motion.div
                                    initial={{ opacity: 0, y: 20 }}
                                    animate={{ opacity: 1, y: 0 }}
                                    transition={{ duration: 0.3, delay: index() * 0.05 }}
                                    class="break-inside-avoid mb-4 md:mb-6"
                                >
                                    <div class="card bg-base-100 shadow-sm overflow-hidden h-auto">
                                        <div class="skeleton w-full aspect-video rounded-t-box"></div>
                                        <div class="p-3">
                                            <div class="skeleton h-4 w-4/5 mb-2"></div>
                                            <div class="skeleton h-3 w-1/2"></div>
                                        </div>
                                    </div>
                                </Motion.div>
                            )}
                        </For>
                    </Show>
                </div>
            </Show>

            {/* 空状态 */}
            <Show when={!props.loading && props.videos.length === 0}>
                <div class="flex flex-col items-center justify-center  text-base-content opacity-60">
                    <img src="/images/video_gen2.png" alt="empty-state" class="w-60 mb-4" />
                    <p class="text-sm mt-2">请输入提示词生成新的视频</p>
                </div>
            </Show>
        </div>
    )
}
