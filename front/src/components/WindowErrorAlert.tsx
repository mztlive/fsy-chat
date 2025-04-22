import { Motion } from 'solid-motionone'

interface WindowErrorAlertProps {
    sseError: string
}

const WindowErrorAlert = (props: WindowErrorAlertProps) => {
    return (
        <Motion.div
            animate={{ opacity: [0, 1], transform: ['translateY(-10px)', 'translateY(0)'] }}
            transition={{ duration: 1, easing: 'ease-in-out' }}
            role="alert"
            class="alert alert-error max-w-3xl mx-auto mt-4 px-4 md:px-8 w-full"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 shrink-0 stroke-current"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{props.sseError}</span>
        </Motion.div>
    )
}

export default WindowErrorAlert
