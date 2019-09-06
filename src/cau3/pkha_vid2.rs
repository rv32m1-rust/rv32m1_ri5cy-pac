#[doc = "Reader of register PKHA_VID2"]
pub type R = crate::R<u32, super::PKHA_VID2>;
#[doc = "Reader of field `ECO_REV`"]
pub type ECO_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `ARCH_ERA`"]
pub type ARCH_ERA_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - ECO Revision Number"]
  #[inline(always)]
  pub fn eco_rev(&self) -> ECO_REV_R {
    ECO_REV_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - Architecture ERA"]
  #[inline(always)]
  pub fn arch_era(&self) -> ARCH_ERA_R {
    ARCH_ERA_R::new(((self.bits >> 8) & 0xff) as u8)
  }
}
