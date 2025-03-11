#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};
use embedded_alloc::LlffHeap as Heap;
use qemu_uart::{UART, csprintln, unsafeprintln};
use riscv_rt::entry;

static INIT_PROGRAM_FLAG: AtomicBool = AtomicBool::new(false);

fn set_flag() {
    INIT_PROGRAM_FLAG.store(true, Ordering::SeqCst);
}

fn check_flag() -> bool {
    INIT_PROGRAM_FLAG.load(Ordering::SeqCst)
}

#[unsafe(export_name = "_mp_hook")]
#[rustfmt::skip]
pub extern "Rust" fn user_mp_hook(hartid: usize) -> bool {
    if hartid == 0 {
        true
    } else {
        loop {
            if check_flag() {
                break;
            }
        }
        false
    }
}

use core::mem::MaybeUninit;
use multi_hart_critical_section as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

unsafe extern "C" {
    static _heap_size: u8;
}

#[entry]
fn main(hartid: usize) -> ! {
    if hartid == 0 {
        let heap_size = unsafe { &_heap_size as *const u8 as usize };
        let heap_bottom = riscv_rt::heap_start() as usize;
        unsafe {
            HEAP.init(heap_bottom, heap_size);
        }
        critical_section::with(|cs| {
            csprintln!(
                cs,
                "Mem arr {:p}, alligned: {}",
                riscv_rt::heap_start(),
                (riscv_rt::heap_start() as usize % 32) == 0
            )
        });
        set_flag();
    }

    {
        let used = HEAP.used();

        let free = HEAP.free();
        critical_section::with(|cs| {
            csprintln!(cs, "Hart {:?} used", used);
            csprintln!(cs, "Hart {:?} free", free);
        });
    }

    let mut xs: Option<Vec<usize>> = None;
    let mut ys: Option<Vec<usize>> = None;
    if hartid == 1 {
        xs = Some(Vec::<usize>::with_capacity(2));
    }
    {
        let used = HEAP.used();

        let free = HEAP.free();
        critical_section::with(|cs| {
            csprintln!(cs, "Hart {} {:?} used", hartid, used);
            csprintln!(cs, "Hart {} {:?} free", hartid, free);
            csprintln!(cs, "Hart {:?} {:?} xs", hartid, xs);
        });
    }

    if hartid == 1 {
        if let Some(ref mut v) = xs {
            v.push(1);
        }
    }
    {
        let used = HEAP.used();

        let free = HEAP.free();
        critical_section::with(|cs| {
            csprintln!(cs, "Hart {} {:?} used", hartid, used);
            csprintln!(cs, "Hart {} {:?} free", hartid, free);
            csprintln!(cs, "Hart {:?} {:?} ys", hartid, ys);
        });
    }

    // if hartid == 1 {
    //     drop(xs);
    // }

    {
        let used = HEAP.used();

        let free = HEAP.free();
        critical_section::with(|cs| {
            csprintln!(cs, "Hart {} {:?} used", hartid, used);
            csprintln!(cs, "Hart {} {:?} free", hartid, free);
            csprintln!(cs, "Hart {:?} {:?} ys", hartid, ys);
        });
    }
    riscv::asm::fence();
    riscv::asm::fence_i();
    if hartid == 1 {
        // CRASHES
        for _i in 0..3 {
            let used = HEAP.used();

            let free = HEAP.free();
            critical_section::with(|cs| {
                csprintln!(cs, "{} WAITING, free: {}", hartid, free);
            });
        }
        ys = Some(Vec::<usize>::with_capacity(2));
    }
    {
        let used = HEAP.used();

        let free = HEAP.free();
        critical_section::with(|cs| {
            csprintln!(cs, "Hart {} {:?} used", hartid, used);
            csprintln!(cs, "Hart {} {:?} free", hartid, free);
            csprintln!(cs, "Hart {:?} {:?} ys", hartid, ys);
            csprintln!(cs, "Hart {:?} DONE", hartid);
        });
    }
    loop {}
}

#[unsafe(export_name = "ExceptionHandler")]
fn exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    unsafeprintln!(
        "\n\n\nHart {} had an exception! with trapframe: {:?}, with mcause: {:?}, with mepc: {:?}",
        riscv::register::mhartid::read(),
        trap_frame,
        riscv::register::mcause::read(),
        riscv::register::mepc::read(),
    );
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    unsafeprintln!("{}", info);
    loop {}
}
