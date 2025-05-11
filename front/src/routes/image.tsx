import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show, For } from 'solid-js'
import { GeneratedImages, AspectRatio, EmptyState } from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { useImageGenerate } from '~/hooks/image_generate'

export const Route = createFileRoute('/image')({
    component: ImageRoute,
})

function ImageRoute() {
    const availableAspectRatios: AspectRatio[] = [
        { id: '1:1', label: '1:1', ratio: '1:1', width: 1024, height: 1024 },
        { id: '16:9', label: '16:9', ratio: '16:9', width: 1440, height: 810 },
        { id: '9:16', label: '9:16', ratio: '9:16', width: 810, height: 1440 },
        { id: '4:3', label: '4:3', ratio: '4:3', width: 1440, height: 1080 },
        { id: '3:4', label: '3:4', ratio: '3:4', width: 1080, height: 1440 },
    ]

    const [isSmartRewrite, setIsSmartRewrite] = createSignal(false)
    const [ratio, setRatio] = createSignal<AspectRatio>(
        availableAspectRatios.find(r => r.id === '9:16') || availableAspectRatios[0]
    )
    const [prompt, setPrompt] = createSignal('一个粉红色的橡皮擦，椭圆形的，上面是小猪的头，长鼻子')

    const { generatedImages, createImage, error, isPending } = useImageGenerate()

    const handleImageSelect = (imageId: string) => {
        console.log('选择图片:', imageId)
        // 实际选择处理...
    }

    const handleGenerate = async () => {
        if (!prompt().trim()) {
            console.warn('Prompt is empty, generation skipped.')
            return
        }
        await createImage(prompt(), ratio().width, ratio().height, isSmartRewrite())
    }

    const LeftPanel = () => (
        <div class="w-[360px] flex-shrink-0 bg-white p-6 flex flex-col space-y-4  h-full overflow-y-auto rounded-lg">
            <h1 class="text-xl font-semibold text-gray-800">文字作画</h1>

            <div class="form-control">
                <textarea
                    class="textarea w-full h-32 resize-none text-sm leading-relaxed focus:outline-none"
                    placeholder="输入画面描述，例如：一只可爱的猫咪，水彩画风格"
                    value={prompt()}
                    onInput={e => setPrompt(e.currentTarget.value)}
                />
                <div class="flex items-center justify-between mt-2">
                    <button
                        class={`btn border-none btn-sm normal-case font-medium ${isSmartRewrite() ? 'btn-active bg-blue-100 text-blue-600 hover:bg-blue-200' : 'btn-ghost bg-gray-100 hover:bg-gray-200 text-gray-700'}`}
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
                        智能扩写
                    </button>
                    <div class="text-xs text-gray-400">{prompt().length}/800</div>
                </div>
            </div>

            <div class="space-y-2">
                <label class="text-sm font-medium text-gray-800">比例</label>
                <div class="grid grid-cols-3 gap-2">
                    <For each={availableAspectRatios}>
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

            <div class="flex-grow"></div>

            <button
                class="btn btn-primary btn-md w-full normal-case text-base font-medium"
                onClick={handleGenerate}
                disabled={isPending() || !prompt().trim()}
            >
                {isPending() ? <span class="loading loading-spinner loading-sm"></span> : null}
                生成画作
            </button>
        </div>
    )

    const RightPanel = () => (
        <div class="flex-1 p-6 bg-white  overflow-y-auto h-full rounded-lg ml-4">
            <GeneratedImages
                images={generatedImages()}
                onImageSelect={handleImageSelect}
                loading={isPending()}
                aspectRatio={ratio()}
            />
            <Show when={!isPending() && generatedImages().length === 0}>
                <EmptyState />
            </Show>
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
