#[doc = "Reader of register FRMNUML"]
pub type R = crate::R<u8, super::FRMNUML>;
#[doc = "Writer for register FRMNUML"]
pub type W = crate::W<u8, super::FRMNUML>;
#[doc = "Register FRMNUML `reset()`'s with value 0"]
impl crate::ResetValue for super::FRMNUML {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `FRM`"]
pub type FRM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRM`"]
pub struct FRM_W<'a> {
  w: &'a mut W,
}
impl<'a> FRM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:7 - FRM"]
  #[inline(always)]
  pub fn frm(&self) -> FRM_R {
    FRM_R::new((self.bits & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 0:7 - FRM"]
  #[inline(always)]
  pub fn frm(&mut self) -> FRM_W {
    FRM_W { w: self }
  }
}
