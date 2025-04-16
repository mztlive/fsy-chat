import { Message } from "../../types/chat";

interface UserMessageProps {
  message: Message;
}

export const UserMessage = ({ message }: UserMessageProps) => {
  return (
    <div class="flex gap-3 flex-row-reverse">
      <div class="bg-primary text-primary-content w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold self-start flex-shrink-0">
        您
      </div>
      <div class="flex-1 text-right overflow-hidden">
        <div class="whitespace-pre-wrap break-words text-sm">
          {message.content}
        </div>
      </div>
    </div>
  );
};
