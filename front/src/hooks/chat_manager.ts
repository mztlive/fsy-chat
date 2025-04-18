import { createMemo, createResource, onCleanup, onMount } from 'solid-js'
import { createChatSession, getAllDocumentCategories, getSessionHistory } from '~/api/chat'
import { SessionHistory } from '~/api/types'

export const useChatManager = () => {
    const [remoteCategories] = createResource(
        async () => {
            const response = await getAllDocumentCategories()
            return response.data
        },
        {
            initialValue: [],
        }
    )

    const categories = createMemo(() => {
        return ['不立人设', ...remoteCategories()]
    })

    const [sessionHistory, { refetch: refetchSessionHistory }] = createResource(
        async () => {
            const response = await getSessionHistory()
            return response.data
        },
        {
            initialValue: [],
        }
    )

    // 创建新会话
    const createSession = async (category?: string) => {
        const response = await createChatSession({ category })
        return response.data.session_id
    }

    return {
        categories,
        createSession,
        sessionHistory,
        refetchSessionHistory,
    }
}
