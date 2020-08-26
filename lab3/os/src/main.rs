//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(llvm_asm)]`  
//!   内嵌汇编
#![feature(llvm_asm)]
//!
//! - `#![feature(global_asm)]`  
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
// mod mem;
mod memory;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore-Tutorial!");
    // panic!("end of rust_main");

    // interrupt::init();

    println!("******************************************* lab2 *******************************");
    memory::init();
    // 动态内存分配测试
    use memory::alloc::boxed::Box;
    use memory::alloc::vec::Vec;

    let v = Box::new(5);
    println!("heap test passed");


    println!("{}", *memory::config::KERNEL_END_ADDRESS);//objdump -x ../os查看输出是否一致

    let mut v = Vec::<memory::frame::frame_tracker::FrameTracker>::new();

    // 物理页分配
    for _ in 0..5 {
        let frame_0 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_2 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        v.push(frame_0);
        v.push(frame_1);
        v.push(frame_2);
        // println!("{} and {}", frame_0.address(), frame_1.address());
    }
    for vv in v.iter(){
        println!("{}", vv.address());
    }
    println!("******************************************* lab3 *******************************");
    /// 1.完成页表的映射
    println!("{}", *memory::config::KERNEL_END_ADDRESS);//objdump -x ../os查看输出是否一致

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("{:?}",remap.mapping);


    println!("kernel remapped");

    /// 2.debug模式下手动验证mapping_test虚拟地址到物理地址的映射，比如虚拟地址为0xFFFFFFFF80A25098，解析的物理地址为0x80A25098
    /// 
    let mut mapping_test:i64=0x123456789;
    let addr = &mapping_test as *const i64 as usize;
    println!("vaddr：0x{:X}",addr);
    let i=1;
    while mapping_test>0 {};

    panic!();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };

    while true{};
    unreachable!();

}