#[doc = "Reader of register PLAMC"]
pub type R = crate::R<u16, super::PLAMC>;
#[doc = "Reader of field `AMC`"]
pub type AMC_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
  #[inline(always)]
  pub fn amc(&self) -> AMC_R {
    AMC_R::new((self.bits & 0xff) as u8)
  }
}
