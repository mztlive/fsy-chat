import { Match } from 'solid-js'

import { Switch } from 'solid-js'

interface GenerateButtonProps {
    disabled: boolean
    onGenerate: () => void
}

const GenerateButton = (props: GenerateButtonProps) => {
    return (
        <button class="btn btn-primary" onClick={props.onGenerate} disabled={props.disabled}>
            <Switch>
                <Match when={props.disabled}>
                    <div class="mr-2 animate-spin w-4 h-4 border-2 border-white/20 border-t-white rounded-full"></div>
                    生成中...
                </Match>
                <Match when={!props.disabled}>
                    <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M13 10V3L4 14h7v7l9-11h-7z"
                        />
                    </svg>
                    立即生成
                </Match>
            </Switch>
        </button>
    )
}

export default GenerateButton
