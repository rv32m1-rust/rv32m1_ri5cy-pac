#[doc = "Reader of register HWCFG1"]
pub type R = crate::R<u32, super::HWCFG1>;
#[doc = "Reader of field `DID`"]
pub type DID_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:3 - Domain identifier number"]
  #[inline(always)]
  pub fn did(&self) -> DID_R {
    DID_R::new((self.bits & 0x0f) as u8)
  }
}
