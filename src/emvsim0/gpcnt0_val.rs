#[doc = "Reader of register GPCNT0_VAL"]
pub type R = crate::R<u32, super::GPCNT0_VAL>;
#[doc = "Writer for register GPCNT0_VAL"]
pub type W = crate::W<u32, super::GPCNT0_VAL>;
#[doc = "Register GPCNT0_VAL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::GPCNT0_VAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xffff
  }
}
#[doc = "Reader of field `GPCNT0`"]
pub type GPCNT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPCNT0`"]
pub struct GPCNT0_W<'a> {
  w: &'a mut W,
}
impl<'a> GPCNT0_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:15 - General Purpose Counter 0 Timeout Value"]
  #[inline(always)]
  pub fn gpcnt0(&self) -> GPCNT0_R {
    GPCNT0_R::new((self.bits & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:15 - General Purpose Counter 0 Timeout Value"]
  #[inline(always)]
  pub fn gpcnt0(&mut self) -> GPCNT0_W {
    GPCNT0_W { w: self }
  }
}
