import { CopyIcon, DownloadIcon, MoreIcon, TrashIcon } from '../icons'
import { createSignal } from 'solid-js'

interface VideoActionMenuProps {
    url: string
    onDelete?: () => void
    onCopy?: () => void
    onDownload?: () => void
}

export default function VideoActionMenu(props: VideoActionMenuProps) {
    const handleDownload = (e: Event) => {
        e.preventDefault()
        e.stopPropagation() // 阻止事件冒泡
        if (props.onDownload) {
            props.onDownload()
        } else {
            // 默认下载行为
            const link = document.createElement('a')
            link.href = props.url
            link.download = `video-${Date.now()}.mp4`
            document.body.appendChild(link)
            link.click()
            document.body.removeChild(link)
        }
    }

    const handleCopy = (e: Event) => {
        e.preventDefault()
        e.stopPropagation() // 阻止事件冒泡
        if (props.onCopy) {
            props.onCopy()
        } else {
            // 默认复制行为
            navigator.clipboard
                .writeText(props.url)
                .then(() => {
                    // 可以添加成功提示
                })
                .catch(err => {
                    console.error('复制失败:', err)
                })
        }
    }

    const handleDelete = (e: Event) => {
        e.preventDefault()
        e.stopPropagation() // 阻止事件冒泡
        if (props.onDelete) {
            props.onDelete()
        }
    }

    return (
        <div class="dropdown dropdown-end" onClick={e => e.stopPropagation()}>
            {/* 菜单按钮 */}
            <button
                class="btn btn-sm btn-circle bg-base-100/70 hover:bg-base-100/90 backdrop-blur-sm shadow-sm"
                onClick={e => e.stopPropagation()}
            >
                <MoreIcon class="h-4 w-4" />
            </button>

            {/* 下拉菜单内容 */}
            <ul class="dropdown-content z-[100] menu p-2 shadow-lg bg-base-100 rounded-box w-48">
                <li>
                    <button onClick={handleDownload} class="flex items-center gap-2">
                        <DownloadIcon class="h-4 w-4" />
                        <span>下载视频</span>
                    </button>
                </li>
                <li>
                    <button onClick={handleCopy} class="flex items-center gap-2">
                        <CopyIcon class="h-4 w-4" />
                        <span>复制链接</span>
                    </button>
                </li>
                {props.onDelete && (
                    <li>
                        <button onClick={handleDelete} class="flex items-center gap-2 text-error">
                            <TrashIcon class="h-4 w-4" />
                            <span>删除视频</span>
                        </button>
                    </li>
                )}
            </ul>
        </div>
    )
}
