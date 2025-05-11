import { BASE_URL, post } from './request'
import { ApiResponse, GeneratedImage } from './types'

export const imageGeneration = async (
    prompt: string,
    width: number,
    height: number,
    isSmartRewrite: boolean
): Promise<ApiResponse<GeneratedImage>> => {
    return post(`/image/generation`, {
        prompt,
        width,
        height,
        is_smart_rewrite: isSmartRewrite,
    })
}
