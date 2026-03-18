use crate::println;
use fdt::Fdt;

const PAGE_SIZE: usize = 1 << 12;
static mut BITMAP: Option<MemoryBitmap> = None;

#[derive(Clone, Copy)]
struct MemoryBitmap {
    addr: *mut u8,
    mem_base: *mut u8,
    mem_pages: usize,
}

fn pages_needed_for_bytes(count: usize) -> usize {
    count / PAGE_SIZE + (if count % PAGE_SIZE != 0 { 1 } else { 0 })
}

unsafe extern "C" {
    static _start: u8;
    static _end: u8;
}

pub fn init(fdt: &Fdt) {
    let kernel_base = unsafe { &_start as *const _ as usize };
    let kernel_length = unsafe { (&_end as *const _ as usize) - (kernel_base) };
    let kernel_length_pages = pages_needed_for_bytes(kernel_length);
    println!(
        "mm: kernel is {kernel_length} bytes ({kernel_length_pages} pages) big at 0x{kernel_base:x}"
    );

    let mem = fdt.memory();
    // todo: handle more than 1 region
    let region = mem.regions().nth(0).unwrap();
    let mem_base = region.starting_address as usize;
    let mem_size = region.size.unwrap_or(0);
    assert!(mem_size % PAGE_SIZE == 0);
    let mem_pages = mem_size / PAGE_SIZE;
    println!(
        "mm: physical mem starts at 0x{mem_base:x} and is {mem_size} bytes ({mem_pages} pages) big"
    );
    let bitmap_size = mem_pages / 8;
    let bitmap_pages = pages_needed_for_bytes(bitmap_size);
    println!("mm: need a bitmap {bitmap_size} bytes ({bitmap_pages} pages) big");

    // todo: handle memory reservations
    // for now we can assume it's safe to place after the kernel
    unsafe {
        let bitmap_addr = ((kernel_length_pages) * PAGE_SIZE + mem_base) as *mut u8;
        println!("mm: putting bitmap at 0x{:x}", bitmap_addr as usize);
        bitmap_addr.write_bytes(0, bitmap_size);

        BITMAP = Some(MemoryBitmap {
            addr: bitmap_addr,
            mem_base: mem_base as *mut u8,
            mem_pages,
        });

        mark_pages(
            bitmap_pages,
            (bitmap_addr as usize - mem_base) / PAGE_SIZE,
            true,
        );

        mark_pages(
            kernel_length_pages,
            (kernel_base - mem_base) / PAGE_SIZE,
            true,
        );
    }
}

// not the most optimized code in the world, that's for sure
/// SAFETY: this function is not thread-safe.
unsafe fn mark_pages(count: usize, offset: usize, set: bool) {
    let bitmap_addr = unsafe {
        BITMAP
            .expect("attempt to allocate memory before mm init")
            .addr
    };

    for i in offset..(offset + count) {
        let byte_off = (i / 8) as isize;
        let bit_off = i % 8;
        let bit_mask = (1 << bit_off) as u8;

        // set/clear bit inside the byte
        unsafe {
            let mut byte = bitmap_addr.offset(byte_off).read();
            if set {
                // set the bit
                byte |= bit_mask;
            } else {
                // clear the bit
                byte &= !bit_mask;
            }
            bitmap_addr.offset(byte_off).write(byte);
        }
    }
}

/// Finds free contiguous pages in physical memory.
///
/// Returns the first page, or [`None`].
///
/// SAFETY: this function is not thread-safe.
unsafe fn find_free_pages(count: usize) -> Option<usize> {
    unsafe {
        let bitmap_addr = BITMAP
            .expect("attempt to allocate memory before mm init")
            .addr;

        for i in 0..(BITMAP.unwrap().mem_pages) {
            let byte_off = (i / 8) as isize;
            let bit_off = i % 8;
            let bit_mask = (1 << bit_off) as u8;

            // check if page i is set
            if bitmap_addr.offset(byte_off).read() & bit_mask != 0 {
                continue;
            }

            // check if pages i+1...i+count are set
            for j in (i + 1)..(count + i) {
                let byte_off = (j / 8) as isize;
                let bit_off = j % 8;
                let bit_mask = (1 << bit_off) as u8;

                if bitmap_addr.offset(byte_off).read() & bit_mask != 0 {
                    continue;
                }
            }

            // if we got here, we likely found contiguous space for this alloc
            return Some(i);
        }
    }

    // if we went through the entire memory space and found zero holes for our allocation,
    // we don't have enough memory for it, or our memory is too fragmented
    None
}

// todo: track allocations
pub fn page_alloc(page_count: usize) -> Option<*mut u8> {
    unsafe {
        find_free_pages(page_count).map(|offset| {
            // mark as used and return pages
            mark_pages(page_count, offset, true);
            BITMAP.unwrap().mem_base.add(offset * PAGE_SIZE)
        })
    }
}

pub fn page_free(ptr: *mut u8, page_count: usize) {
    unsafe {
        mark_pages(page_count, (ptr as usize) / PAGE_SIZE, false);
    }
}
