#[doc = "Reader of register ARR"]
pub type R = crate::R<u32, super::ARR>;
#[doc = "Writer for register ARR"]
pub type W = crate::W<u32, super::ARR>;
#[doc = "Register ARR `reset()`'s with value 0"]
impl crate::ResetValue for super::ARR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `ARRL`"]
pub type ARRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ARRL`"]
pub struct ARRL_W<'a> {
  w: &'a mut W,
}
impl<'a> ARRL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Address Remap Register List"]
  #[inline(always)]
  pub fn arrl(&self) -> ARRL_R {
    ARRL_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Address Remap Register List"]
  #[inline(always)]
  pub fn arrl(&mut self) -> ARRL_W {
    ARRL_W { w: self }
  }
}
