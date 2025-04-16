import { markedHighlight } from "marked-highlight";
import { createMemo, onMount } from "solid-js";
import hljs from "highlight.js";
import { Marked } from "marked";
import { addCopyButtons } from "./CodeCopyButton";
import "highlight.js/styles/github.css";
import "github-markdown-css";

// 配置marked使用highlight.js
const marked = new Marked(
  markedHighlight({
    emptyLangClass: "hljs",
    langPrefix: "hljs language-",
    highlight(code, lang, info) {
      const language = hljs.getLanguage(lang) ? lang : "plaintext";
      return hljs.highlight(code, { language }).value;
    },
  })
);

interface MarkdownRendererProps {
  content: string;
}

export const MarkdownRenderer = (props: MarkdownRendererProps) => {
  const parsedContent = createMemo(() => {
    return marked.parse(props.content, { async: false }) as string;
  });

  // 在内容渲染后添加复制按钮
  onMount(() => {
    addCopyButtons();
  });

  return <div class="markdown-body text-sm" innerHTML={parsedContent()} />;
};
