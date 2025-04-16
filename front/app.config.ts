import { defineConfig } from "@solidjs/start/config";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import tailwindcss from '@tailwindcss/vite';


export default defineConfig({
  ssr: false,
  vite: {
    plugins: [TanStackRouterVite({ target: "solid" }), tailwindcss()],
  }
});
