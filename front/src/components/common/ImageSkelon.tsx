const ImageSkelon = () => {
    return (
        <div class="relative aspect-square overflow-hidden rounded-lg bg-gray-200">
            <div
                class="absolute inset-0"
                style={{
                    'background-image':
                        'linear-gradient(135deg, rgba(255,255,255,0) 0%, rgba(255,255,255,0.2) 50%, rgba(255,255,255,0) 100%)',
                    'background-size': '200% 200%',
                    animation: 'shimmer 1.5s infinite',
                }}
            />
            <style>
                {`
            @keyframes shimmer {
                0% {
                    background-position: -100% -100%;
                }
                100% {
                    background-position: 200% 200%;
                }
            }
            `}
            </style>
            <div class="absolute inset-0 flex items-center justify-center">
                <svg
                    class="w-12 h-12 text-gray-300"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 4.75v1.5M17.127 6.873l-1.061 1.061M19.25 12h-1.5M17.127 17.127l-1.061-1.061M12 17.75v1.5M7.934 16.066l-1.06 1.06M6.25 12h-1.5M7.934 7.934l-1.06-1.06"
                    />
                </svg>
            </div>
        </div>
    )
}

export default ImageSkelon
