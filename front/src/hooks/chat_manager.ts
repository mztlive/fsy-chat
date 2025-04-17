import { createResource } from 'solid-js'
import { createChatSession, getAllDocumentCategories } from '~/api/chat'
import { createMessage, createNewSession } from '~/utils/chatUtils'

export const useChatManager = () => {
    const categories = createResource(async () => {
        const response = await getAllDocumentCategories()
        if (response.status === 200) {
            return response.data
        }

        return []
    })

    // 创建新会话
    const createSession = async (category?: string) => {
        const response = await createChatSession({ category })
        return response.data.session_id
    }

    return {
        categories,
        createSession,
    }
}
