#[doc = "Reader of register DSM_TIMER"]
pub type R = crate::R<u32, super::DSM_TIMER>;
#[doc = "Reader of field `DSM_TIMER`"]
pub type DSM_TIMER_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:23 - Deep Sleep Mode Timer"]
  #[inline(always)]
  pub fn dsm_timer(&self) -> DSM_TIMER_R {
    DSM_TIMER_R::new((self.bits & 0x00ff_ffff) as u32)
  }
}
