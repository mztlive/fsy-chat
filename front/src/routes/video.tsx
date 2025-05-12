import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show, For } from 'solid-js'
import { AspectRatio } from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import GeneratedVideoSet from '~/components/image/GeneratedVideoSet'
import { useVideoGenerate } from '~/hooks/video_generate'
import { PencilIcon } from '~/components/icons'
import { VideoAspectRatios } from '~/components/image/AspectRatioSelector'

export const Route = createFileRoute('/video')({
    component: VideoRoute,
})

function VideoRoute() {
    const [ratio, setRatio] = createSignal<AspectRatio>(VideoAspectRatios[0])
    const [prompt, setPrompt] = createSignal('')

    const [isSmartRewrite, setIsSmartRewrite] = createSignal(false)

    const { generatedVideos, createVideo, error, isPending } = useVideoGenerate()

    // 选择视频
    const handleVideoSelect = (videoId: string) => {
        console.log('选择视频:', videoId)
        // 实际选择处理...
    }

    // 生成视频
    const handleGenerate = async () => {
        if (!prompt().trim()) {
            console.warn('Prompt is empty, generation skipped.')
            return
        }
        await createVideo(prompt(), ratio().width, ratio().height, isSmartRewrite())
    }

    const LeftPanel = () => (
        <div class="w-[360px] flex-shrink-0 bg-white p-6 flex flex-col space-y-4 h-full overflow-y-auto rounded-lg">
            <h1 class="text-xl font-semibold text-gray-800">文字生成视频</h1>

            <div class="form-control">
                <textarea
                    class="textarea w-full h-32 resize-none text-sm leading-relaxed focus:outline-none"
                    placeholder="输入视频描述，例如：一只可爱的猫咪在草地上奔跑"
                    value={prompt()}
                    onInput={e => setPrompt(e.currentTarget.value)}
                />
                <div class="flex justify-end mt-2">
                    <div class="text-xs text-gray-400">{prompt().length}/800</div>
                </div>
            </div>

            <div class="space-y-2">
                <label class="text-sm font-medium text-gray-800">比例</label>
                <div class="grid grid-cols-3 gap-2">
                    <For each={VideoAspectRatios}>
                        {ar => (
                            <button
                                class={`btn btn-sm normal-case border-none font-medium ${ratio().id === ar.id ? 'btn-active bg-blue-100 text-blue-600 hover:bg-blue-200' : 'bg-gray-50 hover:bg-gray-100 text-gray-700'}`}
                                onClick={() => setRatio(ar)}
                            >
                                {ar.label}
                            </button>
                        )}
                    </For>
                </div>
            </div>

            <div class="space-y-4 flex flex-col gap-2">
                <span class="text-sm font-medium text-gray-800">AI增强</span>
                <button
                    class={`btn border-none btn-sm normal-case font-medium ${isSmartRewrite() ? 'btn-active bg-blue-100 text-blue-600 hover:bg-blue-200' : 'btn-ghost bg-gray-100 hover:bg-gray-200 text-gray-700'}`}
                    onClick={() => setIsSmartRewrite(!isSmartRewrite())}
                >
                    <PencilIcon class="w-4 h-4" />
                    智能扩写
                </button>
            </div>

            <div class="flex-grow"></div>

            <button
                class="btn btn-primary btn-md w-full normal-case text-base font-medium"
                onClick={handleGenerate}
                disabled={isPending() || !prompt().trim()}
            >
                {isPending() ? <span class="loading loading-spinner loading-sm"></span> : null}
                生成视频
            </button>
        </div>
    )

    const RightPanel = () => (
        <div class="flex-1 p-6 bg-white overflow-y-auto h-full rounded-lg ml-4">
            <GeneratedVideoSet
                videos={generatedVideos()}
                onVideoSelect={handleVideoSelect}
                loading={isPending()}
            />

            <Show when={error()}>
                <ErrorAlert message={error()?.message} />
            </Show>
        </div>
    )

    return (
        <div class="flex h-screen max-h-screen overflow-hidden bg-gray-50 p-6">
            <LeftPanel />
            <RightPanel />
        </div>
    )
}
