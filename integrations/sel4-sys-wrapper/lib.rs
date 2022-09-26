#![feature(lang_items)]
#![no_std]
#![allow(non_snake_case)]

use cfg_if::cfg_if;
use core::panic::PanicInfo;

use macros::*;
use sel4_sys::*;

#[cfg(all(target_arch = "x86"))]
export_syscalls!("ia32_syscall_stub.rs");
#[cfg(all(target_arch = "x86_64"))]
export_syscalls!("x86_64_syscall_stub.rs");
#[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
export_syscalls!("aarch32_syscall_stub.rs");
#[cfg(target_arch = "riscv32")]
export_syscalls!("riscv32_syscall_stub.rs");

// Export common syscalls

#[export_syscall]
#[inline(always)]
pub unsafe fn seL4_Send(dest: seL4_CPtr, msgInfo: seL4_MessageInfo) {}

#[export_syscall]
#[inline(always)]
pub unsafe fn seL4_NBSend(dest: seL4_CPtr, msgInfo: seL4_MessageInfo) {}

#[export_syscall]
#[inline(always)]
pub unsafe fn seL4_Signal(dest: seL4_CPtr) {}

#[export_syscall]
#[inline(always)]
pub unsafe fn seL4_Call(dest: seL4_CPtr, msgInfo: seL4_MessageInfo) -> seL4_MessageInfo {}

#[export_syscall]
#[inline(always)]
pub unsafe fn seL4_Yield() {}

cfg_if! {
    if #[cfg(feature = "CONFIG_KERNEL_MCS")] {
        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_Recv(
            src: seL4_CPtr,
            sender: *mut seL4_Word,
            reply: seL4_CPtr,
        ) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_NBRecv(
            src: seL4_CPtr,
            sender: *mut seL4_Word,
            reply: seL4_CPtr,
        ) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_Wait(
            src: seL4_CPtr,
            sender: *mut seL4_Word,
        ) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_NBWait(
            src: seL4_CPtr,
            sender: *mut seL4_Word,
        ) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_NBSendRecv(
            dest: seL4_CPtr,
            msgInfo: seL4_MessageInfo,
            src: seL4_CPtr,
            sender: *mut seL4_Word,
            reply: seL4_CPtr,
        ) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_NBSendWait(
            dest: seL4_CPtr,
            msgInfo: seL4_MessageInfo,
            src: seL4_CPtr,
            sender: *mut seL4_Word,
        ) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_ReplyRecv(
            src: seL4_CPtr,
            msgInfo: seL4_MessageInfo,
            sender: *mut seL4_Word,
            reply: seL4_CPtr,
        ) -> seL4_MessageInfo {}
    } else {
        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_Reply(msgInfo: seL4_MessageInfo) {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_Recv(src: seL4_CPtr, sender: *mut seL4_Word) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_NBRecv(src: seL4_CPtr, sender: *mut seL4_Word) -> seL4_MessageInfo {}

        #[export_syscall]
        #[inline(always)]
        pub unsafe fn seL4_ReplyRecv(
            src: seL4_CPtr,
            msgInfo: seL4_MessageInfo,
            sender: *mut seL4_Word,
        ) -> seL4_MessageInfo {}
    } // !CONFIG_KERNEL_MCS
}

// TODO(sleffler): CONFIG_PRINTING
// TODO(sleffler): CONFIG_DEBUG_BUILD
// TODO(sleffler): CONFIG_ENABLE_BENCHMARKS?
// TODO(sleffler): CONFIG_SET_TLS_BASE_SELF

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
