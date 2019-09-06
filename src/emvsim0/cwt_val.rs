#[doc = "Reader of register CWT_VAL"]
pub type R = crate::R<u32, super::CWT_VAL>;
#[doc = "Writer for register CWT_VAL"]
pub type W = crate::W<u32, super::CWT_VAL>;
#[doc = "Register CWT_VAL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CWT_VAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xffff
  }
}
#[doc = "Reader of field `CWT`"]
pub type CWT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CWT`"]
pub struct CWT_W<'a> {
  w: &'a mut W,
}
impl<'a> CWT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:15 - Character Wait Time Value"]
  #[inline(always)]
  pub fn cwt(&self) -> CWT_R {
    CWT_R::new((self.bits & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:15 - Character Wait Time Value"]
  #[inline(always)]
  pub fn cwt(&mut self) -> CWT_W {
    CWT_W { w: self }
  }
}
