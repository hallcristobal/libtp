use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn my_panic(info: &PanicInfo) -> ! {
    report!("{}", info);
    loop {}
}
