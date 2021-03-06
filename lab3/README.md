### lab3实验报告
 ##### 一. 实验目的和内容
  - ###### 虚拟地址和物理地址
  - ###### 页表实现
 
 ##### 二. 操作方法和实验步骤
  - ###### 虚拟地址和物理地址
    * ###### Sv39模式下，物理地址56位，0-11是offset，12-55位为物理页号ppn。对虚拟地址64位，0-11位offset，12-38位为虚拟页号vpn。Sv39 模式里面的一个页表项大小为 64 位（即 8 字节）。其中第 53-10 共 44 位为一个物理页号，表示这个虚拟页号映射到的物理页号。后面的第 9-0 位则描述页的相关状态信息。 页表的基址（起始地址）保存在一个特殊的寄存器satp中
  - ###### 页表实现
    * ###### Sv39模式下需要三级页表才可以从虚拟地址映射到物理地址，虚拟地址12-38位每9位一组划分成vpn2,vpn1,vnp0,从satp中获取第3三级页表的物理页号(satp的0-43位)base,得到对应的物理地址为：
    ```
    [{[(base<<12+vpn2*8)>>10 & 0xfffffffffff)<<12+vpn1*8]>>10 & 0xfffffffffff}>>10 & 0xfffffffffff]<<12 + offset(虚拟地址0-11位)。
    ```
  - ###### 启动程序，执行如下代码，通过i虚拟地址Debug调试下i的物理地址
    ```rust
        let i:f64=0x12345678;
        loop{};
    ```
  - ###### 实验
    * ##### 原理：在 os/src/entry.asm 中，boot_page_table 的意义是什么？当跳转执行 rust_main 时，不考虑缓存，硬件通过哪些地址找到了 rust_main 的第一条指令？
      + ###### lds脚本修改内核第一条指令地址为0xffffffff80200000，qemu启动opensbi跳转的地址为0x80200000,boot_page_table建立页表，虚拟地址到物理地址建立映射关系，也为了正常加载内核代码；执行jarl rust_main后，跳转到rust_main地址0xffffffff8xx...,从satp地位读取低0-43位作为页表页号(boot_page_table),获取虚拟地址页号vpn2,为510,查看boot_page_table的第510页表项，读取二级页表物理页号，因为boot_page_table的XWR全为1，说明是指向一个大页，同时V标志位也为1,说明在内存中，物理地址为页表项中物理页号<<12+偏移地址12位找到物理地址
    * ##### 分析：为什么 Mapping 中的 page_tables 和 mapped_pairs 都保存了一些 FrameTracker？二者有何不同？
      + ###### page_tables中保存的是所有使用到的页表的FrameTracker, mapped_pairs保存的是进程的虚拟地址和物理地址映射关系
    * ##### 分析：假设某进程需要虚拟地址 A 到物理地址 B 的映射，这需要操作系统来完成。那么操作系统在建立映射时有没有访问 B？如果有，它是怎么在还没有映射的情况下访问 B 的呢？
      + ###### 建立映射不需要访问B,操作页表就行。整个物理空间和内核建立了线性映射，可以通过加上偏移地址0xffffffff00000000来访问
    * ##### 实验：了解并实现时钟页面置换算法（或任何你感兴趣的算法），可以自行设计样例来比较性能
      + ###### 未做


 ##### 三. 实验结果和分析
  - ###### 代码流程分析 
      * ###### PageTableEntry页表项主要是设置页表项各个bit的设置
      * ###### PageTable是页表一个类型为PageTableEntry大小为512的数组
      * ###### PageTableTracker用来管理页表PageTable
      * ###### Segment定义页表映射方式以及虚拟地址范围，主要考虑到内核是放在一段高虚拟地址空间，对所有的进程都一样的，所以线性映射就行，对用户程序而言，看到的虚拟地址空间是一样的(地址隔离)，即所有的用户程序虚拟地址空间都是一样的，基于页分配方式去映射而非线性映射
      * ###### MemorySet封装了映射关系mapping以及要映射的段segments，segments是个Vec\<Segment\>，Memoryset::new_kernel中把各个内核段text_start，rodata_start，data_start，bss_start封装成Segment并push到segments中，同时初始化mapping。mapping.map进行映射，先是mapping.find查找虚拟页号对应的页表项，页表项为空，则新建页表，并填充页表项，进入下一级页表，执行同样的操作，注意的是页表的查询是用了虚拟地址了，页表项中是物理页号，计算下一级页表base时候转换成虚拟地址才行。

 ##### 四. 问题建议以及改进的地方
  - ###### Debug 时候gdb发现无法查看物理内存，给调试页表带来不便
  - ###### rust语法的还是不是很熟练，昨晚lab3同时补了不少rust Deref，From，Into以及宏的语法知识
  - ###### 目前页面置换算法没去做，涉及到进程线程，后面实验在补上
  <!-- - ###### 实验题目前先不做，先刷一遍整体对代码有理解在刷吧 -->

