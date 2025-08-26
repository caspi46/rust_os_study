use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{OffsetPageTable, PageTable},
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}

// Returns a mutable reference to the active level 4 table
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

// translate address
//
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3; // CR3 register 
    // Frame error: Frame Not Present, Huge Frame
    use x86_64::structures::paging::page_table::FrameError;

    // read the active level 4 frame from the CR3 register
    // Top-level (Level 4) physical frame
    let (level_4_table_frame, _) = Cr3::read();

    // gets the indices of the page table from the virtual address: addr, one of the argument
    // from level 4 to level 1
    let table_indexes = [
        addr.p4_index(), // Level 4
        addr.p3_index(), // Level 3
        addr.p2_index(), // Level 2
        addr.p1_index(), // Level 1
    ];
    // set initial frame (the top-level table frame)
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr = virt.as_ptr() as *const PageTable;
        let table = unsafe { &*table_ptr };

        // read the page table entry and update the frame
        let entry = &table[index];
        // get the next frame by entry
        // (L4 ->) L3 -> L2 -> L1 -> Physical frame
        frame = match entry.frame() {
            Ok(frame) => frame,
            // if no next frame
            Err(FrameError::FrameNotPresent) => return None,
            // error b/c huge size of frame, such as 2 MB or 1 GB pages
            // this occurs when the current frame is directly mapping to the final result
            // but this is not expected situation: haven't handle this situation
            Err(FrameError::HugeFrame) => panic!("huge page not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    // page offset: The position (in bytes) within a 4 KB page for a specific virtual address.
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
