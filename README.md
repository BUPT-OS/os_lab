# 实时操作系统基本知识

<!-- TOC -->

- [实时操作系统基本知识](#%E5%AE%9E%E6%97%B6%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F%E5%9F%BA%E6%9C%AC%E7%9F%A5%E8%AF%86)
    - [Linux实时操作系统](#linux%E5%AE%9E%E6%97%B6%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F)
        - [双内核操作系统基本知识](#%E5%8F%8C%E5%86%85%E6%A0%B8%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F%E5%9F%BA%E6%9C%AC%E7%9F%A5%E8%AF%86)
        - [Xenomai操作系统](#xenomai%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F)
        - [rros](#rros)
    - [rros编译](#rros%E7%BC%96%E8%AF%91)
        - [docker安装及拉取代码](#docker%E5%AE%89%E8%A3%85%E5%8F%8A%E6%8B%89%E5%8F%96%E4%BB%A3%E7%A0%81)
        - [rros编译过程](#rros%E7%BC%96%E8%AF%91%E8%BF%87%E7%A8%8B)
    - [使用Qemu进行模拟同时使用gdb对进行Debug](#%E4%BD%BF%E7%94%A8qemu%E8%BF%9B%E8%A1%8C%E6%A8%A1%E6%8B%9F%E5%90%8C%E6%97%B6%E4%BD%BF%E7%94%A8gdb%E5%AF%B9%E8%BF%9B%E8%A1%8Cdebug)
    - [引用](#%E5%BC%95%E7%94%A8)

<!-- /TOC -->

## Linux实时操作系统

Linux是一个通用操作系统，而不是一个实时操作系统。目前有多种方法可以让Linux成为一个实时操作系统或者具备实时操作系统的能力：

- 一个广泛使用的方案是让给实时操作系统提供兼容POSIX的API接口来改善生态问题，但是这个方法在面对大型项目的移植时的效果不好，也没有解决性能问题；
- 另一条技术路线是可以对Linux内核采用抢占补丁，但是这个方法只能让Linux成为一个软实时的操作系统，同时没有任何的隔离措施，不利于稳定性；
- 还可以采用虚拟机来同时在一个主机上同时运行多个内核，一个是较为简单的硬实时内核，另一个是Linux内核，这个方法的稳定性很强，但是由于采用了硬件虚拟化，实时性和性能受损，同时在Linux内核上运行的应用不具备实时性能，在实时内核上运行的应用不能享受到Linux生态的好处，两者之间的数据交互比较困难；
- 最后一条路线是采用双内核的方法，在一个内核空间里面同时运行两个内核，实时内核和Linux同时并行，实时内核的优先级更高，Linux内核作为一个idle任务在实时内核中调度，这个方法同时兼顾了实时性，性能和应用生态，实时任务可以同时利用Linux的生态和实时应用的能力，但是这个方法的缺点是稳定性不足，因为两个内核同处一个地址空间，所以如果Linux内核出现故障，没有任何的隔离措施，很容易导致操作系统崩溃；

我们的rros（rust-based real operating system）就是采用了双内核的技术路线，下面首先介绍一下双内核操作系统。

### 双内核操作系统基本知识

双内核操作系统主要分为两部分，一个是硬件虚拟层（HAL），主要完成中断虚拟化等相关工作；第二个是实时内核，主要负责处理实时请求，和Linux内核在逻辑上是并列的关系，但是优先级要比Linux内核高。

双内核的实现主要分为RTLinux，RTAI，Xenomai三个项目。
双内核最早的实现RTLinux是在[1]这篇论文中提出的。
硬件虚拟层是双内核路线中的关键一环，RTHAL是在RTLinux的论文中提出来的，被另一个实时操作系统RTAI仿照实现，但是RTLinux的项目组后来申请了专利，RTAI被迫采用了其他硬件虚拟层技术ADEOS。
ADEOS是在[2]这篇论文中提出的，采用不同方法换了RT-Linux提出的RTHAL，规避了专利问题。所以后来RTAI和Xenomai社区采用了ADEOS。

> 拓展阅读
> ADEOS的实现细节在[3]中可以看到；  
> ADEOS和RTHAL两种硬件虚拟层技术的比较在[4]中可以看到；  

RTLinux自从被同类竞品VxWorks收购后，已经从开源逐步走向关停。
而Xenomai和RTAI两个项目在一段时间有过短暂的合并。但是后来因为开发的目标不同，两个项目又逐渐分离。rros主要仿照的就是Xenomai操作系统。

### Xenomai操作系统

[Xenomai](https://evlproject.org/)目前已经进展到4.0了。
Xenomai4.0主体是两个部分，硬件中断层dovetail和实时内核evl。
dovetail主要是以代码树形式提供，直接修改了Linux内核的代码，作用是根据优先级将中断分发给cobalt内核和evl内核。
实时内核evl则作为一个module插入到linux系统中，和Linux内核一起启动，启动后接管整个操作系统。

### rros

rros采用了dovetail硬件中断层，用rust重写了实时内核evl，下面我们将会介绍rros的基本情况。

项目的代码树主体结构如下，相对于Linux代码树新增的文件用*标出，一些重要的目录或者文件我们加以解释：

```
.
├── arch
├── block
├── certs
├── COPYING
├── CREDITS
├── crypto
├── .config                       这个文件中包含了Linux编译时
├── Documentation                 Linux项目的文档
├── drivers
├── fs
├── gr                            * 包含了运行rust-gdb命令的文件
├── include
├── init
├── io_uring
├── ipc
├── Kbuild
├── Kconfig
├── kernel                        Linux内核的主要代码
│   ├── acct.c
│   ├...
│   ├── rros                      * rros实时内核的主要代码
│   |   ├── built-in.a
│   |   ├── clock.rs
│   |   ├── clock_test.rs
│   |   ├── crossing.rs
│   |   ├── double_linked_list_test.rs
│   |   ├── factory.rs
│   |   ├── fifo.rs
│   |   ├── fifo_test.rs
│   |   ├── file.rs
│   |   ├── idle.rs
│   |   ├── init.o
│   |   ├── init.rs
│   |   ├── libinit.rmeta
│   |   ├── list.rs
│   |   ├── list_test.rs
│   |   ├── lock.rs
│   |   ├── Makefile
│   |   ├── memory.rs             内存子系统
│   |   ├── modules.order
│   |   ├── monitor.rs
│   |   ├── net.rs
│   |   ├── queue.rs
│   |   ├── sched.rs              调度子系统      
│   |   ├── sched_test.rs
│   |   ├── stat.rs
│   |   ├── stat_test.rs
│   |   ├── syscall.rs
│   |   ├── test.rs
│   |   ├── thread.rs             线程子系统
│   |   ├── thread_test.rs
│   |   ├── tick.rs               tick子系统
│   |   ├── timeout.rs
│   |   ├── timer.rs              时钟子系统
│   |   ├── timer_test.rs
│   |   ├── wait.rs
│   |   └── weak.rs
│   ├...
│   └── workqueue.o
├── lib
├── LICENSES
├── MAINTAINERS
├── Makefile
├── mm
├── modules.builtin
├── modules.builtin.modinfo
├── modules.order
├── Module.symvers
├── net
├── README
├── rust                          * rust-for-linux的代码         
├── samples                       包含了各个子系统的一些示例代码
├── scripts
├── security
├── sound
├── System.map
├── tools
├── usr
├── virt
├── vmlinux
├── vmlinux.a
└── vmlinux.o
```

因为dovetail硬件中断层已经合入到代码树中，通过`git log`的历史记录是看不出来的，如果想要知道dovetail修改了哪些内容，可以从[patch-5.15.9-dovetail1.patch](https://xenomai.org/downloads/dovetail/patch-5.15.51-dovetail1.patch.bz2)这个patch中看到。

rust的支持是通过rust-for-linux（rfl）项目[5]，rfl目前已经合入linux主线，由于历史原因，我们项目中rfl的支持是通过补丁的形式进行的，和目前主线上的rfl不兼容。rfl项目支持我们用rust写linux的驱动，我们的rros就是以驱动的形式加载到linux内核中的。

## rros编译

为了方便大家做实验，我们以docker的形式提供一个可用的环境，大家只需要安装docker并拉取镜像，然后按照我们的编译说明进行编译，就可以做后续的实验了。

### docker安装及拉取代码

docker在windows/linux/mac上都可以直接安装，主要参考[官方的文档](https://docs.docker.com/desktop/)，具体步骤方法如下：

- 在windows上安装  
  - 下载[docker desktop](https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe)，并点击安装。
  - 安装完docker后，如果提示因为wsl2没有安装不能正常启动的话，这是因为在windows上使用docker需要开启wsl2或者hyper-v相关的组件，我们这里采用wsl2，这部分内容参考微软的[官方说明](https://learn.microsoft.com/zh-cn/windows/wsl/install-manual#step-4---download-the-linux-kernel-update-package)。
    - 以管理员身份打开 PowerShell（“开始”菜单 >“PowerShell” >单击右键 >“以管理员身份运行”），然后输入以下命令：
      ```powershell
      dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
      dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart
      ```
    - 下载[wsl更新包](https://wslstorestorage.blob.core.windows.net/wslblob/wsl_update_x64.msi)，并安装执行。
  - 重启电脑，并启动docker desktop，就可以正常启动了
    ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118024109.png)
- 在linux ubuntu/mac上安装  
  - 这里考虑到linux上安装时可能没有图形化界面，所以下面用命令行说明
  - linux上运行docker的原理是使用kvm虚拟化技术，可以使用下列命令检测linux是否满足docker的条件
    ```bash
    lsmod | grep kvm
    ```
    正确的输出如下:
    ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118030849.png)
  - 对linux软件包安装地址进行换源
    ``` bash
    sudo add-apt-repository "deb [arch=amd64] https://mirrors.aliyun.com/docker-ce/linux/ubuntu $(lsb_release -cs) stable"
    sudo apt-get update
    ```
  - 安装docker
    ```bash
    sudo apt-get install docker-ce
    sudo docker run hello-world
    ```
  - docker换源
    ```bash
    vim /etc/docker/daemon.json
    # 加入下面的内容
    # {
    #   "registry-mirrors": ["https://akchsmlh.mirror.aliyuncs.com"]
    # }    
    ```
  - 检查是否可以正常执行
    ``` bash
    sudo docker run hello-world
    ```
    ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118030247.png)

安装完成后，打开命令行窗口，使用`docker pull l543306408/rros_lab`命令来拉取rros docker的镜像image。接着使用`docker run -itd --security-opt seccomp=unconfined --name rros_lab l543306408/rros_lab /bin/bash`来运行一个名为rros_lab的container。

最后我们利用vscode来完成后续实验。
- 如果你的docker是运行在本机上，而不是远程的Linux服务器，只需要在vscode应用市场中安装`dev-container`插件：
  ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230223105223.png)
- 点击插件后，我们就可以看到我们运行起来的docker了，然后点击`Attach in New Window`进入我们的docker
  ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230223100654.png)
- 然后在container中打开我们项目的文件夹
  ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118033949.png)
- 输入`/data/bupt-rtos/rros`
  ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230223101753.png)


> 远端的Linux配置方法如下：
> - 如果你的docker运行在远程的Linux服务器，需要安装`remote-ssh`插件
>   ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118032735.png)
> - 需要先配置一下ssh
>   ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118033530.png)
> - 将下图的`ip_address`换成Linux服务器的ip地址，`ssh_port`设置为对应ssh的服务端口（一般是22），`user_name`替换成Linux服务器的用户名字
>   ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118033325.png)
> - 最后打开配置好的远端服务器，之后和在本地vscode打开docker容器的步骤一致
> ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118033629.png)

### rros编译过程

首先，rros的编译和linux编译的方法相似，都是通过Kconfig系统。
对于rros比linux多出的dovetail和rfl子系统，都可以通过在主目录下执行配置menuconfig的命令进行开启，对于其他的子系统也可以在同时进行配置，配置好的结果会保存到`.config`文件中。
![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118011936.png)
为了方便大家做后面的实验，我们提供一个已经配置好的`.config`文件，不需要大家手动配置。并且这个`.config`中的选项经过了剪裁，所以操作系统编译的速度会大大加快。如果大家想要体会手动配置config的过程，可以参考下面的编译tips中第四点。

然后，可以用`make LLVM=1 -j80 >compile.txt 2>&1 && tail -10 compile.txt`对整个操作系统进行编译，这个命令后面的重定向是由于目前项目中有大量的warning没有被消除，所以我们最好把编译的结果保存到一个compile.txt文件中。如果代码中没有错误，这个命令会输出10行编译信息如下图所示。
![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118014307.png)
> 下面的`/data/bupt-rtos/linux-dovetail-v5.13-dovetail-rebase`路径和大家看到的`/data/bupt-rtos/rros`路径是等价的。  

如果代码中有错误（大家可以试着改动一下kernel/rros/init.rs中的代码，比如注释`init_core`的代码），可以用[finderr.py](https://github.com/Pettttter1/rtos_tools/blob/main/finderr.py)脚本对错误进行过滤，将脚本下载到项目工程`/data/bupt-rtos/rros`中，执行命令是`python3 finderr.py compile.txt`，最后利用`cat result`查看`result`文件中的错误信息。
![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118014613.png)

最后，如果编译成功了，我们就可以在主目录下看到最新生成的`vmlinux`文件，然后用qemu去模拟运行这个操作系统，使用qemu运行操作系统的部分，会在[使用Qemu进行模拟](#%E4%BD%BF%E7%94%A8qemu%E8%BF%9B%E8%A1%8C%E6%A8%A1%E6%8B%9F)小节进行讲解。

> 编译tips：  
> 1. 可以注意到make命令中使用了`LLVM=1`，这个参数会让编译过程中使用llvm而不是gcc，这是因为我们需要rfl项目的支持，而rfl需要通过llvm才能成功编译rust。所以我们在rros的大部分编译命令中都需要加入`LLVM=1`。  
> 2. rros目标的平台是在arm64，所以涉及到交叉编译的知识，交叉编译就是编译代码的环境和执行代码的环境不在一个平台上，比如在x86_64平台下，编译arm64的目标文件，我们在编译时通过使用了两个环境变量来说明这两个信息，rros编译时会自动读取这两个环境变量来获得这部分信息。
> ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118015945.png)
> 3. 一些docker环境中隐藏的细节：docker环境中配置了可以支持交叉编译环境的gcc，llvm，qemu，gdb，objdump等编译相关的工具，以及cmake，rust等开发相关的工具，有些软件是从源码编译安装的，因为ubuntu/centos的apt-get和yum包管理工具会限制这些软件的版本。
> 4. config如何手动配置：config中需要手动配置的主要是分为三部分：如何开启dovetail和rfl，如何开启debug相关的选项，如何裁剪和rros内核无关的config，下面介绍前两部分。
>   - 执行`make LLVM=1 menuconfig`配置命令
>   - Kernel Features中开启Dovetail interface  
>     ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118021223.png)
>     ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118021334.png)
>   - General setup中开启Rust support
>     ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118021143.png)
>     ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118021205.png)
>   - debug相关的config
>     ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118021729.png)
>     ![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118021814.png)

