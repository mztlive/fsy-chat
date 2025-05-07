import { BASE_URL, post } from './request'
import { ApiResponse, GeneratedImage } from './types'

export const imageGeneration = async (
    prompt: string,
    width: number,
    height: number
): Promise<ApiResponse<GeneratedImage>> => {
    return post(`/image/generation`, {
        prompt,
        width,
        height,
    })
}
