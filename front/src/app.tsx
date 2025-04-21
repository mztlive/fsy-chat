import { router } from './router'
import { RouterProvider } from '@tanstack/solid-router'
import { QueryClient, QueryClientProvider } from '@tanstack/solid-query'
import './app.css'
import { Meta, MetaProvider } from '@solidjs/meta'

export default function App() {
    const queryClient = new QueryClient()

    return (
        <MetaProvider>
            <Meta title="AI助手" />
            <Meta
                name="viewport"
                content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
            />
            <Meta name="apple-mobile-web-app-capable" content="yes" />
            <Meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
            <QueryClientProvider client={queryClient}>
                <RouterProvider router={router} />
            </QueryClientProvider>
        </MetaProvider>
    )
}
