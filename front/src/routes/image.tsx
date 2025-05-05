import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show } from 'solid-js'
import { nanoid } from 'nanoid'
import { imageGeneration } from '../api/image'
import {
    AspectRatioSelector,
    GeneratedImages,
    PromptInput,
    AspectRatio,
    GeneratedImage,
    EmptyState,
} from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { Portal } from 'solid-js/web'

export const Route = createFileRoute('/image')({
    component: ImageRoute,
})

function ImageRoute() {
    // 提示词和图片生成状态
    const [prompt, setPrompt] = createSignal('')
    const [generatedImages, setGeneratedImages] = createSignal<GeneratedImage[]>([])
    const [loading, setLoading] = createSignal(false)
    const [error, setError] = createSignal('')

    // 选择图片
    const handleImageSelect = (imageId: string) => {
        console.log('选择图片:', imageId)
        // 实际选择处理...
    }

    // 生成图片
    const handleGenerate = async () => {
        if (!prompt().trim()) {
            setError('请输入描述文字')
            return
        }

        setLoading(true)
        setError('')

        try {
            // 调用API生成图片
            const response = await imageGeneration(prompt())

            if (response.status === 200 && response.data) {
                setGeneratedImages([response.data, ...generatedImages()])
                setPrompt('') // 清空输入框
            } else {
                // 处理失败情况
                setError(response.message || '图像生成失败')
            }
        } catch (error) {
            // 处理异常
            console.error('图像生成出错:', error)
            setError(error instanceof Error ? error.message : '未知错误')
        } finally {
            setLoading(false)
        }
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
                        loading={loading()}
                    />

                    <Show when={generatedImages().length === 0 && !loading()}>
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
                            disabled={loading()}
                        />

                        <Show when={error()}>
                            <ErrorAlert message={error()} />
                        </Show>
                    </div>
                </Portal>
            </div>
        </div>
    )
}
