#[doc = "Reader of register WOR_DURATION"]
pub type R = crate::R<u32, super::WOR_DURATION>;
#[doc = "Reader of field `WOR_DSM_DURATION`"]
pub type WOR_DSM_DURATION_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:23 - WOR Deep Sleep Time Elapsed"]
  #[inline(always)]
  pub fn wor_dsm_duration(&self) -> WOR_DSM_DURATION_R {
    WOR_DSM_DURATION_R::new((self.bits & 0x00ff_ffff) as u32)
  }
}
