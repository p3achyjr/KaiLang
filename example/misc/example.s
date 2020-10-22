	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 13
	.globl	_main                   ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, ___gxx_personality_v0
	.cfi_lsda 16, Lexception0
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movl	$0, -4(%rbp)
	movl	$8, %eax
	movl	%eax, %edi
	callq	__Znwm
	movq	%rax, %rdi
	movq	%rax, %rcx
Ltmp0:
	movl	$5, %esi
	movl	$6, %edx
	movq	%rdi, -40(%rbp)         ## 8-byte Spill
	movq	%rax, %rdi
	movq	%rcx, -48(%rbp)         ## 8-byte Spill
	callq	__ZN1AC1Eii
Ltmp1:
	jmp	LBB0_1
LBB0_1:
	movq	-48(%rbp), %rax         ## 8-byte Reload
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rcx
	movl	(%rcx), %edx
	movq	-16(%rbp), %rcx
	addl	4(%rcx), %edx
	movl	%edx, %eax
	addq	$48, %rsp
	popq	%rbp
	retq
LBB0_2:
Ltmp2:
	movl	%edx, %ecx
	movq	%rax, -24(%rbp)
	movl	%ecx, -28(%rbp)
	movq	-40(%rbp), %rdi         ## 8-byte Reload
	callq	__ZdlPv
## %bb.3:
	movq	-24(%rbp), %rdi
	callq	__Unwind_Resume
	ud2
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table0:
Lexception0:
	.byte	255                     ## @LPStart Encoding = omit
	.byte	155                     ## @TType Encoding = indirect pcrel sdata4
	.byte	41                      ## @TType base offset
	.byte	3                       ## Call site Encoding = udata4
	.byte	39                      ## Call site table length
Lset0 = Lfunc_begin0-Lfunc_begin0       ## >> Call Site 1 <<
	.long	Lset0
Lset1 = Ltmp0-Lfunc_begin0              ##   Call between Lfunc_begin0 and Ltmp0
	.long	Lset1
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
Lset2 = Ltmp0-Lfunc_begin0              ## >> Call Site 2 <<
	.long	Lset2
Lset3 = Ltmp1-Ltmp0                     ##   Call between Ltmp0 and Ltmp1
	.long	Lset3
Lset4 = Ltmp2-Lfunc_begin0              ##     jumps to Ltmp2
	.long	Lset4
	.byte	0                       ##   On action: cleanup
Lset5 = Ltmp1-Lfunc_begin0              ## >> Call Site 3 <<
	.long	Lset5
Lset6 = Lfunc_end0-Ltmp1                ##   Call between Ltmp1 and Lfunc_end0
	.long	Lset6
	.long	0                       ##     has no landing pad
	.byte	0                       ##   On action: cleanup
	.p2align	2
                                        ## -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZN1AC1Eii             ## -- Begin function _ZN1AC1Eii
	.weak_def_can_be_hidden	__ZN1AC1Eii
	.p2align	4, 0x90
__ZN1AC1Eii:                            ## @_ZN1AC1Eii
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movl	%esi, -12(%rbp)
	movl	%edx, -16(%rbp)
	movq	-8(%rbp), %rdi
	movl	-12(%rbp), %esi
	movl	-16(%rbp), %edx
	callq	__ZN1AC2Eii
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	__ZN1AC2Eii             ## -- Begin function _ZN1AC2Eii
	.weak_def_can_be_hidden	__ZN1AC2Eii
	.p2align	4, 0x90
__ZN1AC2Eii:                            ## @_ZN1AC2Eii
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	%rdi, -8(%rbp)
	movl	%esi, -12(%rbp)
	movl	%edx, -16(%rbp)
	movq	-8(%rbp), %rdi
	movl	-12(%rbp), %edx
	movl	%edx, (%rdi)
	movl	-16(%rbp), %edx
	movl	%edx, 4(%rdi)
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function

.subsections_via_symbols
