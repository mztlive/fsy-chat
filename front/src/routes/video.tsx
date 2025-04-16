import { createFileRoute } from "@tanstack/solid-router";

export const Route = createFileRoute("/video")({
  component: VideoRoute,
});

function VideoRoute() {
  return (
    <div class="min-h-screen bg-gradient-to-br from-amber-500 to-orange-700 flex flex-col items-center justify-center p-4">
      <h1 class="text-4xl font-bold text-white mb-8">AI视频生成</h1>
      <div class="w-full max-w-4xl bg-white rounded-lg shadow-xl p-6">
        <div class="mb-6">
          <label class="block text-lg font-medium mb-2">场景描述</label>
          <textarea
            placeholder="详细描述您想要生成的视频场景和内容..."
            class="textarea textarea-bordered w-full h-32"
          ></textarea>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
          <div>
            <label class="block text-lg font-medium mb-2">视频风格</label>
            <select class="select select-bordered w-full">
              <option value="" disabled selected>
                选择视频风格
              </option>
              <option value="realistic">写实风格</option>
              <option value="animation">动画风格</option>
              <option value="abstract">抽象艺术</option>
              <option value="cinematic">电影风格</option>
            </select>
          </div>

          <div>
            <label class="block text-lg font-medium mb-2">视频时长</label>
            <select class="select select-bordered w-full">
              <option value="5">5秒</option>
              <option value="10" selected>
                10秒
              </option>
              <option value="15">15秒</option>
              <option value="30">30秒</option>
            </select>
          </div>
        </div>

        <div class="flex justify-center">
          <button class="btn btn-accent btn-lg">
            生成视频
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
                d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
              />
            </svg>
          </button>
        </div>

        <div class="mt-8 border-t pt-6">
          <h2 class="text-xl font-medium mb-4">生成结果</h2>
          <div class="bg-gray-100 h-80 rounded-lg flex items-center justify-center">
            <div class="text-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-16 w-16 mx-auto text-gray-400 mb-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                />
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <p class="text-gray-400">您生成的视频将显示在这里</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
