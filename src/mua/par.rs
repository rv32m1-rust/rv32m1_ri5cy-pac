#[doc = "Reader of register PAR"]
pub type R = crate::R<u32, super::PAR>;
#[doc = "Reader of field `PARAMETER`"]
pub type PARAMETER_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - This bitfield contains the parameter settings of MUA."]
  #[inline(always)]
  pub fn parameter(&self) -> PARAMETER_R {
    PARAMETER_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
