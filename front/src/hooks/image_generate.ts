import { useMutation } from '@tanstack/solid-query'
import { createMemo } from 'solid-js'
import { GeneratedImage } from '~/api'
import { imageGeneration } from '~/api/image'

export const useImageGenerate = () => {
    const generate = useMutation(() => ({
        mutationFn: (params: {
            prompt: string
            width: number
            height: number
            isSmartRewrite: boolean
        }) => imageGeneration(params.prompt, params.width, params.height, params.isSmartRewrite),
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

    const createImage = (
        prompt: string,
        width: number,
        height: number,
        isSmartRewrite: boolean
    ) => {
        if (!prompt.trim()) {
            return
        }

        return generate.mutateAsync({ prompt, width, height, isSmartRewrite })
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
