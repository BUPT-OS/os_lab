# RROS 任务列表

RROS目前大概任务分类和举例如下，如果大家对改进内核和Lab有想法，也欢迎大家向我们自定义新的任务：

- RROS kernel相关：
  1. 实时调度算法：已有调度算法的调优，移植新的调度算法，调度观测机制的建立；
  2. 实时内存分配算法调优：已有分配算法调优；
  3. 实时内核功能补全：实时内核类futex机制，用户态观测/控制实时内核的机制；
  4. 在Rust-for-Linux（RFL）框架下进行实时驱动开发：gpio，monitor驱动开发；
  5. 移植内核到新的架构或者开发板下：内核移植，补丁回合，Bsp板级包构建，文件系统打包；
  6. RFL测量（学术向）：测量RFL编写driver的性能/存储开销，为提升RROS做准备；

- Lab相关：
  1. 新lab的设计：设计内核态/用户态Lab，设计Rust-for-Linux包裹Linux接口的lab，编写驱动的Lab，网络模块的Lab；
  2. 已有lab的改进：改进Lab文档/代码注释，Lab测试内容的改进；

## RROS侧

1. 任务1：补全实时内核中时钟子系统的部分Syscall[已被选，2023/7/2]
   - 时间：3周
   - 描述：目前实时内核的时钟子系统还没有给用户态用的syscall接口，只能在内核态调用实时接口，而我们编写实时程序一般需要在用户态，所以现在需要在用户态使用系统调用导出实时接口
   - 要求：
     - 仿照evl的kernel/evl/clock.c代码，在rros的时钟子系统中添加EVL_CLKIOC_GET_RES，EVL_CLKIOC_GET_TIME，EVL_CLKIOC_SET_TIME，EVL_CLKIOC_SLEEP四个系统调用；
     - 添加的系统调用可以通过libevl中的lib/clock.c相关的测试任务；
   - 提示
     - 运行qemu+raspi树莓派文件系统+libevl的文档
     - 可以参考rros中kernel/rros/proxy.rs或者kernel/rros/thread.rs的系统调用是如何实现的
     - [evl的项目链接](https://github.com/rust-real-time-os/xenomai_sourcecode)
     - [libevl的项目链接](https://github.com/rust-real-time-os/libevl/tree/r27_net)
   - 导师：李弘宇 微信：13935084378
   - 技术栈：
     - rust
     - C
     - linux
     - rust-for-linux

2. 任务2：测量RFL社区driver的性能（学术向）
   - 时间：2周
   - 描述：目前RFL社区用Rust重写了Linux社区的很多driver，如果能够系统地测量这些driver的性能，比如说网络设备的latency，磁盘设备的读写性能，对我们进一步用rust开发driver有很大的意义。
   - 要求：
     - 测量以下driver的性能
       - https://lore.kernel.org/rust-for-linux/87mt2fae4i.fsf@metaspace.dk/T/#t
       - https://lore.kernel.org/rust-for-linux/87mt29e9vb.fsf@metaspace.dk/T/#t
       - https://lore.kernel.org/rust-for-linux/20230609063118.24852-1-amiculas@cisco.com/T/#mebb060381c0071b913228a9892d464a7dfc27a29
     - 为保证测量结果的准确性，需要在物理pc上测试
     - 具体测量指标需要和导师联系；
   - 提示
     - 需要在RFL的最新代码上回合以上driver补丁，然后编译RFL内核，替换物理pc内核；
   - 导师：李弘宇 微信：13935084378
   - 技术栈：
     - rust
     - C
     - linux
     - rust-for-linux

## lab侧

1. 任务1：让网站支持通过HTTPS传输
   - 时间： 1周
   - 描述：目前网站仅支持HTTP协议，需要进行一定的配置以支持HTTPS协议。
   - 要求：
     - 提供能够运行的脚本和相应的说明（比如修改Dockerfile和一些script）
     - 在服务器上部署
   - 提示：
     - 网站使用uvicorn作为ASGI，也可以添加一层nginx，然后参考网上教程
   - 导师：邱奇琛 微信：ruiqurm
   - 技术栈：
     - python
     - https
     - docker

2. 任务2：使用异步方式查询数据库、启动容器
   - 时间： 2周
   - 描述：目前项目使用sqlalchemy查询和修改postgres数据库，使用docker库启动容器。使用同步的方式会导致服务器有时无法即使响应新的请求。我们希望将旧的接口替换为异步的接口。
   - 要求：
     - 修改相关代码，并在本地运行
     - 在服务器上成功部署
   - 提示：
     - sqlalchemy支持启动一个async session，然后进行异步操作。
     - aiodocker是一个异步操作docker接口的库。可以将目前的接口替换为aiodocker的接口。
   - 导师：邱奇琛 微信：ruiqurm
   - 技术栈：
     - python
     - python asyncio
