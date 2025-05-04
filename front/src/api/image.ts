import { BASE_URL, post } from './request'
import { ApiResponse, ImageGenerationResponse } from './types'

export const imageGeneration = async (
    prompt: string
): Promise<ApiResponse<ImageGenerationResponse>> => {
    return post<ImageGenerationResponse>(`${BASE_URL}/image/generation`, {
        prompt,
    })
}
