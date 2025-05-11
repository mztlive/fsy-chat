import { createFileRoute } from '@tanstack/solid-router'
import { createSignal, Show, For, createMemo, Switch } from 'solid-js'
import { GeneratedImages, AspectRatio, EmptyState } from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { useImageGenerate } from '~/hooks/image_generate'
import { ImageGenerationRequest } from '~/api/types'
import { createStore } from 'solid-js/store'
import { PencilIcon } from '~/components/icons'

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

    // const [isSmartRewrite, setIsSmartRewrite] = createSignal(false)
    // const [ratio, setRatio] = createSignal<AspectRatio>(
    //     availableAspectRatios.find(r => r.id === '9:16') || availableAspectRatios[0]
    // )
    // const [prompt, setPrompt] = createSignal('一个粉红色的橡皮擦，椭圆形的，上面是小猪的头，长鼻子')

    const [form, setForm] = createStore({
        prompt: '',
        ratio: availableAspectRatios[0],
        is_smart_rewrite: false,
        negative_prompt: '',
    })

    const request = createMemo(() => ({
        prompt: form.prompt,
        width: form.ratio.width,
        height: form.ratio.height,
        is_smart_rewrite: form.is_smart_rewrite,
        negative_prompt: form.negative_prompt,
    }))

    const { generatedImages, createImage, error, isPending } = useImageGenerate()

    const handleImageSelect = (imageId: string) => {
        console.log('选择图片:', imageId)
        // 实际选择处理...
    }

    const handleGenerate = async () => {
        if (!request().prompt.trim()) {
            console.warn('Prompt is empty, generation skipped.')
            return
        }
        await createImage(request())
    }

    const LeftPanel = () => (
        <div class="w-[360px] flex-shrink-0 bg-white p-6 flex flex-col space-y-4  h-full overflow-y-auto rounded-lg">
            <h1 class="text-xl font-semibold text-gray-800">文字作画</h1>

            <div class="form-control space-y-4">
                <div class="space-y-4">
                    <span class="text-sm font-medium text-gray-800">画面描述</span>
                    <textarea
                        class="textarea w-full h-32 resize-none text-sm leading-relaxed focus:outline-none"
                        placeholder="输入画面描述，例如：一只可爱的猫咪，水彩画风格"
                        value={form.prompt}
                        onInput={e => setForm('prompt', e.currentTarget.value)}
                    />
                    <div class="text-xs text-right text-gray-400">{form.prompt.length}/800</div>
                </div>

                <div class="space-y-4">
                    <span class="text-sm font-medium text-gray-800">禁止内容</span>
                    <textarea
                        class="textarea w-full h-32 resize-none text-sm leading-relaxed focus:outline-none"
                        placeholder="输入禁止内容，例如：人物"
                        value={form.negative_prompt}
                        onInput={e => setForm('negative_prompt', e.currentTarget.value)}
                    />
                </div>
            </div>

            <div class="space-y-2">
                <label class="text-sm font-medium text-gray-800">比例</label>
                <div class="grid grid-cols-3 gap-2">
                    <For each={availableAspectRatios}>
                        {ar => (
                            <button
                                class={`btn btn-sm normal-case border-none font-medium ${form.ratio.id === ar.id ? 'btn-active bg-blue-100 text-blue-600 hover:bg-blue-200' : 'bg-gray-50 hover:bg-gray-100 text-gray-700'}`}
                                onClick={() => setForm('ratio', ar)}
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
                    class={`btn border-none btn-sm normal-case font-medium ${form.is_smart_rewrite ? 'btn-active bg-blue-100 text-blue-600 hover:bg-blue-200' : 'btn-ghost bg-gray-100 hover:bg-gray-200 text-gray-700'}`}
                    onClick={() => setForm('is_smart_rewrite', !form.is_smart_rewrite)}
                >
                    <PencilIcon class="w-4 h-4" />
                    智能扩写
                </button>
            </div>

            <div class="flex-grow"></div>

            <button
                class="btn btn-primary btn-md w-full normal-case text-base font-medium"
                onClick={handleGenerate}
                disabled={isPending() || !form.prompt.trim()}
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
                aspectRatio={form.ratio}
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
