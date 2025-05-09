use crate::kernel::Kernel;

/// 应用状态结构体
///
/// 封装了Web应用程序的全局状态，主要包含系统内核
/// 作为应用程序的核心状态，它在各个HTTP处理器之间共享，
/// 提供对系统内核及其管理的会话的访问
#[derive(Clone)]
pub struct AppState {
    /// 系统内核，管理聊天会话、文档和向量存储
    kernel: Kernel,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// # 参数
    /// * `kernel` - 系统内核实例
    ///
    /// # 返回值
    /// 返回包含指定内核的应用状态实例
    pub fn new(kernel: Kernel) -> Self {
        Self { kernel }
    }

    /// 获取系统内核引用
    ///
    /// # 返回值
    /// 返回对系统内核的只读引用
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }
}
