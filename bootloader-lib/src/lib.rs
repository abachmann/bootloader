#![feature(slice_fill)]
#![no_std]

use core::panic::PanicInfo;
pub use logger::{FrameBufferInfo, PixelFormat};
use x86_64::{
    structures::paging::{FrameAllocator, MapperAllSizes, Size4KiB},
    VirtAddr,
};

mod load_kernel;
mod logger;

pub fn init_logger(framebuffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = logger::LOGGER.get_or_init(move || logger::LockedLogger::new(framebuffer, info));
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(log::LevelFilter::Trace);
}

pub fn load_kernel(
    kernel: &'static [u8],
    page_table: &mut impl MapperAllSizes,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> VirtAddr {
    load_kernel::load_kernel(kernel, page_table, frame_allocator).expect("Failed to parse kernel")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { logger::LOGGER.get().map(|l| l.force_unlock()) };
    log::error!("{}", info);
    loop {}
}