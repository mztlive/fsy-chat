import { useMutation } from '@tanstack/solid-query'
import { createMemo } from 'solid-js'
import { GeneratedImage, ImageGenerationRequest } from '~/api/types'
import { imageGeneration } from '~/api/image'

export const useImageGenerate = () => {
    const generate = useMutation(() => ({
        mutationFn: (params: ImageGenerationRequest) => imageGeneration(params),
        initialData: {
            data: {
                urls: [],
                timestamp: 0,
                actual_prompt: '',
            },
            status: 200,
            message: '',
        },
    }))

    const createImage = (request: ImageGenerationRequest) => {
        if (!request.prompt) {
            return
        }

        return generate.mutateAsync(request)
    }

    const generatedImages = createMemo((prev: GeneratedImage[]) => {
        if (generate.data?.data) {
            return [generate.data.data, ...prev]
        }

        return prev
    }, [])

    return {
        generatedImages,
        createImage,
        error: () => generate.error,
        isPending: () => generate.isPending,
    }
}
