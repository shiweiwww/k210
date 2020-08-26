### lab6实验报告
 ##### 一. 实验目的和内容
  - ###### 构建用户程序框架
  - ###### 解析 ELF 文件并创建线程
 
 ##### 二. 操作方法和实验步骤
  - ###### 参考实验指导书中线程和进程的介绍
  - ###### 实验
    * ##### 1.原理：使用条件变量之后，分别从线程和操作系统的角度而言读取字符的系统调用是阻塞的还是非阻塞的
        + ###### 对于线程而言是阻塞的，io完成读后触发条件变量通知操作系统；对系统而言是非阻塞的，等待io的时候os会执行其他线程
    * ##### 2.设计：如果要让用户线程能够使用 Vec 等，需要做哪些工作？如果要让用户线程能够使用大于其栈大小的动态分配空间，需要做哪些工作？
        + ###### vec动态空间在内核.bss段，用户线程访映射的空间是内核之上的，把动态分配空间移动到kernel_end以上的某个空间就可以，其他的和内核使用vec等没什么区别;应该和String，Box使用没什么区别，记录开始的指针和长度，把这两项压入栈
    * ##### 3.实验：实现 get_tid 系统调用，使得用户线程可以获取自身的线程 ID
        + ###### 定义user态的get_id和sys_get_id函数，代码如下
        ```rust
        /// user/src/syscall.rs
        const SYSCALL_GETID: usize = 101;
        pub fn sys_get_id()->isize{
            let ret = syscall(SYSCALL_GETID,0,0,0);
            return ret;
        }
        /// user/src/hello_world.rs
        let tid = get_id();
        println!("xxxxxxxxx thread id is:{}",tid);
        ..
        ..
        /// os/src/kernel/process.rs
        pub (super) fn sys_get_id()->SyscallResult{
            let ret = PROCESSOR.get().current_thread().id;
            return SyscallResult::Proceed(ret);
        }

        ```
    * ##### 4.实验：基于你在实验四（上）的实践，实现 sys_fork 系统调用。该系统调用复制一个进程，并为父进程返回 1（目前没有引入进程 ID，也可以自行补充为进程 ID），而为子进程返回 0。
        + ###### 系统调用类似实验3题，定义sys_fork，不再列出,fork代码如下,还是fork了当前线程，fork进程需要复制页表，后面有时间再补上
        ```rust
            pub fn fork(&self,context:&Context)->MemoryResult<Arc<Thread>> {
                let stack = self.process
                    .write()
                    .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

                for p in 0..STACK_SIZE{
                    *VirtualAddress(stack.start.0+p).deref::<u8>()=*VirtualAddress(self.stack.start.0+p).deref::<u8>()
                }
                let mut new_context = context.clone();
                let s:usize = stack.start.into();
                let e:usize = self.stack.start.into();
                new_context.set_sp(s+context.sp()-e);
                // // 打包成线程
                let thread = Arc::new(Thread {
                    id: unsafe {
                        THREAD_COUNTER += 1;
                        THREAD_COUNTER
                    },
                    stack,
                    process:self.process.clone(),
                    inner: Mutex::new(ThreadInner {
                        context: Some(new_context),
                        sleeping: false,
                        dead: false,
                        descriptors: vec![STDIN.clone(), STDOUT.clone()],
                    }),
                });
                Ok(thread)

            }
        ```

 ##### 三. 实验结果和分析
  - ###### 代码流程分析 
      * ###### 打包磁盘镜像,运行如下代码,为目标下载一个标准库和core库，然后运行make build就行
      ```rust
      rustup target add riscv64imac-unknown-none-elf
      ```
      * ###### 从磁盘解析elf文件,Process::from_elf最终调用MemorySet::from_elf完成elf文件的解析，建立内核映射的MemorySet和elf每个段进行映射(需要把数据copy到对于的物理地址中去)，Tread::new创建线程，并把elf入口地址传进去就可，其他的过程和lab4没什么区别
      ```rust
      pub fn create_user_process(name: &str) -> Arc<Thread> {
          let app = ROOT_INODE.find(name).unwrap();
          let data = app.readall().unwrap();
          // 解析 ELF 文件
          let elf = ElfFile::new(data.as_slice()).unwrap();
          // 利用 ELF 文件创建线程，映射空间并加载数据
          let process = Process::from_elf(&elf, true).unwrap();
          // 再从 ELF 中读出程序入口地址
          Thread::new(process, elf.header.pt2.entry_point() as usize, None).unwrap()
      }
      ```
      * ###### 解析磁盘上的hello_world程序，hello_world中调用了println!本质上是执行了系统调用sys_write，sys_write底层是ecall的调用，产生一个Exception::UserEnvCall中断，由kernel/syscall/syscall_handler来处理，context通过syscall_id来决定调用sys_write
      ```rust
      let syscall_id = context.x[17];
      let args = [context.x[10], context.x[11], context.x[12]];

      let result = match syscall_id {
          SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
          SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
          SYS_EXIT => sys_exit(args[0]),
          _ => {
              println!("unimplemented syscall: {}", syscall_id);
              SyscallResult::Kill
          }
      };
      ```
      ###### sys_write向STDOUT写入字符串，调用的是inode.write_at,STDOUT代表终端市场
      ```rust
      pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
      if let Some(inode) = PROCESSOR.get().current_thread().inner().descriptors.get(fd) {
          let buffer = unsafe { from_raw_parts_mut(buffer, size) };
          if let Ok(ret) = inode.write_at(0, buffer) {
              let ret = ret as isize;
              if ret >= 0 {
                  return SyscallResult::Proceed(ret);
              }
          }
      }
      SyscallResult::Proceed(-1)
      }
      ```
 ##### 四. 问题建议以及改进的地方
  - ###### 自己按照实验指导做了下，似乎是进程那块有点问题，卡住了，只能看了下跑了下master的代码
  <!-- - ###### 实验题目前先不做，先刷一遍整体对代码有理解在刷吧 -->

