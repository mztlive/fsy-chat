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
                    url: imageUrl, // 直接使用API返回的URL字符串
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
        <div class="min-h-screen bg-gray-50">
            <div class="max-w-5xl mx-auto px-4 py-8">
                {/* 标题区域 */}
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold mb-2 text-gray-800">AI图像生成</h1>
                    <p class="text-gray-500">通过文字描述创建令人惊艳的图像</p>
                </div>

                {/* 主内容区 */}
                <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
                    {/* 创作区域 */}
                    <div class="p-6">
                        <div class="mb-4">
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                选择图片比例
                            </label>
                            <AspectRatioSelector
                                ratios={aspectRatios()}
                                selectedRatio={selectedRatio()}
                                onRatioSelect={setSelectedRatio}
                            />
                        </div>

                        <div class="mt-6">
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                描述你想要的图像
                            </label>
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
                    </div>

                    {/* 分隔线 */}
                    <div class="h-px bg-gray-200"></div>

                    {/* 生成的图片展示 */}
                    <div class="p-6">
                        <h2 class="text-lg font-medium text-gray-800 mb-4">生成的图片</h2>
                        <GeneratedImages
                            images={generatedImages()}
                            onImageSelect={handleImageSelect}
                            loading={loading()}
                        />

                        <Show when={generatedImages().length === 0 && !loading()}>
                            <div class="text-center py-16 text-gray-400">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-16 w-16 mx-auto mb-4 opacity-20"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="1"
                                        d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                    />
                                </svg>
                                <p>您创建的图像将在这里展示</p>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    )
}
