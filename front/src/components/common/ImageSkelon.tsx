import { AspectRatio } from '../image'

interface ImageSkelonProps {
    aspectRatio: AspectRatio
}

const ImageSkelon = (props: ImageSkelonProps) => {
    return (
        <div
            class="skeleton rounded-box"
            style={{
                'aspect-ratio': `${props.aspectRatio.width}/${props.aspectRatio.height}`,
            }}
        ></div>
    )
}

export default ImageSkelon
