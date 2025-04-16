import { Dynamic } from "solid-js/web";
import { Message } from "../types/chat";
import { UserMessage, AssistantMessage } from "./chat-message";

interface ChatMessageProps {
  message: Message;
}

export function ChatMessage(props: ChatMessageProps) {
  const { message } = props;
  const isUser = message.role === "user";

  return (
    <Dynamic
      component={isUser ? UserMessage : AssistantMessage}
      message={message}
    />
  );
}
