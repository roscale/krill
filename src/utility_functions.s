.intel_syntax noprefix

.global reload_segments
reload_segments:
    jmp    0x08:reload_cs
reload_cs:
    mov    ax, 0x10
    mov    ds, ax
    mov    es, ax
    mov    fs, ax
    mov    gs, ax
    mov    ss, ax
    ret

.global enable_paging
enable_paging:
    mov    eax, cr0
    or     eax, 0x80000001
    mov    cr0, eax
    ret

.global set_page_directory
set_page_directory:
    mov    eax, [esp+4]
    mov    cr3, eax
    ret

.global jump_usermode
jump_usermode:
	mov ax, (4 * 8) | 3 # ring 3 data with bottom 2 bits set for ring 3
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax # SS is handled by iret

	# set up the stack frame iret expects
	mov eax, esp
	push (4 * 8) | 3 # data selector
	push eax # current esp
	pushf # eflags
	push (3 * 8) | 3 # code selector (ring 3 code with bottom 2 bits set for ring 3)
	push OFFSET hello_userspace # instruction address to return to
	iret

.global get_cr2
get_cr2:
    mov eax, cr2
    ret

.global get_esp
get_esp:
    mov eax, esp
    ret

.global hello_userspace
hello_userspace:
    sub esp, 4
a:
    mov ebx, 0x10000000
c:
    mov [esp + 4], ebx
    mov ebx, [esp + 4]
    cmp ebx, 0
    jz z
    dec ebx
    jmp c
z:
    int 0x80
    jmp a
