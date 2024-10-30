## base64 in rust

A repo for learning how base64 encoding works and prcaticing rust program skills.

### Base64 的原理以及我们为什么使用它。
我们日常使用的字符码一般为`ASCII`，但是在网络传输时，一般会将其在发送方用`UTF-8`编码为二进制格式，再在接收端使用`UTF-8`进行解码。

但是如果传输过程中二进制位出现了丢失，则在解码时就会出现解码失败的问题。而且基础`ASCII`码只有128个字符，假如要传输汉字就会很麻烦。`Base64`只使用最基础的64个`ASCII`码来对任何二进制数据进行编码。

算法的核心逻辑是：原始字符串每个字符占一个字节（即8bit），每三个字节为一组（即24bit）。将每一组划分为4块，每块为6bit。如果一组不足三个字节，则在末尾添加一组或两组`00`（一个字节时(8+2*2)/6=2，两个字节时(16+2)/6=3），不存在数据的块用`=`填充。然后每6bit的二进制数映射到base64码表中。

本项目运行方法：
运行`cargo run` + `encode/decode` + `目标字符串`即可编码/解码

```shell
cargo run encode
> I love base64!
> SSBsb3ZlIGJhc2U2NCE=
cargo run decode
> SSBsb3ZlIGJhc2U2NCE=
> I love base64!
```