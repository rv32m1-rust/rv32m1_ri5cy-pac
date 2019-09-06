#[doc = "Reader of register MAN_WAKE"]
pub type R = crate::R<u32, super::MAN_WAKE>;
#[doc = "Writer for register MAN_WAKE"]
pub type W = crate::W<u32, super::MAN_WAKE>;
#[doc = "Register MAN_WAKE `reset()`'s with value 0"]
impl crate::ResetValue for super::MAN_WAKE {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `MAN_WAKE_TIME`"]
pub type MAN_WAKE_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MAN_WAKE_TIME`"]
pub struct MAN_WAKE_TIME_W<'a> {
  w: &'a mut W,
}
impl<'a> MAN_WAKE_TIME_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
    self.w
  }
}
#[doc = "Reader of field `MAN_FSM_STATE`"]
pub type MAN_FSM_STATE_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:23 - MAN Deep Sleep Module Wake Time"]
  #[inline(always)]
  pub fn man_wake_time(&self) -> MAN_WAKE_TIME_R {
    MAN_WAKE_TIME_R::new((self.bits & 0x00ff_ffff) as u32)
  }
  #[doc = "Bits 28:30 - MAN Deep Sleep State Machine State"]
  #[inline(always)]
  pub fn man_fsm_state(&self) -> MAN_FSM_STATE_R {
    MAN_FSM_STATE_R::new(((self.bits >> 28) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:23 - MAN Deep Sleep Module Wake Time"]
  #[inline(always)]
  pub fn man_wake_time(&mut self) -> MAN_WAKE_TIME_W {
    MAN_WAKE_TIME_W { w: self }
  }
}
