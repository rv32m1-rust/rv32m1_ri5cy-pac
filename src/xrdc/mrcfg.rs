#[doc = "Reader of register MRCFG%s"]
pub type R = crate::R<u8, super::MRCFG>;
#[doc = "Reader of field `NMRGD`"]
pub type NMRGD_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:4 - Number of memory region descriptors for memory region controller n"]
  #[inline(always)]
  pub fn nmrgd(&self) -> NMRGD_R {
    NMRGD_R::new((self.bits & 0x1f) as u8)
  }
}
