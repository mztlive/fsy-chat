import { Motion } from 'solid-motionone'
import { ImageActionButtons } from '.'

interface GeneratedImageProps {
    url: string
}

const GeneratedImage = (props: GeneratedImageProps) => {
    return (
        <Motion.div
            class="relative aspect-square overflow-hidden rounded-lg cursor-pointer hover:shadow-lg transition group"
            initial={{ scale: 0.95, opacity: 0.5 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ duration: 0.3 }}
        >
            <img
                src={props.url}
                class="w-full h-full object-contain transition-transform duration-300 group-hover:scale-105"
                loading="lazy"
            />
            <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-all duration-200">
                <div class="absolute bottom-2 right-2 flex space-x-1">
                    <ImageActionButtons
                        onDownload={() => console.log('下载图片')}
                        onCopy={() => console.log('复制图片')}
                    />
                </div>
            </div>
        </Motion.div>
    )
}

export default GeneratedImage
