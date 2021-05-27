	.file	"example.cpp"
	.text
	.globl	__gxx_personality_v0
	.bss
	.align 8
	.type	__gxx_personality_v0, @object
	.size	__gxx_personality_v0, 8
__gxx_personality_v0:
	.zero	8
	.section	.rodata
.LC0:
	.string	"constructing %x %x\n"
	.text
	.align 2
	.globl	_ZN7SelfRefC2Ev
	.type	_ZN7SelfRefC2Ev, @function
_ZN7SelfRefC2Ev:
.LFB1:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	-8(%rbp), %rax
	movq	-8(%rbp), %rdx
	movq	%rdx, (%rax)
	movq	-8(%rbp), %rax
	movl	$123456, 8(%rax)
	movq	-8(%rbp), %rax
	movq	(%rax), %rdx
	movq	-8(%rbp), %rax
	movq	%rax, %rsi
	leaq	.LC0(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.size	_ZN7SelfRefC2Ev, .-_ZN7SelfRefC2Ev
	.globl	_ZN7SelfRefC1Ev
	.set	_ZN7SelfRefC1Ev,_ZN7SelfRefC2Ev
	.section	.rodata
.LC1:
	.string	"constructing %x\n"
	.text
	.align 2
	.globl	_ZN7SelfRefC2Ei
	.type	_ZN7SelfRefC2Ei, @function
_ZN7SelfRefC2Ei:
.LFB4:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movl	%esi, -12(%rbp)
	movq	-8(%rbp), %rax
	movq	-8(%rbp), %rdx
	movq	%rdx, (%rax)
	movq	-8(%rbp), %rax
	movl	-12(%rbp), %edx
	movl	%edx, 8(%rax)
	movq	-8(%rbp), %rax
	movq	%rax, %rsi
	leaq	.LC1(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE4:
	.size	_ZN7SelfRefC2Ei, .-_ZN7SelfRefC2Ei
	.globl	_ZN7SelfRefC1Ei
	.set	_ZN7SelfRefC1Ei,_ZN7SelfRefC2Ei
	.section	.rodata
.LC2:
	.string	"destructing %x %x\n"
	.text
	.align 2
	.globl	_ZN7SelfRefD2Ev
	.type	_ZN7SelfRefD2Ev, @function
_ZN7SelfRefD2Ev:
.LFB7:
	.cfi_startproc
	.cfi_personality 0x9b,DW.ref.__gxx_personality_v0
	.cfi_lsda 0x1b,.LLSDA7
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	-8(%rbp), %rax
	movq	(%rax), %rdx
	movq	-8(%rbp), %rax
	movq	%rax, %rsi
	leaq	.LC2(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE7:
	.globl	__gxx_personality_v0
	.section	.gcc_except_table,"a",@progbits
.LLSDA7:
	.byte	0xff
	.byte	0xff
	.byte	0x1
	.uleb128 .LLSDACSE7-.LLSDACSB7
.LLSDACSB7:
.LLSDACSE7:
	.text
	.size	_ZN7SelfRefD2Ev, .-_ZN7SelfRefD2Ev
	.globl	_ZN7SelfRefD1Ev
	.set	_ZN7SelfRefD1Ev,_ZN7SelfRefD2Ev
	.section	.rodata
.LC3:
	.string	"copy construct"
	.text
	.align 2
	.globl	_ZN7SelfRefC2ERKS_
	.type	_ZN7SelfRefC2ERKS_, @function
_ZN7SelfRefC2ERKS_:
.LFB10:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	%rsi, -16(%rbp)
	leaq	.LC3(%rip), %rdi
	call	puts@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE10:
	.size	_ZN7SelfRefC2ERKS_, .-_ZN7SelfRefC2ERKS_
	.globl	_ZN7SelfRefC1ERKS_
	.set	_ZN7SelfRefC1ERKS_,_ZN7SelfRefC2ERKS_
	.section	.rodata
.LC4:
	.string	"move construct"
	.text
	.align 2
	.globl	_ZN7SelfRefC2EOS_
	.type	_ZN7SelfRefC2EOS_, @function
_ZN7SelfRefC2EOS_:
.LFB13:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	%rsi, -16(%rbp)
	leaq	.LC4(%rip), %rdi
	call	puts@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE13:
	.size	_ZN7SelfRefC2EOS_, .-_ZN7SelfRefC2EOS_
	.globl	_ZN7SelfRefC1EOS_
	.set	_ZN7SelfRefC1EOS_,_ZN7SelfRefC2EOS_
	.section	.rodata
.LC5:
	.string	"copy assign"
	.text
	.align 2
	.globl	_ZN7SelfRefaSERKS_
	.type	_ZN7SelfRefaSERKS_, @function
_ZN7SelfRefaSERKS_:
.LFB15:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	%rsi, -16(%rbp)
	leaq	.LC5(%rip), %rdi
	call	puts@PLT
	movq	-8(%rbp), %rax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE15:
	.size	_ZN7SelfRefaSERKS_, .-_ZN7SelfRefaSERKS_
	.section	.rodata
.LC6:
	.string	"move assign"
	.text
	.align 2
	.globl	_ZN7SelfRefaSEOS_
	.type	_ZN7SelfRefaSEOS_, @function
_ZN7SelfRefaSEOS_:
.LFB16:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	%rsi, -16(%rbp)
	leaq	.LC6(%rip), %rdi
	call	puts@PLT
	movq	-8(%rbp), %rax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE16:
	.size	_ZN7SelfRefaSEOS_, .-_ZN7SelfRefaSEOS_
	.section	.rodata
.LC7:
	.string	"checking %x %x %i\n"
	.text
	.align 2
	.globl	_ZN7SelfRef5checkEv
	.type	_ZN7SelfRef5checkEv, @function
_ZN7SelfRef5checkEv:
.LFB17:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	movq	-8(%rbp), %rax
	movl	8(%rax), %ecx
	movq	-8(%rbp), %rax
	movq	(%rax), %rdx
	movq	-8(%rbp), %rax
	movq	%rax, %rsi
	leaq	.LC7(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE17:
	.size	_ZN7SelfRef5checkEv, .-_ZN7SelfRef5checkEv
	.globl	ptr1
	.bss
	.align 8
	.type	ptr1, @object
	.size	ptr1, 8
ptr1:
	.zero	8
	.globl	ptr2
	.align 8
	.type	ptr2, @object
	.size	ptr2, 8
ptr2:
	.zero	8
	.text
	.globl	_Z6myfuncv
	.type	_Z6myfuncv, @function
_Z6myfuncv:
.LFB33:
	.cfi_startproc
	.cfi_personality 0x9b,DW.ref.__gxx_personality_v0
	.cfi_lsda 0x1b,.LLSDA33
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r12
	pushq	%rbx
	.cfi_offset 12, -24
	.cfi_offset 3, -32
#APP
# 94 "example.cpp" 1
	marker0
# 0 "" 2
#NO_APP
	movl	$15, %edi
	call	malloc@PLT
#APP
# 96 "example.cpp" 1
	marker1
# 0 "" 2
#NO_APP
	movl	$16, %edi
.LEHB0:
	call	_Znwm@PLT
.LEHE0:
	movq	%rax, %rbx
	movq	%rbx, %rdi
.LEHB1:
	call	_ZN7SelfRefC1Ev
.LEHE1:
	movq	%rbx, ptr1(%rip)
#APP
# 98 "example.cpp" 1
	marker3
# 0 "" 2
#NO_APP
	movl	$16, %edi
.LEHB2:
	call	_Znwm@PLT
.LEHE2:
	movq	%rax, %rbx
	movl	$13, %esi
	movq	%rbx, %rdi
.LEHB3:
	call	_ZN7SelfRefC1Ei
.LEHE3:
	movq	%rbx, ptr2(%rip)
#APP
# 100 "example.cpp" 1
	marker2
# 0 "" 2
#NO_APP
	jmp	.L16
.L14:
	movq	%rax, %r12
	movl	$16, %esi
	movq	%rbx, %rdi
	call	_ZdlPvm@PLT
	movq	%r12, %rax
	movq	%rax, %rdi
.LEHB4:
	call	_Unwind_Resume@PLT
.L15:
	movq	%rax, %r12
	movl	$16, %esi
	movq	%rbx, %rdi
	call	_ZdlPvm@PLT
	movq	%r12, %rax
	movq	%rax, %rdi
	call	_Unwind_Resume@PLT
.LEHE4:
.L16:
	popq	%rbx
	popq	%r12
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE33:
	.section	.gcc_except_table
.LLSDA33:
	.byte	0xff
	.byte	0xff
	.byte	0x1
	.uleb128 .LLSDACSE33-.LLSDACSB33
.LLSDACSB33:
	.uleb128 .LEHB0-.LFB33
	.uleb128 .LEHE0-.LEHB0
	.uleb128 0
	.uleb128 0
	.uleb128 .LEHB1-.LFB33
	.uleb128 .LEHE1-.LEHB1
	.uleb128 .L14-.LFB33
	.uleb128 0
	.uleb128 .LEHB2-.LFB33
	.uleb128 .LEHE2-.LEHB2
	.uleb128 0
	.uleb128 0
	.uleb128 .LEHB3-.LFB33
	.uleb128 .LEHE3-.LEHB3
	.uleb128 .L15-.LFB33
	.uleb128 0
	.uleb128 .LEHB4-.LFB33
	.uleb128 .LEHE4-.LEHB4
	.uleb128 0
	.uleb128 0
.LLSDACSE33:
	.text
	.size	_Z6myfuncv, .-_Z6myfuncv
	.hidden	DW.ref.__gxx_personality_v0
	.weak	DW.ref.__gxx_personality_v0
	.section	.data.rel.local.DW.ref.__gxx_personality_v0,"awG",@progbits,DW.ref.__gxx_personality_v0,comdat
	.align 8
	.type	DW.ref.__gxx_personality_v0, @object
	.size	DW.ref.__gxx_personality_v0, 8
DW.ref.__gxx_personality_v0:
	.quad	__gxx_personality_v0
	.ident	"GCC: (GNU) 10.2.0"
	.section	.note.GNU-stack,"",@progbits
