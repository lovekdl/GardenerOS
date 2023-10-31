# 操作系统实验1

> 邓人嘉 21301032

### 一、实验步骤

#### 1.1 创建一个Rust项目

* 进入mnt目录，执行"cargo new os --bin"
  ![image-20231101002938929](pictures/image-20231101002938929.png)

* cd os
  cargo build
  cargo run

  ![image-20231101003013329](pictures/image-20231101003013329.png)

#### 1.2 移除标准库依赖

* 修改target为riscv64
  ![image-20231101003237331](pictures/image-20231101003237331.png)
  ![image-20231101003330975](pictures/image-20231101003330975.png)
* ![image-20231101003406487](pictures/image-20231101003406487.png)
  ![image-20231101003459667](pictures/image-20231101003459667.png)
* 安装相关软件包
  ![image-20231101003811538](pictures/image-20231101003811538.png)
* ![image-20231101003849926](pictures/image-20231101003849926.png)
* git提交
  ![image-20231101004200019](pictures/image-20231101004200019.png)![image-20231101004118723](pictures/image-20231101004118723.png)

* 执行命令分析移除标准库后的独立可执行程序。![image-20231101004802834](pictures/image-20231101004802834.png)

#### 1.3 用户态可执行的环境

* 增加入口函数
  ![image-20231101005554758](pictures/image-20231101005554758.png)

* 编译运行
  ![image-20231101005924918](pictures/image-20231101005924918.png)

* 去掉loop尝试
  ![image-20231101010012056](pictures/image-20231101010012056.png)![image-20231101010051479](pictures/image-20231101010051479.png)

* 实现退出机制
  ![image-20231101011934195](pictures/image-20231101011934195.png)
  ![image-20231101012023907](pictures/image-20231101012023907.png)

* 实现输出支持

  ![image-20231101012728774](pictures/image-20231101012728774.png)

  

