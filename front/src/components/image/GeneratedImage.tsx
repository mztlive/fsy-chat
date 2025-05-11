import { Motion } from 'solid-motionone'
import { AspectRatio, ImageActionButtons } from '.'
import { createSignal, Show } from 'solid-js'

interface GeneratedImageProps {
    url: string
    aspectRatio: AspectRatio
}

const GeneratedImage = (props: GeneratedImageProps) => {
    const [showToast, setShowToast] = createSignal(false)
    const [toastMessage, setToastMessage] = createSignal('')
    const [toastType, setToastType] = createSignal<'info' | 'success' | 'error'>('info')
    const { width, height } = props.aspectRatio

    // 显示提示消息
    const showMessage = (message: string, type: 'info' | 'success' | 'error' = 'info') => {
        setToastMessage(message)
        setToastType(type)
        setShowToast(true)
        setTimeout(() => setShowToast(false), 2000)
    }

    // 检测是否为移动设备
    const isMobile = () => {
        return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
            navigator.userAgent
        )
    }

    const handleDownload = async () => {
        try {
            window.open(props.url, '_blank')
            showMessage('请长按图片进行保存', 'info')
        } catch (error) {
            console.error('下载图片失败:', error)
            showMessage('下载失败', 'error')
        }
    }

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(props.url)
            showMessage('图片URL已复制到剪贴板', 'success')
        } catch (error) {
            console.error('复制图片失败:', error)
            showMessage('复制失败', 'error')
        }
    }

    return (
        <>
            <Motion.div
                class="relative w-full h-full overflow-hidden rounded-lg cursor-pointer hover:shadow-lg transition group"
                style={{
                    'aspect-ratio': `${width}/${height}`,
                }}
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
                        <ImageActionButtons onDownload={handleDownload} onCopy={handleCopy} />
                    </div>
                </div>
            </Motion.div>

            {/* daisyUI Toast */}
            <Show when={showToast()}>
                <div class="toast toast-bottom toast-center z-50">
                    <div
                        class={`alert ${
                            toastType() === 'success'
                                ? 'alert-success'
                                : toastType() === 'error'
                                  ? 'alert-error'
                                  : 'alert-info'
                        }`}
                    >
                        <span>{toastMessage()}</span>
                    </div>
                </div>
            </Show>
        </>
    )
}

export default GeneratedImage
