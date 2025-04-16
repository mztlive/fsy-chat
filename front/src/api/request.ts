import { ApiResponse } from './types'

// 服务器基础URL
export const BASE_URL = 'http://localhost:3001/api'

// 请求方法类型
type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

// 请求配置类型
interface RequestOptions {
    method: HttpMethod
    headers?: Record<string, string>
    params?: Record<string, any>
    data?: any
}

// 基础请求方法
export async function request<T = any>(
    url: string,
    options: RequestOptions
): Promise<ApiResponse<T>> {
    // 构建完整URL，处理查询参数
    let fullUrl = `${BASE_URL}${url}`

    if (options.params) {
        const queryParams = new URLSearchParams()
        Object.entries(options.params).forEach(([key, value]) => {
            if (value !== undefined && value !== null) {
                queryParams.append(key, String(value))
            }
        })

        const queryString = queryParams.toString()
        if (queryString) {
            fullUrl += `?${queryString}`
        }
    }

    // 构建请求配置
    const fetchOptions: RequestInit = {
        method: options.method,
        headers: {
            'Content-Type': 'application/json',
            ...options.headers,
        },
        credentials: 'include', // 包含跨域请求的cookies
    }

    // 添加请求体
    if (options.data) {
        fetchOptions.body = JSON.stringify(options.data)
    }

    try {
        const response = await fetch(fullUrl, fetchOptions)

        // 解析响应
        const responseData = await response.json()

        if (response.ok) {
            return responseData as ApiResponse<T>
        } else {
            throw new Error(responseData.message || '请求失败')
        }
    } catch (error) {
        console.error('请求错误:', error)
        return {
            status: -1,
            message: error instanceof Error ? error.message : '网络错误',
            data: null as unknown as T,
        }
    }
}

// 封装GET请求
export function get<T = any>(
    url: string,
    params?: Record<string, any>,
    headers?: Record<string, string>
) {
    return request<T>(url, {
        method: 'GET',
        params,
        headers,
    })
}

// 封装POST请求
export function post<T = any>(
    url: string,
    data?: any,
    params?: Record<string, any>,
    headers?: Record<string, string>
) {
    return request<T>(url, {
        method: 'POST',
        data,
        params,
        headers,
    })
}

// 封装PUT请求
export function put<T = any>(
    url: string,
    data?: any,
    params?: Record<string, any>,
    headers?: Record<string, string>
) {
    return request<T>(url, {
        method: 'PUT',
        data,
        params,
        headers,
    })
}

// 封装DELETE请求
export function del<T = any>(
    url: string,
    params?: Record<string, any>,
    headers?: Record<string, string>
) {
    return request<T>(url, {
        method: 'DELETE',
        params,
        headers,
    })
}
