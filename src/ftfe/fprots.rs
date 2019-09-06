#[doc = "Reader of register FPROTS%s"]
pub type R = crate::R<u8, super::FPROTS>;
#[doc = "Writer for register FPROTS%s"]
pub type W = crate::W<u8, super::FPROTS>;
#[doc = "Register FPROTS%s `reset()`'s with value 0"]
impl crate::ResetValue for super::FPROTS {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PROTS`"]
pub type PROTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTS`"]
pub struct PROTS_W<'a> {
  w: &'a mut W,
}
impl<'a> PROTS_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:7 - Secondary Program Flash Region Protect"]
  #[inline(always)]
  pub fn prots(&self) -> PROTS_R {
    PROTS_R::new((self.bits & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 0:7 - Secondary Program Flash Region Protect"]
  #[inline(always)]
  pub fn prots(&mut self) -> PROTS_W {
    PROTS_W { w: self }
  }
}
