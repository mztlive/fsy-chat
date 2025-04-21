import { createEffect, For } from 'solid-js'
import { ChatSession } from '../types/chat'
import { SessionListItem } from './SessionListItem'
import { PlusIcon } from './icons'
import { SessionHistory } from '~/api/types'
import { useChatManager } from '~/hooks/chat_manager'

interface SessionListProps {
    sessions: SessionHistory[]
    activeSessionId: string | null
    onSelectSession: (sessionId: string) => void
    onDeleteSession?: (sessionId: string) => void
    onCreateNewSession: (category?: string) => void
    categories?: string[]
}

export function SessionList(props: SessionListProps) {
    return (
        <div class="flex flex-col h-full">
            <div class="p-2">
                <button
                    class="btn btn-sm w-full bg-base-100 hover:bg-base-200 text-base-content normal-case font-normal justify-start gap-2 rounded-md border border-base-300/50"
                    onClick={() => props.onCreateNewSession()}
                >
                    <PlusIcon />
                    新建聊天
                </button>
            </div>

            {/* 显示文档类别列表（如果有） */}
            {props.categories && props.categories.length > 0 && (
                <div class="px-2 mb-2">
                    <div class="text-xs font-medium text-base-content/70 mb-1 px-2">文档类别</div>
                    <For each={props.categories}>
                        {category => (
                            <button
                                class="btn btn-xs btn-ghost w-full justify-start text-left mb-1"
                                onClick={() => props.onCreateNewSession(category)}
                            >
                                {category}
                            </button>
                        )}
                    </For>
                </div>
            )}

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
