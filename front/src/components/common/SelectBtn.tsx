import { ParentProps } from 'solid-js'

interface SelectBtnProps extends ParentProps {
    isSelected: boolean
    onSelect: () => void
}

const SelectBtn = (props: SelectBtnProps) => {
    return (
        <button
            classList={{
                btn: true,
                'btn-sm': true,
                'normal-case': true,
                'border-none': true,
                'font-medium': true,
                'btn-active': props.isSelected,
                'bg-blue-100': props.isSelected,
                'text-blue-600': props.isSelected,
                'hover:bg-blue-200': props.isSelected,
                'bg-gray-50': !props.isSelected,
                'hover:bg-gray-100': !props.isSelected,
                'text-gray-700': !props.isSelected,
            }}
            onClick={props.onSelect}
        >
            {props.children}
        </button>
    )
}

export default SelectBtn
