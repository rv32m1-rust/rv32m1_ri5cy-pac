#[doc = "Reader of register DERR_W0_17"]
pub type R = crate::R<u32, super::DERR_W0_17>;
#[doc = "Reader of field `EADDR`"]
pub type EADDR_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - Error address"]
  #[inline(always)]
  pub fn eaddr(&self) -> EADDR_R {
    EADDR_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
