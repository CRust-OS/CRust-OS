
.global start

STACK_SIZE = 0x1000

.text
start:
	movq stack + STACK_SIZE, %rsp
	call init

.bss
stack:
	org $STACK_SIZE
