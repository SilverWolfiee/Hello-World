#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
type Errno = i64;
#[no_mangle]
pub extern "C" fn _start()-> !{
    let result = write(1, b"Hello, World!!\n");
    let exit_code = match result {
        Ok(_) => 0,
        Err(_) => 1,
    };
    exit(exit_code)
}
#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    loop{}
}

fn write(fd: u32, buf: &[u8]) -> Result<usize, Errno> {
    let mut ret: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1,                
            in("rdi") fd,               
            in("rsi") buf.as_ptr(),    
            in("rdx") buf.len(),        
            lateout("rax") ret,         
            out("rcx") _,
            out("r11") _,
        );
    }

    if ret < 0 {
        Err(ret)
    } else {
        Ok(ret as usize)
    }
}

fn exit(code: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,               
            in("rdi") code,
            options(noreturn)
        );
    }
}