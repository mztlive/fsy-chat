import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show } from 'solid-js'
import { PromptInput, AspectRatio, AspectRatioSelector } from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { Portal } from 'solid-js/web'
import { useVideoGenerate } from '~/hooks/video_generate'
import GeneratedVideoSet from '~/components/image/GeneratedVideoSet'

export const Route = createFileRoute('/video')({
    component: ImageRoute,
})

function ImageRoute() {
    const aspectRatios: AspectRatio[] = [
        {
            id: '1280x720',
            label: '1280x720',
            ratio: '16:9',
            width: 1280,
            height: 720,
        },
        { id: '960x960', label: '960x960', ratio: '1:1', width: 960, height: 960 },
        {
            id: '720x1280',
            label: '720x1280',
            ratio: '9:16',
            width: 720,
            height: 1280,
        },
        {
            id: '1088x832',
            label: '1088x832',
            ratio: '4:3',
            width: 1088,
            height: 832,
        },
        {
            id: '832x1088',
            label: '832x1088',
            ratio: '3:4',
            width: 832,
            height: 1088,
        },
        { id: '832x480', label: '832x480', ratio: '16:9', width: 832, height: 480 },
        { id: '624x624', label: '624x624', ratio: '1:1', width: 624, height: 624 },
        { id: '480x832', label: '480x832', ratio: '9:16', width: 480, height: 832 },
    ]

    const [ratio, setRatio] = createSignal<AspectRatio>(aspectRatios[0])

    const { generatedVideos, createVideo, error, isPending } = useVideoGenerate()

    // 选择视频
    const handleVideoSelect = (videoId: string) => {
        console.log('选择视频:', videoId)
        // 实际选择处理...
    }

    // 生成图片
    const handleGenerate = async (prompt: string) => {
        await createVideo(prompt, ratio().width, ratio().height)
    }

    return (
        <div class="min-h-screen bg-gray-50">
            <div class="max-w-5xl mx-auto px-4 py-8">
                {/* 主内容区 */}
                {/* 生成的图片展示 */}
                <div class="p-6 pb-32">
                    <GeneratedVideoSet
                        videos={generatedVideos()}
                        onVideoSelect={handleVideoSelect}
                        loading={isPending()}
                    />
                </div>

                {/* 创作区域 - 通过Portal固定在底部 */}
                <Portal>
                    <div class="fixed bottom-4 left-0 right-0 p-6 z-10">
                        <PromptInput onGenerate={handleGenerate} loading={isPending()}>
                            <AspectRatioSelector
                                aspectRatios={aspectRatios}
                                onRatioSelect={setRatio}
                                selectedRatio={ratio()}
                            />
                        </PromptInput>

                        <Show when={error()}>
                            <ErrorAlert message={error()?.message} />
                        </Show>
                    </div>
                </Portal>
            </div>
        </div>
    )
}
