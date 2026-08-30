.global _start
.global header_start
.extern rust_main

.section .multiboot_header, "a"
.align 8
header_start:
    .long 0xe85250d6
    .long 0
    .long header_end - header_start
    .long -(0xe85250d6 + 0 + (header_end - header_start))

    .word 0
    .word 0
    .long 8
header_end:

.section .bss, "aw", @nobits
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 16384
stack_top:

.section .rodata
.align 8
gdt64:
    .quad 0
.set code_segment, . - gdt64
    .quad (1<<43) | (1<<44) | (1<<47) | (1<<53)
gdt64_pointer:
    .word . - gdt64 - 1
    .quad gdt64

.section .text
.code32
_start:
    movl $stack_top, %esp

    /* map first P4 entry to P3 table */
    movl $p3_table, %eax
    orl $3, %eax
    movl %eax, p4_table

    /* map first P3 entry to P2 table */
    movl $p2_table, %eax
    orl $3, %eax
    movl %eax, p3_table

    /* map P2 entries to huge 2MiB pages */
    movl $0, %ecx
map_p2_table:
    movl $0x200000, %eax
    mull %ecx
    orl $131, %eax
    movl %eax, p2_table(,%ecx,8)
    movl %edx, p2_table+4(,%ecx,8)
    incl %ecx
    cmpl $512, %ecx
    jne map_p2_table

    /* load P4 to cr3 */
    movl $p4_table, %eax
    movl %eax, %cr3

    /* enable PAE in cr4 */
    movl %cr4, %eax
    orl $32, %eax
    movl %eax, %cr4

    /* set long mode bit in EFER MSR */
    movl $0xC0000080, %ecx
    rdmsr
    orl $256, %eax
    wrmsr

    /* enable paging in cr0 */
    movl %cr0, %eax
    orl $0x80000000, %eax
    movl %eax, %cr0

    /* load 64-bit GDT */
    lgdt gdt64_pointer

    /* far jump into 64-bit mode using retf */
    pushl $8
    pushl $long_mode_start
    lret

.code64
long_mode_start:
    /* clear data segment registers */
    movw $0, %ax
    movw %ax, %ss
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs

    /* Call the 64-bit Rust kernel entry point */
    call rust_main
    
os_returned:
    hlt
    jmp os_returned
