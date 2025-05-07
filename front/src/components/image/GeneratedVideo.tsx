import { createSignal } from 'solid-js'

interface GeneratedVideoProps {
    url: string
}

const GeneratedVideo = (props: GeneratedVideoProps) => {
    const [isLoaded, setIsLoaded] = createSignal(false)

    // 加载视频时检测完成
    const handleVideoLoad = (e: Event) => {
        setIsLoaded(true)
    }

    return (
        <div class="w-full overflow-hidden">
            <div
                class={`w-full ${isLoaded() ? '' : 'bg-base-200 animate-pulse min-h-[200px] flex items-center justify-center'}`}
            >
                {!isLoaded() && <div class="loading loading-spinner loading-md"></div>}
                <video
                    src={props.url}
                    class="w-full h-auto rounded-t-box"
                    controls
                    preload="metadata"
                    onLoadedMetadata={handleVideoLoad}
                    onLoadedData={handleVideoLoad}
                />
            </div>
        </div>
    )
}

export default GeneratedVideo
