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

.global jump_to_higher_half
jump_to_higher_half:
    mov    eax, [esp + 4]
    add    eax, HIGHER_HALF_ADDRESS
    add    esp, HIGHER_HALF_ADDRESS
    call   eax
