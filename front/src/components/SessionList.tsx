import { For } from 'solid-js'
import { SessionListItem } from './SessionListItem'
import { PencilIcon } from './icons'
import { SessionHistory } from '~/api/types'

interface SessionListProps {
    sessions: SessionHistory[]
    activeSessionId: string | null
    onSelectSession: (sessionId: string) => void
    onDeleteSession?: (sessionId: string) => void
    onCreateNewSession: (category?: string) => void
}

export function SessionList(props: SessionListProps) {
    return (
        <div class="flex flex-col h-full">
            <div class="p-6 flex items-center justify-between">
                <span class="text-md font-medium">New Chat</span>
                <button
                    class="btn btn-sm btn-ghost btn-circle"
                    onClick={() => props.onCreateNewSession()}
                >
                    <PencilIcon size={18} />
                </button>
            </div>

            <div class="overflow-y-auto flex-1 px-2 pb-4">
                <For
                    each={props.sessions}
                    fallback={
                        <div class="text-center text-sm text-base-content/50 py-4">
                            暂无历史会话
                        </div>
                    }
                >
                    {session => (
                        <SessionListItem
                            session={session}
                            isActive={session.session_id == props.activeSessionId}
                            onClick={props.onSelectSession}
                            onDelete={props.onDeleteSession}
                        />
                    )}
                </For>
            </div>
        </div>
    )
}
