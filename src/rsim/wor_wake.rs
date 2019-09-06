#[doc = "Reader of register WOR_WAKE"]
pub type R = crate::R<u32, super::WOR_WAKE>;
#[doc = "Writer for register WOR_WAKE"]
pub type W = crate::W<u32, super::WOR_WAKE>;
#[doc = "Register WOR_WAKE `reset()`'s with value 0"]
impl crate::ResetValue for super::WOR_WAKE {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `WOR_WAKE_TIME`"]
pub type WOR_WAKE_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WOR_WAKE_TIME`"]
pub struct WOR_WAKE_TIME_W<'a> {
  w: &'a mut W,
}
impl<'a> WOR_WAKE_TIME_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
    self.w
  }
}
#[doc = "Reader of field `WOR_FSM_STATE`"]
pub type WOR_FSM_STATE_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:23 - WOR Deep Sleep Module Wake Time"]
  #[inline(always)]
  pub fn wor_wake_time(&self) -> WOR_WAKE_TIME_R {
    WOR_WAKE_TIME_R::new((self.bits & 0x00ff_ffff) as u32)
  }
  #[doc = "Bits 28:30 - WOR Deep Sleep State Machine State"]
  #[inline(always)]
  pub fn wor_fsm_state(&self) -> WOR_FSM_STATE_R {
    WOR_FSM_STATE_R::new(((self.bits >> 28) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:23 - WOR Deep Sleep Module Wake Time"]
  #[inline(always)]
  pub fn wor_wake_time(&mut self) -> WOR_WAKE_TIME_W {
    WOR_WAKE_TIME_W { w: self }
  }
}
