use crate::PageId;
pub struct PageInfo{
    page_id : PageId,
    pin_count : u32,
    dirty_bit : u32,
    time : u32
}

impl PageInfo{
    pub fn new( page_id:PageId, pin_count : u32, dirty_bit : u32, time : u32)->Self {
        Self { 
            page_id, 
            pin_count, 
            dirty_bit, 
            time 
        }
    }

    pub fn get_file_id(&self)->u32{
        self.page_id.get_FileIdx()
    }
    pub fn get_page_id(&self)->u32{
        self.page_id.get_PageIdx()
    }
    pub fn get_pin_count(&self)->u32{
        self.pin_count
    }
    pub fn get_dirty(&self)->u32{
        self.dirty_bit
    }
    pub fn get_time(&self)->u32{
        self.time
    }

    pub fn set_pin_count(&mut self, pin_count: u32) { 
        self.pin_count = pin_count; 
    }

    pub fn set_dirty_bit(&mut self, dirty_bit: u32) { 
        self.dirty_bit = dirty_bit; 
    }
    
    pub fn set_time(&mut self, time: u32) { 
        self.time = time; 
    }


}