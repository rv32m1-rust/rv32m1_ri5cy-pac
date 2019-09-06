#[doc = "Reader of register CMR"]
pub type R = crate::R<u32, super::CMR>;
#[doc = "Writer for register CMR"]
pub type W = crate::W<u32, super::CMR>;
#[doc = "Register CMR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `COMPARE`"]
pub type COMPARE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMPARE`"]
pub struct COMPARE_W<'a> {
  w: &'a mut W,
}
impl<'a> COMPARE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Compare Value"]
  #[inline(always)]
  pub fn compare(&self) -> COMPARE_R {
    COMPARE_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Compare Value"]
  #[inline(always)]
  pub fn compare(&mut self) -> COMPARE_W {
    COMPARE_W { w: self }
  }
}
