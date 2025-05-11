import { BASE_URL, post } from './request'
import { ApiResponse, GeneratedVideo } from './types'

export const videoGeneration = async (
    prompt: string,
    width: number,
    height: number,
    isSmartRewrite: boolean
): Promise<ApiResponse<GeneratedVideo>> => {
    return post(`/video/generation`, {
        prompt,
        width,
        height,
        is_smart_rewrite: isSmartRewrite,
    })
}
