import { ChatSession, Message } from "../types/chat";

// 生成唯一ID
export function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).substring(2);
}

// 创建新消息
export function createMessage(role: "user" | "assistant" | "system", content: string): Message {
  return {
    id: generateId(),
    role,
    content,
    timestamp: Date.now(),
  };
}

// 创建新会话
export function createNewSession(initialMessage?: Message): ChatSession {
  const now = Date.now();
  const messages = initialMessage ? [initialMessage] : [];
  
  return {
    id: generateId(),
    title: initialMessage
      ? initialMessage.content.slice(0, 30) +
        (initialMessage.content.length > 30 ? "..." : "")
      : "新会话",
    messages,
    createdAt: now,
    updatedAt: now,
    metadata: {},
  };
}

// 从本地存储加载会话列表
export function loadSessionsFromStorage(): ChatSession[] {
  try {
    const storedSessions = localStorage.getItem("chat-sessions");
    return storedSessions ? JSON.parse(storedSessions) : [];
  } catch (error) {
    console.error("Failed to load sessions from storage:", error);
    return [];
  }
}

// 保存会话列表到本地存储
export function saveSessionsToStorage(sessions: ChatSession[]): void {
  try {
    localStorage.setItem("chat-sessions", JSON.stringify(sessions));
  } catch (error) {
    console.error("Failed to save sessions to storage:", error);
  }
} 