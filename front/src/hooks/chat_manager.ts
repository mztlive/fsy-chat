import { createResource, onCleanup, onMount } from 'solid-js'
import { createChatSession, getAllDocumentCategories, getSessionHistory } from '~/api/chat'
import { SessionHistory } from '~/api/types'

export const useChatManager = () => {
    const categories = createResource(async () => {
        const response = await getAllDocumentCategories()
        if (response.status === 200) {
            return response.data
        }

        return []
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
