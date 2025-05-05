import { nanoid } from 'nanoid'
export type MessageRole = 'user' | 'assistant' | 'system'

export interface Message {
    id: string
    role: MessageRole
    content: string
    timestamp: number
}

export interface ChatSession {
    id: string
    title: string
    messages: Message[]
    createdAt: number
    updatedAt: number
    metadata: any
}

export const createAssistantMessageForImage = (url: string, id?: string) => {
    return {
        id: id || nanoid(),
        role: 'assistant',
        content: `
            <img src="${url}" alt="AI生成的图像" class="w-full h-auto rounded-lg mb-2" />
        `,
    }
}
