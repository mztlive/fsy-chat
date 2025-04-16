import { Show } from "solid-js";
import { ChatSession } from "../types/chat";
import { ChatBubbleIcon, TrashIcon } from "./icons";

interface SessionListItemProps {
  session: ChatSession;
  isActive: boolean;
  onClick: (sessionId: string) => void;
  onDelete?: (sessionId: string) => void;
}

export function SessionListItem(props: SessionListItemProps) {
  const { session, isActive } = props;

  // 计算会话的相对时间显示
  const getRelativeTime = (timestamp: number) => {
    const now = new Date();
    const date = new Date(timestamp);
    const diffInDays = Math.floor(
      (now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24)
    );

    if (diffInDays === 0) {
      return "今天";
    } else if (diffInDays === 1) {
      return "昨天";
    } else if (diffInDays < 7) {
      return `${diffInDays}天前`;
    } else {
      return date.toLocaleDateString();
    }
  };

  return (
    <div
      class={`flex items-center p-2 rounded-md cursor-pointer group my-1 ${
        isActive
          ? "bg-base-200 text-base-content"
          : "hover:bg-base-200/40 text-base-content/80"
      }`}
      onClick={() => props.onClick(session.id)}
    >
      <div class="pr-2">
        <ChatBubbleIcon />
      </div>
      <div class="flex-1 min-w-0">
        <div class="font-normal text-sm truncate">{session.title}</div>
      </div>
      <div class="opacity-0 group-hover:opacity-100 transition-opacity ml-2">
        <button
          class="btn btn-xs btn-ghost btn-square"
          onClick={(e) => {
            e.stopPropagation();
            props.onDelete?.(session.id);
          }}
          title="删除对话"
        >
          <TrashIcon />
        </button>
      </div>
    </div>
  );
}
