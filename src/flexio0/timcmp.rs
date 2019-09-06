#[doc = "Reader of register TIMCMP[%s]"]
pub type R = crate::R<u32, super::TIMCMP>;
#[doc = "Writer for register TIMCMP[%s]"]
pub type W = crate::W<u32, super::TIMCMP>;
#[doc = "Register TIMCMP[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMCMP {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `CMP`"]
pub type CMP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMP`"]
pub struct CMP_W<'a> {
  w: &'a mut W,
}
impl<'a> CMP_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:15 - Timer Compare Value"]
  #[inline(always)]
  pub fn cmp(&self) -> CMP_R {
    CMP_R::new((self.bits & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:15 - Timer Compare Value"]
  #[inline(always)]
  pub fn cmp(&mut self) -> CMP_W {
    CMP_W { w: self }
  }
}
