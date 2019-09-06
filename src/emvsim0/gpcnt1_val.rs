#[doc = "Reader of register GPCNT1_VAL"]
pub type R = crate::R<u32, super::GPCNT1_VAL>;
#[doc = "Writer for register GPCNT1_VAL"]
pub type W = crate::W<u32, super::GPCNT1_VAL>;
#[doc = "Register GPCNT1_VAL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::GPCNT1_VAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xffff
  }
}
#[doc = "Reader of field `GPCNT1`"]
pub type GPCNT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPCNT1`"]
pub struct GPCNT1_W<'a> {
  w: &'a mut W,
}
impl<'a> GPCNT1_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:15 - General Purpose Counter 1 Timeout Value"]
  #[inline(always)]
  pub fn gpcnt1(&self) -> GPCNT1_R {
    GPCNT1_R::new((self.bits & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:15 - General Purpose Counter 1 Timeout Value"]
  #[inline(always)]
  pub fn gpcnt1(&mut self) -> GPCNT1_W {
    GPCNT1_W { w: self }
  }
}
