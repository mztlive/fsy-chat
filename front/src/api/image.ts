import { post } from './request'
import { ApiResponse, GeneratedImage, ImageGenerationRequest } from './types'

export const imageGeneration = async (
    params: ImageGenerationRequest
): Promise<ApiResponse<GeneratedImage>> => {
    return post(`/image/generation`, params)
}
