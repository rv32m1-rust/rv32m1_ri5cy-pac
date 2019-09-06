#[doc = "Reader of register RCTRL"]
pub type R = crate::R<u32, super::RCTRL>;
#[doc = "Writer for register RCTRL"]
pub type W = crate::W<u32, super::RCTRL>;
#[doc = "Register RCTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x03
  }
}
#[doc = "Reader of field `REGSEL`"]
pub type REGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGSEL`"]
pub struct REGSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> REGSEL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - REGSEL"]
  #[inline(always)]
  pub fn regsel(&self) -> REGSEL_R {
    REGSEL_R::new((self.bits & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - REGSEL"]
  #[inline(always)]
  pub fn regsel(&mut self) -> REGSEL_W {
    REGSEL_W { w: self }
  }
}
