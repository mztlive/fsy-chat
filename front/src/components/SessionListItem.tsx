import { ChatBubbleIcon, TrashIcon } from './icons'
import { SessionHistory } from '~/api/types'

interface SessionListItemProps {
    session: SessionHistory
    isActive: boolean
    onClick: (sessionId: string) => void
    onDelete?: (sessionId: string) => void
}

export function SessionListItem(props: SessionListItemProps) {
    return (
        <div
            class={`flex items-center p-2 rounded-md cursor-pointer group my-1 ${
                props.isActive
                    ? 'bg-gray-200/60 text-base-content'
                    : 'hover:bg-gray-200/40 text-base-content/80'
            }`}
            onClick={() => props.onClick(props.session.session_id)}
        >
            <div class="pr-2">
                <ChatBubbleIcon />
            </div>
            <div class="flex-1 min-w-0">
                <div class="font-normal text-sm truncate">{props.session.title}</div>
            </div>
            <div class="opacity-0 group-hover:opacity-100 transition-opacity ml-2">
                <button
                    class="btn btn-xs btn-ghost btn-square"
                    onClick={e => {
                        e.stopPropagation()
                        props.onDelete?.(props.session.session_id)
                    }}
                    title="删除对话"
                >
                    <TrashIcon />
                </button>
            </div>
        </div>
    )
}
