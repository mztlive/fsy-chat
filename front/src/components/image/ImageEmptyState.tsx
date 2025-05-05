import { Component } from 'solid-js'

const EmptyState: Component = () => {
    return (
        <div class="text-center py-16 text-gray-400">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-16 w-16 mx-auto mb-4 opacity-20"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1"
                    d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                />
            </svg>
            <p>您创建的图像将在这里展示</p>
        </div>
    )
}

export default EmptyState
