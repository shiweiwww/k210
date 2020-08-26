# Step 2
### 1.week1[8.3-8.9]

  - #### day-8月10号-8月15
    * ##### lab3迁移
    * ##### maix go如何进行调试
  
  - #### Day-8月5日
    * ##### K210开发版跑rcore-tutorial的lab2
    * ##### 尝试lab0和lab1，目前尚未成功
    * ##### k210开发版上部署maixpy的固件，实现简单的人脸识别，拍照功能，但板子并没有摄像头以及显示屏，打算自购一套
    * ##### 参加关于maixpy会议，拆分计划如下
      + ##### step 0：完成lab0-lab6在k210开发板的迁移，完成rcore-tutorial迁移
      + ##### step 1：把k210板sdk管理kpu设备迁移到由rcore-tutorial管理kpu，要求能跑通人脸识别检索类似的Ai程序
      + ##### step 2：maixpy性能和rcore性能对比，具体怎么做，待定，完成step0和step1后再定
  - #### day-8月3号-4号
    * ##### 参加相应的培训以及明确要做的事情

### note
  - #### 由于工作的异动，目前无太多时间在k210上完成相关的AI功能，现记录目前的进展以及后续的一些思路,方便感兴趣的同学可以接着完成
  - #### 目前已经可以k210上跑rcore(参考其他同学的代码即可)
  - #### k210调试问题，maix go自带stm32芯片可以模拟jtags，但是默认的固件是open-ec，需要烧固件CMSIS-DAP，具体[参考](https://cn.maixpy.sipeed.com/dev/zh/develop_kit_board/get_hardware.html?h=调试)，其他maix开发版调试[参考文档](https://cn.maixpy.sipeed.com/dev/zh/develop_kit_board/get_hardware.html?h=调试)
  - #### TODO
    - #### 1. k210支持[maixpy](https://github.com/sipeed/MaixPy)，底层调用的是C的接口，最简单的方式安装riscv和c的交叉[编译工具](https://github.com/riscv/riscv-gnu-toolchain),把k210的AI的C代码直接编译成elf文件，这个elf功能是对k210的AI硬件的管理,可能需要精简下代码，毕竟k210只有6M
    - #### 2. 步骤1把maixpy中的k210的AI硬件打包成elf，接下来下如何让rcore实现使用这个elf文件管理AI硬件,最底层都是riscv的汇编，一种方式使用rust的一些包直接调用(具体百度目前不清楚)；一种方式是直接写汇编调用即可，类似于c和x86汇编相互调用一样
    - #### 3. 经过步骤2,rcore可以管理起k210的AI相关的硬件功能了，在用户态写rust测试代码，添加系统调用就应该可以使用k210的AI相关的功能了
    - #### 4. 如何写rust或者c的测试代码，这个部分需要追踪maixpy应用层的代码，实现类似的操作就行，比如处理一张图片，拍照功能等
    - #### 5. Ai功能的功能依赖于上述4步骤，模型文件放在用户态加载，完成AI测试功能
    - #### 6. 提高kpu利用率，主要是利用起来k210双核的能力实现批量数据预处理以及异步的方式去使用kpu，降低kpu等待数据的时间


      
  <!-- - #### Day-7月5日
    * #####  rust视频观看(B站令狐一冲)
  - #### Day-7月6日
    * #####  rust by example(1-12章学习)
  - #### Day-7月7日
    * #####  rust by example(完成)
    * #####  rustlings(70%完成)
  - #### Day-7月8日
    * #####  完成rustlings，对option，result理解不是很好，需要多练习下相关编程的代码
    * #####  RISC-V指令集的学习以及riscv-tools安装，尚未安装成功
  - #### Day-7月9日
    * #####  RISC-V指令集的学习以及riscv-tools安装，编译成功，目前汇编感觉问题不大，卡在如何riscv汇编输出字符串到屏幕上，待解决
  - #### Day-7月10日
    * #####  RISC-V指令集的学习,不清楚riscv汇编如何向屏幕输出字符串，目前已经提了issue
    * #####  rust15道题，目前做了4道，视频学习资料已经看完，我计划是两周内完成riscv汇编学习已经rust相关练习题
      - ###### 1.[链表，静态双向链表，动态双向链表实现](https://github.com/shiweiwww/rcore/tree/master/exercis/exe1)
      - ###### 2.[栈和队列实现](https://github.com/shiweiwww/rcore/tree/master/exercis/exe2)
      - ###### 3.[一个简单的统计引擎](https://github.com/shiweiwww/rcore/tree/master/exercis/exe3)
      - ###### 4.[输入输出和文件](https://github.com/shiweiwww/rcore/tree/master/exercis/exe4) -->
