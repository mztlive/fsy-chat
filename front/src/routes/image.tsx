import { createFileRoute } from '@tanstack/solid-router'
import { Show, For, createMemo } from 'solid-js'
import { GeneratedImages, AspectRatio } from '../components/image'
import ErrorAlert from '~/components/common/ErrorAlert'
import { useImageGenerate } from '~/hooks/image_generate'
import { createStore } from 'solid-js/store'
import { PencilIcon } from '~/components/icons'
import { ImageAspectRatios } from '~/components/image/AspectRatioSelector'
import SelectBtn from '~/components/common/SelectBtn'

export const Route = createFileRoute('/image')({
    component: ImageRoute,
})

function ImageRoute() {
    const [form, setForm] = createStore({
        prompt: '',
        ratio: ImageAspectRatios[0],
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
                        class="textarea w-full h-32 resize-none text-sm  focus:outline-none"
                        placeholder="输入画面描述，例如：一只可爱的猫咪，水彩画风格"
                        value={form.prompt}
                        onInput={e => setForm('prompt', e.currentTarget.value)}
                    />
                    <div class="text-xs text-right text-gray-400">{form.prompt.length}/800</div>
                </div>

                <div class="space-y-4">
                    <span class="text-sm font-medium text-gray-800">禁止内容</span>
                    <textarea
                        class="textarea w-full h-32 resize-none text-sm  focus:outline-none"
                        placeholder="输入禁止内容，例如：人物、动物、建筑"
                        value={form.negative_prompt}
                        onInput={e => setForm('negative_prompt', e.currentTarget.value)}
                    />
                </div>
            </div>

            <div class="space-y-2">
                <label class="text-sm font-medium text-gray-800">比例</label>
                <div class="grid grid-cols-3 gap-2">
                    <For each={ImageAspectRatios}>
                        {ar => (
                            <SelectBtn
                                isSelected={form.ratio.id === ar.id}
                                onSelect={() => setForm('ratio', ar)}
                            >
                                {ar.label}
                            </SelectBtn>
                        )}
                    </For>
                </div>
            </div>

            <div class="space-y-4 flex flex-col gap-2">
                <span class="text-sm font-medium text-gray-800">AI增强</span>
                <SelectBtn
                    isSelected={form.is_smart_rewrite}
                    onSelect={() => setForm('is_smart_rewrite', !form.is_smart_rewrite)}
                >
                    <PencilIcon class="w-4 h-4" />
                    智能扩写
                </SelectBtn>
            </div>

            <div class="flex-grow"></div>

            <button
                class="btn btn-primary btn-md w-full"
                onClick={handleGenerate}
                disabled={isPending() || !form.prompt.trim()}
            >
                <Show when={isPending()}>
                    <span class="loading loading-spinner loading-sm"></span>
                </Show>
                生成画作
            </button>
        </div>
    )

    const RightPanel = () => (
        <div class="flex-1 p-6 bg-white  overflow-y-auto h-full rounded-lg ml-4">
            <GeneratedImages
                images={generatedImages()}
                loading={isPending()}
                aspectRatio={form.ratio}
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
