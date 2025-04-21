import { useMutation, useQuery } from '@tanstack/solid-query'
import { createMemo, createResource } from 'solid-js'
import { createChatSession, getAllDocumentCategories, getSessionHistory } from '~/api/chat'

const NORMAL_REFETCH_INTERVAL = 10000

const FILTER_CATEGORY = '不立人设'

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
        return [FILTER_CATEGORY, ...remoteCategories()]
    })

    const sessionHistory = useQuery(() => ({
        queryKey: ['sessionHistory'],
        queryFn: async () => {
            const response = await getSessionHistory()
            return response.data
        },
        refetchInterval: NORMAL_REFETCH_INTERVAL,
        refetchIntervalInBackground: true,
        initialData: [],
    }))

    const createSession = useMutation(() => ({
        mutationFn: async (category?: string) => {
            if (category === FILTER_CATEGORY) {
                return (await createChatSession({})).data.session_id
            }

            return (await createChatSession({ category })).data.session_id
        },
    }))

    return {
        categories,
        createSession,
        sessionHistory,
    }
}
