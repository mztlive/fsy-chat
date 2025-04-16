import { createFileRoute } from "@tanstack/solid-router";

export const Route = createFileRoute("/image")({
  component: ImageRoute,
});

function ImageRoute() {
  return (
    <div class="min-h-screen bg-gradient-to-br from-pink-500 to-purple-800 flex flex-col items-center justify-center p-4">
      <h1 class="text-4xl font-bold text-white mb-8">AI图像生成</h1>
      <div class="w-full max-w-4xl bg-white rounded-lg shadow-xl p-6">
        <div class="mb-6">
          <label class="block text-lg font-medium mb-2">输入您的描述</label>
          <textarea
            placeholder="详细描述您想要生成的图像内容..."
            class="textarea textarea-bordered w-full h-32"
          ></textarea>
        </div>

        <div class="mb-6">
          <label class="block text-lg font-medium mb-2">图像风格</label>
          <div class="flex flex-wrap gap-2">
            <button class="btn btn-outline">逼真照片</button>
            <button class="btn btn-outline">插画风格</button>
            <button class="btn btn-outline">动漫风格</button>
            <button class="btn btn-outline">水彩画</button>
            <button class="btn btn-outline">油画</button>
          </div>
        </div>

        <div class="flex justify-center">
          <button class="btn btn-secondary btn-lg">
            生成图像
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 ml-2"
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
          </button>
        </div>

        <div class="mt-8 border-t pt-6">
          <h2 class="text-xl font-medium mb-4">生成结果</h2>
          <div class="bg-gray-100 h-64 rounded-lg flex items-center justify-center">
            <p class="text-gray-400">您生成的图像将显示在这里</p>
          </div>
        </div>
      </div>
    </div>
  );
}
