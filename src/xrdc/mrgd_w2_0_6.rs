#[doc = "Reader of register MRGD_W2_0_6"]
pub type R = crate::R<u32, super::MRGD_W2_0_6>;
#[doc = "Writer for register MRGD_W2_0_6"]
pub type W = crate::W<u32, super::MRGD_W2_0_6>;
#[doc = "Register MRGD_W2_0_6 `reset()`'s with value 0"]
impl crate::ResetValue for super::MRGD_W2_0_6 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `D0SEL`"]
pub type D0SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D0SEL`"]
pub struct D0SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> D0SEL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Reader of field `D1SEL`"]
pub type D1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D1SEL`"]
pub struct D1SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> D1SEL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
    self.w
  }
}
#[doc = "Reader of field `D2SEL`"]
pub type D2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D2SEL`"]
pub struct D2SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> D2SEL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
    self.w
  }
}
#[doc = "Reader of field `EALO`"]
pub type EALO_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:2 - Domain 0 select"]
  #[inline(always)]
  pub fn d0sel(&self) -> D0SEL_R {
    D0SEL_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 3:5 - Domain 1 select"]
  #[inline(always)]
  pub fn d1sel(&self) -> D1SEL_R {
    D1SEL_R::new(((self.bits >> 3) & 0x07) as u8)
  }
  #[doc = "Bits 6:8 - Domain 2 select"]
  #[inline(always)]
  pub fn d2sel(&self) -> D2SEL_R {
    D2SEL_R::new(((self.bits >> 6) & 0x07) as u8)
  }
  #[doc = "Bits 24:27 - Exclusive Access Lock Owner"]
  #[inline(always)]
  pub fn ealo(&self) -> EALO_R {
    EALO_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Domain 0 select"]
  #[inline(always)]
  pub fn d0sel(&mut self) -> D0SEL_W {
    D0SEL_W { w: self }
  }
  #[doc = "Bits 3:5 - Domain 1 select"]
  #[inline(always)]
  pub fn d1sel(&mut self) -> D1SEL_W {
    D1SEL_W { w: self }
  }
  #[doc = "Bits 6:8 - Domain 2 select"]
  #[inline(always)]
  pub fn d2sel(&mut self) -> D2SEL_W {
    D2SEL_W { w: self }
  }
}
