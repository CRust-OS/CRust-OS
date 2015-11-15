.text
.global _start

_start:
    //cld //TODO explain
    mov %rbp, STACK_BOTTOM
    mov %rsp, %rbp
    call main
    hlt    