## 使用Qemu进行模拟同时使用gdb对进行Debug

### 编译等级

调试时可能需要将编译等级调低。我们所给的配置文件`.config`应该已经配置，若没有，你可以手动配置一下。

输入`menuconfig`

![gdb](assets/menuconfig.png)

> 如果出现问题，可能有两种情况：
>
> * 把窗口拉大一点
> * 看bash环境变量是否有\$CROSS_COMPILE，​\$ARCH。这些在之前应该已经配置过。
>
> ```
> # 在~/.bashrc里添加
> export CROSS_COMPILE=aarch64-linux-gnu-
> export ARCH=arm64
> ```

把`kernel hacking`  >  `Rust Hacking`  > `Optimization level` > `debug-level` 调到最低

![gdb](assets/debug-level.png)

按空格确定向右选择Exit，按回车退出，选择保存`Yes`.

### 使用rust-gdb调试

在docker中我们已经安装好qemu了，所以可以直接使用。并且qemu启动所需要的文件系统已经在docker中准备完成。

我们只需要同时打开两个命令行窗口，然后左边运行qemu的命令，右边运行gdb的命令，我们就可以完成对rros的debug工作。

```bash
qemu-system-aarch64 -nographic  -kernel arch/arm64/boot/Image -initrd ../arm64_ramdisk/rootfs.cpio.gz -machine type=virt -cpu cortex-a57 -append "rdinit=/linuxrc console=ttyAMA0" -device virtio-scsi-device -smp 1 -m 4096  -drive if=none,format=qcow2,file=test.qcow2 -s -S
```
最后的两个标志 -s 表示启动gdb server，-S表示不要立刻执行指令，按`c`可以开始执行。

