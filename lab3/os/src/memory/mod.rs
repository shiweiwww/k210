pub mod config;
mod heap;
pub mod address;
pub mod mapping;
pub mod frame;
pub use frame::allocator::FRAME_ALLOCATOR as FRAME_ALLOCATOR;
pub use heap::alloc as alloc;
/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;


// pub use config::KERNEL_END_ADDRESS as KERNEL_END_ADDRESS;
pub fn init() {
    heap::init();
    println!("mem initialized");
}

