pub struct RecordId{
    pageId: PageId,
    slotIdx: u32,
}

impl RecordId{
    pub fn new(pageId: PageId, slotIdx: u32) -> Self{
        Self{
            pageId,
            slotIdx,
        }
    }
    
    pub fn get_page_id(&self) -> &PageId {
        return &pageId;
    }
    
    pub fn get_slot_idx(self) -> u32 {
        return slotIdx;
    } 

}
