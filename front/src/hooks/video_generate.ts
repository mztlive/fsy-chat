import { useMutation } from '@tanstack/solid-query'
import { createMemo } from 'solid-js'
import { GeneratedImage, GeneratedVideo } from '~/api'
import { imageGeneration } from '~/api/image'
import { videoGeneration } from '~/api/video'

export const useVideoGenerate = () => {
    const generate = useMutation(() => ({
        mutationFn: (params: {
            prompt: string
            width: number
            height: number
            isSmartRewrite: boolean
        }) => videoGeneration(params.prompt, params.width, params.height, params.isSmartRewrite),
        initialData: {
            data: {
                url: '',
                timestamp: 0,
                actual_prompt: '',
            },
            status: 200,
            message: '',
        },
    }))

    const createVideo = (
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

    const generatedVideos = createMemo((prev: GeneratedVideo[]) => {
        if (generate.data?.data) {
            return [generate.data.data, ...prev]
        }

        return prev
    }, [])

    return {
        generatedVideos,
        createVideo,
        error: () => generate.error,
        isPending: () => generate.isPending,
    }
}
