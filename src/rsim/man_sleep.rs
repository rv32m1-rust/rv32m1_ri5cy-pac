#[doc = "Reader of register MAN_SLEEP"]
pub type R = crate::R<u32, super::MAN_SLEEP>;
#[doc = "Writer for register MAN_SLEEP"]
pub type W = crate::W<u32, super::MAN_SLEEP>;
#[doc = "Register MAN_SLEEP `reset()`'s with value 0"]
impl crate::ResetValue for super::MAN_SLEEP {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `MAN_SLEEP_TIME`"]
pub type MAN_SLEEP_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MAN_SLEEP_TIME`"]
pub struct MAN_SLEEP_TIME_W<'a> {
  w: &'a mut W,
}
impl<'a> MAN_SLEEP_TIME_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:23 - MAN Deep Sleep Module Sleep Time"]
  #[inline(always)]
  pub fn man_sleep_time(&self) -> MAN_SLEEP_TIME_R {
    MAN_SLEEP_TIME_R::new((self.bits & 0x00ff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:23 - MAN Deep Sleep Module Sleep Time"]
  #[inline(always)]
  pub fn man_sleep_time(&mut self) -> MAN_SLEEP_TIME_W {
    MAN_SLEEP_TIME_W { w: self }
  }
}
