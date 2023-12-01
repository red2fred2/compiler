.globl main
.globl FGETS_BUFFER

.bss
.align 32
.size FGETS_BUFFER, 1024
FGETS_BUFFER: .zero 1024

.data
global_asdf: .zero 8
int_fmt: .string "%d\n"
hw_str: .string "Hello bitches!"

.text
// Begin function
main: push %rbp
movq %rsp, %rbp

// String print
movq $hw_str, %rdi
call puts

// Int print
movq $int_fmt, %rdi
movq $4, %rsi
call printf

// User input
leaq FGETS_BUFFER(%rip), %rdi
movq $1024, %rsi
movq stdin(%rip), %rdx
call fgets
movq %rax, %rdi
call atoi

// Print that back out
movq $int_fmt, %rdi
movq %rax, %rsi
call printf

// End function
addq $4, %rsp
xor %rax, %rax
leave
ret

