import { BASE_URL, post } from './request'
import { ApiResponse } from './types'

export const imageGeneration = async (prompt: string): Promise<ApiResponse<string[]>> => {
    return post(`/image/generation`, {
        prompt,
    })
}
