#![no_std]
#![no_main]

use aya_ebpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[kprobe]
pub fn ru_process_check(ctx: ProbeContext) -> u32 {
    match try_ru_process_check(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_ru_process_check(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function trace_sys_enter called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