```bash
. ./gr
```

![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118034722.png)

如果没有`gr`在路径下面，也可以手动启动一个rust-gdb进程：

```
rust-gdb \
--tui vmlinux \
-ex "target remote localhost:1234"
-ex "set architecture aarch64"
-ex "set auto-load safe-path"
-ex "set lang rust"
```



> 如果不想debug，只想用qemu对操作系统进行模拟运行，那么只需要打开一个窗口，然后去掉`-s -S`这两个gdb相关的参数，运行下列命令即可
> ```bash
> qemu-system-aarch64 -nographic  -kernel arch/arm64/boot/Image -initrd ../arm64_ramdisk/rootfs.cpio.gz -machine type=virt -cpu cortex-a57 -append "rdinit=/linuxrc console=ttyAMA0" -device virtio-scsi-device -smp 1 -m 4096  -drive if=none,format=qcow2,file=test.qcow2
> ```

先在qemu所在窗口执行上述命令，然后在gdb窗口执行上述命令，就可以成功运行
![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118034541.png)

在gdb窗口中按c并回车，可以看到rros操作系统就可以正常执行了。

![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118034922.png)

gdb的具体指令和上学期bos lab中的相关指令一致，具体内容可以回看上学期ppt。

![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118035131.png)

下面给出了gdb的一些常见命令

