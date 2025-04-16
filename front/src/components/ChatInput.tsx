import { createSignal, createEffect, onMount } from "solid-js";
import { PaperClipIcon, GlobeIcon, SparklesIcon, SendIcon } from "./icons";

interface ChatInputProps {
  onSendMessage: (message: string) => void;
  disabled?: boolean;
}

// 定义常量
const INITIAL_HEIGHT = 56; // textarea初始高度(px)
const MAX_HEIGHT = 200; // textarea最大高度(px)

export function ChatInput(props: ChatInputProps) {
  const [inputValue, setInputValue] = createSignal("");
  let textareaRef: HTMLTextAreaElement | undefined;

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    const message = inputValue().trim();
    if (message) {
      props.onSendMessage(message);
      setInputValue("");

      // 重置文本区域高度为初始值
      if (textareaRef) {
        textareaRef.style.height = `${INITIAL_HEIGHT}px`;
      }
    }
  };

  // 自动调整文本区域高度
  const autoResize = () => {
    if (!textareaRef) return;

    // 为空文本特殊处理
    if (!inputValue().trim()) {
      textareaRef.style.height = `${INITIAL_HEIGHT}px`;
      return;
    }

    // 将高度设为0，让scrollHeight能够准确反映内容高度
    textareaRef.style.height = "0px";

    // 根据scrollHeight设置新高度，与初始值取较大值
    const scrollHeight = textareaRef.scrollHeight;
    const newHeight = Math.max(
      INITIAL_HEIGHT,
      Math.min(scrollHeight, MAX_HEIGHT)
    );
    textareaRef.style.height = `${newHeight}px`;
  };

  // 监听值变化
  createEffect(() => {
    const value = inputValue();
    // 当值变为空时，确保高度重置
    if (value === "" && textareaRef) {
      textareaRef.style.height = `${INITIAL_HEIGHT}px`;
    }
  });

  // 处理输入变化
  const handleInput = (e: Event) => {
    const target = e.currentTarget as HTMLTextAreaElement;
    setInputValue(target.value);
    // 直接调整高度，不使用requestAnimationFrame
    autoResize();
  };

  // 组件挂载时聚焦
  onMount(() => {
    if (textareaRef) {
      textareaRef.focus();
      // 确保初始高度正确
      textareaRef.style.height = `${INITIAL_HEIGHT}px`;
    }
  });

  // 处理快捷键
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  return (
    <>
      <div class="fixed bottom-0 left-0 right-0 bg-gradient-to-t from-base-100 via-base-100 to-transparent w-full">
        <div class="max-w-3xl mx-auto px-4 pb-8 w-full">
          <form
            class="rounded border border-base-300 relative overflow-hidden flex items-center bg-base-100 w-full"
            onSubmit={handleSubmit}
          >
            <textarea
              ref={textareaRef}
              placeholder="询问任何问题"
              class={`textarea w-full border-none resize-none py-4 px-4 pr-4 focus:outline-none min-h-[${INITIAL_HEIGHT}px] max-h-[${MAX_HEIGHT}px] bg-transparent text-base-content textarea-transition`}
              value={inputValue()}
              onInput={handleInput}
              onKeyDown={handleKeyDown}
              disabled={props.disabled}
            />
            <div class="flex gap-1 px-2">
              <button
                type="button"
                class="btn btn-circle btn-sm btn-ghost"
                title="上传文件"
              >
                <PaperClipIcon />
              </button>
              {/* <button
                type="button"
                class="btn btn-circle btn-sm btn-ghost"
                title="开启搜索"
              >
                <GlobeIcon />
              </button> */}
              {/* <button
                type="button"
                class="btn btn-circle btn-sm btn-ghost"
                title="更多选项"
              >
                <SparklesIcon />
              </button> */}
              <button
                class="btn btn-circle btn-sm btn-primary"
                type="submit"
                disabled={props.disabled || !inputValue().trim()}
                title="发送消息"
              >
                <SendIcon />
              </button>
            </div>
          </form>
        </div>
      </div>
    </>
  );
}
