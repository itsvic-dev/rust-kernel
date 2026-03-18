use core::alloc::GlobalAlloc;

struct PageAllocator;

unsafe impl GlobalAlloc for PageAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.align() > 4096 {
            panic!("requested alloc with an alignment of >4K, not doing that");
        }

        let page_count = super::bitmap::pages_needed_for_bytes(layout.size());
        super::bitmap::page_alloc(page_count).expect("ran out of memory? allocation returned None")
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let page_count = super::bitmap::pages_needed_for_bytes(layout.size());
        super::bitmap::page_free(ptr, page_count);
    }
}

#[global_allocator]
static PAGE_ALLOCATOR: PageAllocator = PageAllocator;
