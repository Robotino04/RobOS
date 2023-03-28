mod area_memory_allocator;
pub use self::area_memory_allocator::AreaFrameAllocator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame{
    id: usize
}

const PAGE_SIZE: usize = 4096;

impl Frame{
    fn containing_address(address: usize) -> Frame{
        Frame {id: address / PAGE_SIZE}
    }
}

pub trait FrameAllocator{
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}