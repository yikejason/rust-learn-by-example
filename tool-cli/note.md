# Mac zsh/bash 中

- cargo build && target/debug/tool-cli post httpbin.org/post a=1 b=2

# windows 终端中

- cargo build | target/debug/tool-cli post httpbin.org/post a=1 b=2


### 打包运行 httpie 命令
- 在项目下复制 cargo build --release 生成的二进制文件 到 $PATH (环境变量中) 下的某个目录

1. Mac 操作命令：
sudo cp target/release/httpie /usr/local/bin

2. 全局使用
复制后 httpie 即可全局使用   例如  httpie post https://httpbin.org/post a=1 b=2

所以这样就编写了自己的一个命令行工具啦  哈哈

