import { JSX } from 'solid-js'

export interface ErrorAlertProps {
    message: string | JSX.Element
    className?: string
}

const ErrorAlert = (props: ErrorAlertProps) => {
    return (
        <div class={`mt-2 p-3 rounded-lg bg-red-50 text-red-600 text-sm ${props.className || ''}`}>
            <div class="flex items-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5 mr-2 flex-shrink-0"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
                {props.message}
            </div>
        </div>
    )
}

export default ErrorAlert
