import { Message } from "../../types/chat";
import { SendIcon } from "../icons";
import { MarkdownRenderer } from "./MarkdownRenderer";

interface AssistantMessageProps {
  message: Message;
}

export const AssistantMessage = ({ message }: AssistantMessageProps) => {
  return (
    <div class="flex gap-3">
      <div class="bg-secondary text-secondary-content w-8 h-8 rounded-full flex items-center justify-center self-start flex-shrink-0">
        <SendIcon class="w-5 h-5" />
      </div>
      <div class="flex-1 overflow-hidden">
        <MarkdownRenderer content={message.content} />
      </div>
    </div>
  );
};
