import { For, Show, createSignal } from 'solid-js'
import { GeneratedVideo } from '~/api/types'
import { Motion } from 'solid-motionone'
import GeneratedVideoComponent from './GeneratedVideo'
import { CopyIcon, DownloadIcon, TrashIcon, VideoIcon } from '../icons'

export interface GeneratedVideoSet {
    videos: GeneratedVideo[]
    onVideoSelect: (videoId: string) => void
    loading: boolean
}

export default function GeneratedVideoSet(props: GeneratedVideoSet) {
    return (
        <div class="w-full">
            {/* 加载状态下显示骨架屏 */}
            <Show when={props.loading}>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                    {Array(6)
                        .fill(0)
                        .map(() => (
                            <div class="card bg-base-100">
                                <div class="skeleton w-full aspect-video rounded-t-box"></div>
                                <div class="p-3">
                                    <div class="skeleton h-4 w-4/5 mb-2"></div>
                                    <div class="skeleton h-3 w-1/2"></div>
                                </div>
                            </div>
                        ))}
                </div>
            </Show>

            {/* 显示视频内容 */}
            <Show when={props.videos.length > 0}>
                <div class="columns-1 sm:columns-2 lg:columns-3 gap-4 md:gap-6 space-y-4 md:space-y-6">
                    <For each={props.videos}>
                        {video => (
                            <div class="break-inside-avoid mb-4 md:mb-6">
                                <Motion.div
                                    initial={{ opacity: 0, y: 20 }}
                                    animate={{ opacity: 1, y: 0 }}
                                    transition={{ duration: 0.3 }}
                                    class="card bg-base-100 shadow-sm hover:shadow-md transition-all duration-200 overflow-hidden h-auto"
                                >
                                    {/* 视频内容 */}
                                    <div class="relative group">
                                        <div class="w-full bg-base-200/30">
                                            <GeneratedVideoComponent url={video.url} />
                                        </div>

                                        {/* 悬浮操作 */}
                                        <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                                            <div class="dropdown dropdown-end">
                                                <div
                                                    tabindex="0"
                                                    role="button"
                                                    class="btn btn-sm btn-circle btn-ghost bg-base-100/80 backdrop-blur-sm"
                                                >
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        class="h-4 w-4"
                                                        fill="none"
                                                        viewBox="0 0 24 24"
                                                        stroke="currentColor"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"
                                                        />
                                                    </svg>
                                                </div>
                                                <ul
                                                    tabindex="0"
                                                    class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-32"
                                                >
                                                    <li>
                                                        <a>
                                                            <DownloadIcon class="h-4 w-4" />
                                                            <span>下载</span>
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a>
                                                            <CopyIcon class="h-4 w-4" />
                                                            <span>复制</span>
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a class="text-error">
                                                            <TrashIcon class="h-4 w-4" />
                                                            <span>删除</span>
                                                        </a>
                                                    </li>
                                                </ul>
                                            </div>
                                        </div>
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
                                </Motion.div>
                            </div>
                        )}
                    </For>
                </div>
            </Show>

            {/* 空状态 */}
            <Show when={!props.loading && props.videos.length === 0}>
                <div class="flex flex-col items-center justify-center py-16 text-base-content opacity-60">
                    <VideoIcon class="h-16 w-16 mb-4" />
                    <p class="text-lg font-medium">暂无视频内容</p>
                    <p class="text-sm mt-2">请输入提示词生成新的视频</p>
                </div>
            </Show>
        </div>
    )
}
