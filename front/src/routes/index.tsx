import { createFileRoute, Link } from "@tanstack/solid-router";

export const Route = createFileRoute("/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class="min-h-screen bg-base-200 flex flex-col items-center justify-center p-8">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8 w-full max-w-5xl">
        {/* 聊天功能 */}
        <div class="bg-base-100 rounded-lg shadow-md p-6 hover:shadow-lg transition-all">
          <Link to="/chat">
            <img
              src="/images/chat.jpg"
              alt="DeepSeek Chat"
              class="w-full h-30 object-cover rounded-md cursor-pointer"
            />
          </Link>
        </div>

        {/* 文生图功能 */}
        <div class="bg-base-100 rounded-lg shadow-md p-6 hover:shadow-lg transition-all">
          <Link to="/image">
            <img
              src="/images/image.jpg"
              alt="文生图"
              class="w-full h-30 object-cover rounded-md cursor-pointer"
            />
          </Link>
        </div>

        {/* 文生视频功能 */}
        <div class="bg-base-100 rounded-lg shadow-md p-6 hover:shadow-lg transition-all">
          <Link to="/video">
            <img
              src="/images/video.jpg"
              alt="文生视频"
              class="w-full h-30 object-cover rounded-md cursor-pointer"
            />
          </Link>
        </div>
      </div>
    </div>
  );
}
