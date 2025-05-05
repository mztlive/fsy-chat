import { For, Show } from 'solid-js'
import ImageSkelon from '../common/ImageSkelon'
import { GeneratedImage, ImageActionButtons } from './'

export interface GeneratedImagesProps {
    images: GeneratedImage[]
    onImageSelect: (imageId: string) => void
    loading: boolean
}

export default function GeneratedImages(props: GeneratedImagesProps) {
    return (
        <div>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {/* 加载状态下显示骨架屏 */}
                <Show when={props.loading}>
                    {Array(4)
                        .fill(0)
                        .map(() => (
                            <ImageSkelon />
                        ))}
                </Show>

                {/* 显示现有图片 */}
                <Show when={props.images.length > 0}>
                    <For each={props.images}>
                        {image => (
                            <div class="flex flex-col gap-2 w-full">
                                <span>{image.timestamp}</span>

                                {/* 每个图片都占用与骨架屏相同的空间 */}
                                <div class="grid grid-cols-1 gap-4">
                                    <For each={image.urls}>
                                        {url => (
                                            <div class="relative aspect-square overflow-hidden rounded-lg cursor-pointer hover:shadow-lg transition group">
                                                <img
                                                    src={url}
                                                    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                                                    loading="lazy"
                                                    onClick={() => props.onImageSelect(image.id)}
                                                />
                                                <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-all duration-200">
                                                    <div class="absolute bottom-2 right-2 flex space-x-1">
                                                        <ImageActionButtons
                                                            onDownload={() =>
                                                                console.log('下载图片', image.id)
                                                            }
                                                            onCopy={() =>
                                                                console.log('复制图片', image.id)
                                                            }
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        )}
                                    </For>
                                </div>

                                {/* 日期信息移到图片组外部 */}
                                <div class="text-xs text-gray-500">
                                    {new Date(image.timestamp).toLocaleDateString()}
                                </div>
                            </div>
                        )}
                    </For>
                </Show>
            </div>
        </div>
    )
}
