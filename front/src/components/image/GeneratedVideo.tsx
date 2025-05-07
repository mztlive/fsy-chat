import { Motion } from 'solid-motionone'

interface GeneratedVideoProps {
    url: string
}

const GeneratedVideo = (props: GeneratedVideoProps) => {
    return (
        <div class="w-full">
            <video
                src={props.url}
                class="w-full h-auto rounded-t-box object-contain"
                controls
                preload="metadata"
            />
        </div>
    )
}

export default GeneratedVideo
