import { For, Show, createEffect, onMount } from "solid-js";
import { Message } from "../types/chat";
import { ChatMessage } from "./ChatMessage";
import { DocumentIcon, PencilIcon, QuestionIcon, AlignIcon } from "./icons";

interface ChatWindowProps {
  messages: Message[];
  loading?: boolean;
}

export function ChatWindow(props: ChatWindowProps) {
  let messagesEndRef: HTMLDivElement | undefined;

  // 滚动到最新消息
  const scrollToBottom = () => {
    messagesEndRef?.scrollIntoView({ behavior: "smooth" });
  };

  // 当消息列表变化或加载状态变化时，滚动到底部
  createEffect(() => {
    if (props.messages.length || props.loading) {
      setTimeout(scrollToBottom, 100);
    }
  });

  // 组件挂载时滚动到底部
  onMount(() => {
    scrollToBottom();
  });

  return (
    <div class="flex-1 overflow-y-auto overflow-x-hidden">
      <Show
        when={props.messages.length > 0}
        fallback={
          <div class="flex flex-col items-center justify-center h-full text-center p-4">
            <div class="text-2xl font-medium mb-8">有什么可以帮忙的?</div>
            <div class="grid grid-cols-2 gap-3 max-w-md w-full">
              <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                <span class="text-primary flex-shrink-0">
                  <DocumentIcon size={24} />
                </span>
                <span class="text-sm">总结文本</span>
              </button>
              <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                <span class="text-secondary flex-shrink-0">
                  <PencilIcon size={24} />
                </span>
                <span class="text-sm">帮我写</span>
              </button>
              <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                <span class="text-info flex-shrink-0">
                  <QuestionIcon size={24} />
                </span>
                <span class="text-sm">提供建议</span>
              </button>
              <button class="border border-base-300 py-3 px-3 rounded-xl flex items-center gap-2 transition-colors text-left">
                <span class="text-accent flex-shrink-0">
                  <AlignIcon size={24} />
                </span>
                <span class="text-sm">给我惊喜</span>
              </button>
            </div>
            <button class="mt-3 py-2 px-4 rounded-xl transition-colors">
              更多
            </button>
          </div>
        }
      >
        <div class="pb-32 pt-4">
          <For each={props.messages}>
            {(message) => (
              <div
                class={`py-4 ${message.role === "assistant" ? "bg-base-100" : "bg-base-100"}`}
              >
                <div class="max-w-3xl mx-auto px-4 md:px-8 w-full overflow-hidden">
                  <ChatMessage message={message} />
                </div>
              </div>
            )}
          </For>

          <Show when={props.loading}>
            <div class="py-4 bg-base-100">
              <div class="max-w-3xl mx-auto px-4 md:px-8 w-full overflow-hidden">
                <div class="chat chat-start">
                  <div class="chat-bubble bg-base-200 text-base-content">
                    <span class="loading loading-dots loading-md"></span>
                  </div>
                </div>
              </div>
            </div>
          </Show>

          <div ref={messagesEndRef}></div>
        </div>
      </Show>
    </div>
  );
}
