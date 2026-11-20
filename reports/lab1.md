# 总结

主要是在 inner 中添加了每个程序的 trace_map，然后每次 syscall 的时候会去给对应 id 加 1，在调用 trace 时直接读 map 即可。

至于 request 为其他的值按照实验要求完成。

# QA

## Q1

Q:
正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

A:
sbi 为 0.2.0-alpha.2 的 RustSBI，运行后得到结果：
```text
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
```

第一个程序尝试写入地址 0，第二个程序尝试调用 `sret` 特权指令，第三个程序尝试调用 `sstatus` csr 寄存器。这三个程序都会触发指令集规定的异常，然后 trap 进内核，经过 `trap_handler` 的分发处理。

## Q2

Q:
L40：刚进入 __restore 时，sp 代表了什么值。请指出 __restore 的两种使用情景。

A:
sp 此时指向内核栈栈顶，`__restore` 创建一个新的程序上下文（task/mod.rs）和保存上下文（batch.rs）

## Q3

Q:
L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

A:
这几步主要是让 sstatus，sepc，sscratch 恢复成用户态下应该的值。sstatus 负责管理特权等级，意味着此时 CPU 为用户特权级；sepc 负责存放异常发生地址，当 sret 时恢复回 pc；sscratch 保存内核特权级上下文的指针

## Q4

Q:
L50-L56：为何跳过了 x2 和 x4？

A:
sp 要特殊处理，tp 用不到

## Q5

Q:
L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？

A:
切换栈，sp 对应 kernel 栈，sscratch 代表用户栈

## Q6

Q:
__restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

A:
sret，指令集规定，CPU 会把相应 csr 自动根据规定填入对应寄存器中

## Q7

Q:
13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

A:
同 Q5

## Q8

Q：
从 U 态进入 S 态是哪一条指令发生的？

A：
会产生异常的指令，和程序主动触发的 `ecall`

# 荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

Riscv 手册中文翻译

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。