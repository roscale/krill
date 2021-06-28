use crate::paging::PageDirectory;

extern "C" {
    pub fn reload_segments();
    pub fn enable_paging();
    pub fn set_page_directory(pd: &PageDirectory);
    pub fn jump_usermode() -> !;
    pub fn get_cr2() -> u32;
    pub fn get_esp() -> u32;
    pub fn write_syscall(_fd: u32, buf: *const u8, count: usize);
    pub fn system_call_shim();
}