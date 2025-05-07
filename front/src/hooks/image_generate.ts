import { useMutation, useQuery } from '@tanstack/solid-query'
import { createMemo, createSignal } from 'solid-js'
import { GeneratedImage } from '~/api'
import { imageGeneration } from '~/api/image'

export const useImageGenerate = () => {
    const generate = useMutation(() => ({
        mutationFn: (prompt: string) => imageGeneration(prompt),
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

    const createImage = (prompt: string) => {
        if (!prompt.trim()) {
            return
        }

        return generate.mutateAsync(prompt)
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
