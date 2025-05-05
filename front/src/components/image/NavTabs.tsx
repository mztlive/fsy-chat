import { For } from 'solid-js'

interface NavTab {
    id: string
    label: string
    isActive: boolean
    onClick: () => void
}

export interface NavTabsProps {
    tabs: NavTab[]
}

export default function NavTabs(props: NavTabsProps) {
    return (
        <div class="flex space-x-2 border-b border-gray-200">
            <For each={props.tabs}>
                {tab => (
                    <button
                        class={`px-4 py-2 font-medium transition ${
                            tab.isActive
                                ? 'border-b-2 border-blue-500 text-blue-600'
                                : 'text-gray-600 hover:text-gray-900'
                        }`}
                        onClick={tab.onClick}
                    >
                        {tab.label}
                    </button>
                )}
            </For>
        </div>
    )
}
