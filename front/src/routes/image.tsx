import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show } from 'solid-js'
import {
    GeneratedImages,
    PromptInput,
    AspectRatio,
    EmptyState,
    AspectRatioSelector,
} from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { Portal } from 'solid-js/web'
import { useImageGenerate } from '~/hooks/image_generate'

export const Route = createFileRoute('/image')({
    component: ImageRoute,
})

function ImageRoute() {
    const aspectRatios: AspectRatio[] = [
        { id: '1:1', label: '1:1', ratio: '1:1', width: 1024, height: 1024 },
        { id: '16:9', label: '16:9', ratio: '16:9', width: 1440, height: 810 },
        { id: '3:2', label: '3:2', ratio: '3:2', width: 1440, height: 960 },
        { id: '4:3', label: '4:3', ratio: '4:3', width: 1440, height: 1080 },
        { id: '3:4', label: '3:4', ratio: '3:4', width: 1080, height: 1440 },
        { id: '2:3', label: '2:3', ratio: '2:3', width: 960, height: 1440 },
        { id: '9:16', label: '9:16', ratio: '9:16', width: 810, height: 1440 },
    ]

    const [isSmartRewrite, setIsSmartRewrite] = createSignal(false)

    const [ratio, setRatio] = createSignal<AspectRatio>(aspectRatios[0])

    const { generatedImages, createImage, error, isPending } = useImageGenerate()

    // 选择图片
    const handleImageSelect = (imageId: string) => {
        console.log('选择图片:', imageId)
        // 实际选择处理...
    }

    // 生成图片
    const handleGenerate = async (prompt: string) => {
        await createImage(prompt, ratio().width, ratio().height, isSmartRewrite())
    }

    return (
        <div class="min-h-screen bg-gray-50">
            <div class="max-w-5xl mx-auto px-4 py-8">
                {/* 主内容区 */}
                {/* 生成的图片展示 */}
                <div class="p-6 pb-32">
                    <GeneratedImages
                        images={generatedImages()}
                        onImageSelect={handleImageSelect}
                        loading={isPending()}
                    />

                    <Show when={generatedImages().length === 0 && !isPending()}>
                        <EmptyState />
                    </Show>
                </div>

                {/* 创作区域 - 通过Portal固定在底部 */}
                <Portal>
                    <div class="max-w-5xl mx-auto fixed bottom-4 left-0 right-0 p-6 z-10">
                        <PromptInput onGenerate={handleGenerate} loading={isPending()}>
                            <AspectRatioSelector
                                aspectRatios={aspectRatios}
                                onRatioSelect={setRatio}
                                selectedRatio={ratio()}
                            />
                            <div class="join-item">
                                <button
                                    class="flex items-center gap-1 hover:bg-gray-100 transition-colors bg-white shadow-sm data-[active=true]:bg-base-200 data-[active=true]:text-primary rounded-md px-3 py-1.5"
                                    data-active={isSmartRewrite()}
                                    onClick={() => setIsSmartRewrite(!isSmartRewrite())}
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-4 w-4"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" />
                                    </svg>
                                    <span class="text-sm font-medium">智能扩写</span>
                                </button>
                            </div>
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
