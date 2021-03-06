#[doc = "Reader of register CP1MASTER"]
pub type R = crate::R<u32, super::CP1MASTER>;
#[doc = "Reader of field `PPMN`"]
pub type PPMN_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:5 - Processor 1 Physical Master Number"]
  #[inline(always)]
  pub fn ppmn(&self) -> PPMN_R {
    PPMN_R::new((self.bits & 0x3f) as u8)
  }
}
