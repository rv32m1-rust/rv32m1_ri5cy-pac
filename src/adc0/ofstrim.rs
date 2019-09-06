#[doc = "Reader of register OFSTRIM"]
pub type R = crate::R<u32, super::OFSTRIM>;
#[doc = "Writer for register OFSTRIM"]
pub type W = crate::W<u32, super::OFSTRIM>;
#[doc = "Register OFSTRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::OFSTRIM {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `OFSTRIM`"]
pub type OFSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFSTRIM`"]
pub struct OFSTRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> OFSTRIM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:5 - Trim for offset"]
  #[inline(always)]
  pub fn ofstrim(&self) -> OFSTRIM_R {
    OFSTRIM_R::new((self.bits & 0x3f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:5 - Trim for offset"]
  #[inline(always)]
  pub fn ofstrim(&mut self) -> OFSTRIM_W {
    OFSTRIM_W { w: self }
  }
}