| 命令          | 作用                               |
| ------------- | ---------------------------------- |
| c             | 跳到下一个断点                     |
| b 文件名:行号 | 设置断点                           |
| p 变量名      | 打印变量                           |
| x/            | 打印地址下的数据                   |
| finish        | 跳到当前函数的结尾                 |
| frame         | 查看栈帧                           |
| n             | next，下一步，不进入函数           |
| s             | step in， 下一步，但是可能进入函数 |

举一个例子，如果想要在操作系统执行之前打一个断点，可以用`b kernel/rros/init.rs:135`在kernel/rros/init.rs这个文件的135行打一个断点，然后再按c就能执行到这一行了。

![](https://raw.githubusercontent.com/Richardhongyu/pic/main/20230118035858.png)

执行结束时，退出qemu时，先按ctrl+a+x；退出gdb时，按ctrl+d。

### 使用vscode的调试

也可以给vscode添加配置文件。

首先，同样还是在命令行启动调试的qemu：

```
qemu-system-aarch64 -nographic  -kernel arch/arm64/boot/Image -initrd ../arm64_ramdisk/rootfs.cpio.gz -machine type=virt -cpu cortex-a57 -append "rdinit=/linuxrc console=ttyAMA0" -device virtio-scsi-device -smp 1 -m 4096  -drive if=none,format=qcow2,file=test.qcow2 -s -S
```

然后，在项目根目录的.vscode文件夹中，打开.vscode/launch.json(没有的话新建一个)，把下面的配置粘贴进去：

```
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "name": "kernel-debug",
            "type": "cppdbg",
            "request": "launch",
            "miDebuggerServerAddress": "127.0.0.1:1234",
            "program": "${workspaceFolder}/vmlinux",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "externalConsole": false,
            "logging": {
                "engineLogging": false
            },
            "MIMode": "gdb",
            "miDebuggerPath" : "/root/.cargo/bin/rust-gdb",
            // "miDebuggerPath":"/usr/bin/gdb-multiarch",
            "setupCommands": [
                {
                    "description": "set language rust",
                    "text": "set lang rust",
                    "ignoreFailures": true
                }
            ]
        }
    ]
}
```

在行号处点击断点，按F5开始调试

![gdb3](assets\vscode-gdb.png)

如果需要使用gdb命令，可以在下面`DEBUG CONSOLE`，输入-exec {gdb命令}执行

![gdb3](assets\gdb-console.png)

## 引用

[1] A linux-based real-time operating system  
[2] Adaptive domain environment for operating systems  
[3] Life with adoes  
[4] Study and Comparison of the RTHAL-based and ADEOS-based RTAI Real-time Solutions for Linux  
[5] https://github.com/Rust-for-Linux/linux