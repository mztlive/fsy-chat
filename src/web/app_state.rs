use crate::kernel::Kernel;

#[derive(Clone)]
pub struct AppState {
    kernel: Kernel,
}

impl AppState {
    pub fn new(kernel: Kernel) -> Self {
        Self { kernel }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }
}
