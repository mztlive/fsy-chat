import { AspectRatio } from '../image'

interface ImageSkelonProps {
    aspectRatio: AspectRatio
}

const ImageSkelon = (props: ImageSkelonProps) => {
    // 解构，故意破坏响应式，可以保持这一次生成的比例
    const { width, height } = props.aspectRatio

    return (
        <div
            class="skeleton rounded-box"
            style={{
                'aspect-ratio': `${width}/${height}`,
            }}
        ></div>
    )
}

export default ImageSkelon
