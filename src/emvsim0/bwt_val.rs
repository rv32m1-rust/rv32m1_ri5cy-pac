#[doc = "Reader of register BWT_VAL"]
pub type R = crate::R<u32, super::BWT_VAL>;
#[doc = "Writer for register BWT_VAL"]
pub type W = crate::W<u32, super::BWT_VAL>;
#[doc = "Register BWT_VAL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::BWT_VAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xffff_ffff
  }
}
#[doc = "Reader of field `BWT`"]
pub type BWT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BWT`"]
pub struct BWT_W<'a> {
  w: &'a mut W,
}
impl<'a> BWT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Block Wait Time Value"]
  #[inline(always)]
  pub fn bwt(&self) -> BWT_R {
    BWT_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Block Wait Time Value"]
  #[inline(always)]
  pub fn bwt(&mut self) -> BWT_W {
    BWT_W { w: self }
  }
}
