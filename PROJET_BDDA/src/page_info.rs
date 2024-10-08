pub struct PageInfo{
    file_id :  u32, 
    page_id : u32,
    pin_count : u32,
    dirty_bit : u32,
    time : u32
}

impl PageInfo{
    pub fn new(file_id :  u32, 
        page_id : u32,
        pin_count : u32,
        dirty_bit : u32,
        time : u32)->Self
    {
        Self { file_id, page_id, pin_count, dirty_bit, time }
    }

    pub fn get_file_id(&self)->u32{
        self.file_id
    }
    pub fn get_page_id(&self)->u32{
        self.page_id
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

    pub fn set_file_id(&mut self, file_id: u32) { self.file_id = file_id; }
    pub fn set_page_id(&mut self, page_id: u32) { self.page_id = page_id; }
    pub fn set_pin_count(&mut self, pin_count: u32) { self.pin_count = pin_count; }
    pub fn set_dirty_bit(&mut self, dirty_bit: u32) { self.dirty_bit = dirty_bit; }
    pub fn set_time(&mut self, time: u32) { self.time = time; }


}