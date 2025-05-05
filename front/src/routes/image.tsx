import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show } from 'solid-js'
import { nanoid } from 'nanoid'
import { imageGeneration } from '../api/image'
import {
    NavTabs,
    ModelSelector,
    AspectRatioSelector,
    ImageSizeSettings,
    ReferenceUploader,
    GeneratedImages,
    PromptInput,
    Model,
    AspectRatio,
    Resolution,
    GeneratedImage,
} from '../components/image'

export const Route = createFileRoute('/image')({
    component: ImageRoute,
})

function ImageRoute() {
    // 图片比例列表和选中状态
    const [aspectRatios] = createSignal<AspectRatio[]>([
        { id: '1:1', label: '1:1', ratio: '1:1' },
        { id: '16:9', label: '16:9', ratio: '16:9' },
        { id: '3:2', label: '3:2', ratio: '3:2' },
        { id: '4:3', label: '4:3', ratio: '4:3' },
        { id: '3:4', label: '3:4', ratio: '3:4' },
        { id: '2:3', label: '2:3', ratio: '2:3' },
        { id: '9:16', label: '9:16', ratio: '9:16' },
    ])
    const [selectedRatio, setSelectedRatio] = createSignal('1:1')

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
                // API 返回的是图像URL字符串
                const imageUrl = response.data

                // 添加新生成的图像到列表
                const newImage: GeneratedImage = {
                    id: nanoid(),
                    url: imageUrl,
                    timestamp: Date.now(),
                }

                setGeneratedImages([newImage, ...generatedImages()])
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
        <div class="min-h-screen bg-gray-50 flex flex-col">
            {/* 主内容区 */}
            <div class="flex-1 container mx-auto px-4 py-6">
                <div class="flex gap-6">
                    {/* 左侧边栏 */}
                    <div class="w-64 flex-shrink-0">
                        <AspectRatioSelector
                            ratios={aspectRatios()}
                            selectedRatio={selectedRatio()}
                            onRatioSelect={setSelectedRatio}
                        />
                    </div>

                    {/* 右侧主内容 */}
                    <div class="flex-1">
                        <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-4 mb-6">
                            <PromptInput
                                prompt={prompt()}
                                onPromptChange={setPrompt}
                                onGenerate={handleGenerate}
                                disabled={loading()}
                            />

                            <Show when={error()}>
                                <div class="mt-2 p-3 rounded-lg bg-red-50 text-red-600 text-sm">
                                    <div class="flex items-center">
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="h-5 w-5 mr-2 flex-shrink-0"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                            />
                                        </svg>
                                        {error()}
                                    </div>
                                </div>
                            </Show>
                        </div>

                        {/* 生成的图片展示 */}
                        <GeneratedImages
                            images={generatedImages()}
                            onImageSelect={handleImageSelect}
                            loading={loading()}
                        />
                    </div>
                </div>
            </div>
        </div>
    )
}
