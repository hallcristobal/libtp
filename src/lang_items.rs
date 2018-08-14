use arrayvec;
use core::panic::PanicInfo;
use gcn;

#[panic_implementation]
#[no_mangle]
pub fn my_panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    let mut message = arrayvec::ArrayString::<[u8; 1024]>::new();
    write!(message, "{}\0", info).ok();
    unsafe {
        gcn::os::report(message.as_ptr());
    }
    loop {}
}
