extern crate conv;
pub use conv::conversion::*;
use std::any::Any;
pub trait Interconnect: InterconnectClone{
    fn load8(&self, addr:u32)->Option<u8>;
    fn store8(&mut self, addr:u32,val:u8);
    fn load16(&self, addr:u32)->Option<u16>{
        if let Some(b0) = self.load8(addr+0){
            if let Some(b1) = self.load8(addr+1){
               return Some(u8_to_u16(&[b0,b1])); 
            }
        }
        None
    }
    fn store16(&mut self, addr:u32,val:u16){
        let v = u16_to_u8(val);
        for i in 0..v.len(){
            self.store8(addr+i as u32,v[i]);
        }
    }
    fn load32(&self, addr:u32)->Option<u32>{
        if let Some(b0) = self.load8(addr+0){
            if let Some(b1) = self.load8(addr+1){
                if let Some(b2) = self.load8(addr+2){
                    if let Some(b3) = self.load8(addr+3){
                        return Some(u8_to_u32(&[b0,b1,b2,b3])); 
                    }
                }
            }
        }
        None
    }
    fn store32(&mut self, addr:u32,val:u32){
        let v = u32_to_u8(val);
        for i in 0..v.len(){
            self.store8(addr+i as u32,v[i]);
        }
    }
    fn as_any(&self) -> &dyn Any;
}
pub trait InterconnectClone {
    fn clone_box(&self) -> Box<Interconnect>;
}
impl<T> InterconnectClone for T 
where 
    T: 'static + Interconnect + Clone,
{
    fn clone_box(&self) -> Box<Interconnect>{
        Box::new(self.clone())
    }
}
impl Clone for Box<Interconnect>{
    fn clone(&self) -> Box<Interconnect>{
        self.clone_box()
    }
}