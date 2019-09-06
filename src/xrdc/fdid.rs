#[doc = "Reader of register FDID"]
pub type R = crate::R<u32, super::FDID>;
#[doc = "Writer for register FDID"]
pub type W = crate::W<u32, super::FDID>;
#[doc = "Register FDID `reset()`'s with value 0"]
impl crate::ResetValue for super::FDID {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `FDID`"]
pub type FDID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDID`"]
pub struct FDID_W<'a> {
  w: &'a mut W,
}
impl<'a> FDID_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:3 - Domain ID of Faulted Access"]
  #[inline(always)]
  pub fn fdid(&self) -> FDID_R {
    FDID_R::new((self.bits & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:3 - Domain ID of Faulted Access"]
  #[inline(always)]
  pub fn fdid(&mut self) -> FDID_W {
    FDID_W { w: self }
  }
}
