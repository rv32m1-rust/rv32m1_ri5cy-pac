#[doc = "Reader of register PKASZ"]
pub type R = crate::R<u32, super::PKASZ>;
#[doc = "Writer for register PKASZ"]
pub type W = crate::W<u32, super::PKASZ>;
#[doc = "Register PKASZ `reset()`'s with value 0"]
impl crate::ResetValue for super::PKASZ {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PKASZ`"]
pub type PKASZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKASZ`"]
pub struct PKASZ_W<'a> {
  w: &'a mut W,
}
impl<'a> PKASZ_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:8 - PKHA A Size"]
  #[inline(always)]
  pub fn pkasz(&self) -> PKASZ_R {
    PKASZ_R::new((self.bits & 0x01ff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:8 - PKHA A Size"]
  #[inline(always)]
  pub fn pkasz(&mut self) -> PKASZ_W {
    PKASZ_W { w: self }
  }
}
