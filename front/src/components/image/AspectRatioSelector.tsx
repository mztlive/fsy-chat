import { ParentProps } from 'solid-js'

export interface AspectRatio {
    id: string
    label: string
    ratio: string
    width: number
    height: number
}

export interface AspectRatioItemProps extends ParentProps {
    onSelect: () => void
    isSelected: boolean
}

export const ImageAspectRatios: AspectRatio[] = [
    { id: '1:1', label: '1:1', ratio: '1:1', width: 1024, height: 1024 },
    { id: '16:9', label: '16:9', ratio: '16:9', width: 1440, height: 810 },
    { id: '9:16', label: '9:16', ratio: '9:16', width: 810, height: 1440 },
    { id: '4:3', label: '4:3', ratio: '4:3', width: 1440, height: 1080 },
    { id: '3:4', label: '3:4', ratio: '3:4', width: 1080, height: 1440 },
]

export const VideoAspectRatios: AspectRatio[] = [
    {
        id: '1280x720',
        label: '16:9',
        ratio: '16:9',
        width: 1280,
        height: 720,
    },
    { id: '960x960', label: '1:1', ratio: '1:1', width: 960, height: 960 },
    {
        id: '720x1280',
        label: '9:16',
        ratio: '9:16',
        width: 720,
        height: 1280,
    },
    {
        id: '1088x832',
        label: '4:3',
        ratio: '4:3',
        width: 1088,
        height: 832,
    },
    {
        id: '832x1088',
        label: '3:4',
        ratio: '3:4',
        width: 832,
        height: 1088,
    },
]
