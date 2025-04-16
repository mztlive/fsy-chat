import { createFileRoute } from "@tanstack/solid-router";

export const Route = createFileRoute("/about")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class="min-h-screen bg-gradient-to-br from-indigo-900 to-purple-900 py-12 px-4">
      <div class="max-w-4xl mx-auto bg-white rounded-lg shadow-xl overflow-hidden">
        <div class="bg-primary text-white p-6">
          <h1 class="text-3xl font-bold">关于AI助手</h1>
        </div>

        <div class="p-6">
          <section class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">我们的愿景</h2>
            <p class="text-gray-700">
              AI助手旨在利用最先进的人工智能技术，为用户提供全方位的AI体验，让每个人都能轻松使用AI技术提升工作效率与创造力。
            </p>
          </section>

          <section class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">核心功能</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div class="border rounded-lg p-4">
                <div class="flex items-center mb-3">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6 text-primary mr-2"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"
                    />
                  </svg>
                  <h3 class="text-lg font-medium">智能聊天</h3>
                </div>
                <p class="text-gray-600">
                  基于先进大语言模型的对话系统，可以回答问题、提供建议、协助创作等。
                </p>
              </div>

              <div class="border rounded-lg p-4">
                <div class="flex items-center mb-3">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6 text-primary mr-2"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                    />
                  </svg>
                  <h3 class="text-lg font-medium">文生图</h3>
                </div>
                <p class="text-gray-600">
                  通过文字描述生成高质量图像，支持多种风格调整，满足创意需求。
                </p>
              </div>

              <div class="border rounded-lg p-4">
                <div class="flex items-center mb-3">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6 text-primary mr-2"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
                    />
                  </svg>
                  <h3 class="text-lg font-medium">文生视频</h3>
                </div>
                <p class="text-gray-600">
                  将文字描述转化为流畅视频，赋能内容创作者和营销需求。
                </p>
              </div>
            </div>
          </section>

          <section class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">技术优势</h2>
            <ul class="list-disc pl-5 space-y-2 text-gray-700">
              <li>采用最新的大语言模型和多模态AI技术</li>
              <li>高度优化的用户界面，简单易用</li>
              <li>强大的API接口，支持与其他系统集成</li>
              <li>持续更新的模型能力，不断提升生成质量</li>
              <li>隐私保护设计，保障用户数据安全</li>
            </ul>
          </section>

          <section>
            <h2 class="text-2xl font-semibold mb-4">联系我们</h2>
            <p class="text-gray-700">
              如果您有任何问题、建议或合作意向，请随时联系我们：
              <br />
              <a
                href="mailto:contact@aiassistant.com"
                class="text-primary hover:underline"
              >
                contact@aiassistant.com
              </a>
            </p>
          </section>
        </div>
      </div>
    </div>
  );
}
