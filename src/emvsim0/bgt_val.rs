#[doc = "Reader of register BGT_VAL"]
pub type R = crate::R<u32, super::BGT_VAL>;
#[doc = "Writer for register BGT_VAL"]
pub type W = crate::W<u32, super::BGT_VAL>;
#[doc = "Register BGT_VAL `reset()`'s with value 0"]
impl crate::ResetValue for super::BGT_VAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `BGT`"]
pub type BGT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BGT`"]
pub struct BGT_W<'a> {
  w: &'a mut W,
}
impl<'a> BGT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:15 - Block Guard Time Value"]
  #[inline(always)]
  pub fn bgt(&self) -> BGT_R {
    BGT_R::new((self.bits & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:15 - Block Guard Time Value"]
  #[inline(always)]
  pub fn bgt(&mut self) -> BGT_W {
    BGT_W { w: self }
  }
}
