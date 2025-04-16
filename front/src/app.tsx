import { router } from "./router";
import { RouterProvider } from "@tanstack/solid-router";

import "./app.css";
import { Meta, MetaProvider } from "@solidjs/meta";

export default function App() {
  return (
    <MetaProvider>
      <Meta title="AI助手" />
      <Meta
        name="viewport"
        content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
      />
      <Meta name="apple-mobile-web-app-capable" content="yes" />
      <Meta
        name="apple-mobile-web-app-status-bar-style"
        content="black-translucent"
      />
      <RouterProvider router={router} />
    </MetaProvider>
  );
}
