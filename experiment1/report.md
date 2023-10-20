# 操作系统实验1

> 邓人嘉 21301032

### 一、实验步骤

#### 1.1 创建一个Rust项目

* 进入mnt目录，执行"cargo new os --bin"
  ![image-20231020115306068](pictures/image-20231020115306068.png)

* cd os
  cargo build
  cargo run

  ![image-20231020115419302](pictures/image-20231020115419302.png)

#### 1.2 移除标准库依赖

* 修改target为riscv64
  ![image-20231020115851569](pictures/image-20231020115851569.png)
  ![image-20231020115931575](pictures/image-20231020115931575.png)

* 修改main.rs文件![image-20231020115957290](pictures/image-20231020115957290.png)
  ![image-20231020120342297](pictures/image-20231020120342297.png)
* 安装相关软件包
  ![image-20231020120811059](pictures/image-20231020120811059.png)
* cargo build
  ![image-20231020121555922](pictures/image-20231020121555922.png)