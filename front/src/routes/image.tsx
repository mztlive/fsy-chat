import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show } from 'solid-js'
import { nanoid } from 'nanoid'
import { imageGeneration } from '../api/image'
import {
    AspectRatioSelector,
    GeneratedImages,
    PromptInput,
    AspectRatio,
    EmptyState,
} from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { Portal } from 'solid-js/web'
import { GeneratedImage } from '~/api'
import { useImageGenerate } from '~/hooks/image_generate'

export const Route = createFileRoute('/image')({
    component: ImageRoute,
})

function ImageRoute() {
    // 提示词和图片生成状态
    const [prompt, setPrompt] = createSignal('')

    const [ratio, setRatio] = createSignal<AspectRatio>({
        id: '1:1',
        label: '1:1',
        ratio: '1:1',
        width: 1024,
        height: 1024,
    })

    const { generatedImages, createImage, error, isPending } = useImageGenerate()

    // 选择图片
    const handleImageSelect = (imageId: string) => {
        console.log('选择图片:', imageId)
        // 实际选择处理...
    }

    // 生成图片
    const handleGenerate = async () => {
        await createImage(prompt(), ratio().width, ratio().height)
        setPrompt('') // 清空输入框
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
                    <div class="fixed bottom-4 left-0 right-0 p-6 z-10">
                        <PromptInput
                            prompt={prompt()}
                            onPromptChange={setPrompt}
                            onGenerate={handleGenerate}
                            loading={isPending()}
                            onRatioSelect={setRatio}
                        />

                        <Show when={error()}>
                            <ErrorAlert message={error()?.message} />
                        </Show>
                    </div>
                </Portal>
            </div>
        </div>
    )
}
