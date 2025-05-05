import { Component } from 'solid-js'

export interface ImageActionButtonsProps {
    onDownload?: () => void
    onCopy?: () => void
}

const ImageActionButtons: Component<ImageActionButtonsProps> = props => {
    return (
        <>
            <button
                class="p-1.5 bg-white/90 rounded-full text-gray-700 hover:bg-white"
                onClick={props.onDownload}
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                    />
                </svg>
            </button>
            <button
                class="p-1.5 bg-white/90 rounded-full text-gray-700 hover:bg-white"
                onClick={props.onCopy}
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                    />
                </svg>
            </button>
        </>
    )
}

export default ImageActionButtons
