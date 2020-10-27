	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 13
	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	movl	$10, -4(%rsp)
	movl	$11, -8(%rsp)
	movl	$11, %eax
	retq
	.cfi_endproc

.subsections_via_symbols
