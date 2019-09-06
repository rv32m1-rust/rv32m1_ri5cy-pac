#[doc = "Reader of register PIN"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Reader of field `PDI`"]
pub type PDI_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - Pin Data Input"]
  #[inline(always)]
  pub fn pdi(&self) -> PDI_R {
    PDI_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
