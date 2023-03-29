use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryAreaIter, MemoryArea};

pub struct AreaFrameAllocator<'a>{
    next_free_frame: Frame,
    current_area: Option<&'a MemoryArea>,
    areas: MemoryAreaIter<'a>,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl FrameAllocator for AreaFrameAllocator<'_> {
    fn allocate_frame(&mut self) -> Option<Frame>{
        while let Some(area) = self.current_area{
            let frame: Frame = Frame{ id: self.next_free_frame.id};

            let last_frame_current_area = Frame::containing_address((area.end_address() - 1) as usize);
            
            if frame > last_frame_current_area{
                self.choose_next_area();
            }
            else if frame >= self.kernel_start && frame <= self.kernel_end{
                self.next_free_frame = Frame { id: self.kernel_end.id + 1};
            }
            else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                self.next_free_frame = Frame { id: self.multiboot_end.id + 1};
            }
            else{
                self.next_free_frame.id += 1;
                return Some(frame);
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame){
        unimplemented!();
    }
}

impl AreaFrameAllocator<'_>{
    fn choose_next_area(&mut self){
        self.current_area = self.areas.clone().filter(|area| {
            Frame::containing_address((area.end_address() - 1) as usize) >= self.next_free_frame
        }).min_by_key(|area| area.start_address());
        
        if let Some(area) = self.current_area{
            let start_frame = Frame::containing_address(area.start_address() as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }

    pub fn new(memory_areas: MemoryAreaIter,
        kernel_start: usize, kernel_end: usize,
        multiboot_start: usize, multiboot_end: usize
    ) -> AreaFrameAllocator{
        let mut allocator = AreaFrameAllocator{
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end)
        };
        allocator.choose_next_area();
        return allocator;
    }
}