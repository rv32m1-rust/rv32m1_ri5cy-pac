#[doc = "Reader of register RX_THD"]
pub type R = crate::R<u32, super::RX_THD>;
#[doc = "Writer for register RX_THD"]
pub type W = crate::W<u32, super::RX_THD>;
#[doc = "Register RX_THD `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RX_THD {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x01
  }
}
#[doc = "Reader of field `RDT`"]
pub type RDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDT`"]
pub struct RDT_W<'a> {
  w: &'a mut W,
}
impl<'a> RDT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
    self.w
  }
}
#[doc = "Receiver NACK Threshold Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNCK_THD_A {
  #[doc = "0: Zero Threshold. RTE will not be set"]
  RNCK_THD_0,
}
impl From<RNCK_THD_A> for u8 {
  #[inline(always)]
  fn from(variant: RNCK_THD_A) -> Self {
    match variant {
      RNCK_THD_A::RNCK_THD_0 => 0,
    }
  }
}
#[doc = "Reader of field `RNCK_THD`"]
pub type RNCK_THD_R = crate::R<u8, RNCK_THD_A>;
impl RNCK_THD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RNCK_THD_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RNCK_THD_A::RNCK_THD_0),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RNCK_THD_0`"]
  #[inline(always)]
  pub fn is_rnck_thd_0(&self) -> bool {
    *self == RNCK_THD_A::RNCK_THD_0
  }
}
#[doc = "Write proxy for field `RNCK_THD`"]
pub struct RNCK_THD_W<'a> {
  w: &'a mut W,
}
impl<'a> RNCK_THD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RNCK_THD_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Zero Threshold. RTE will not be set"]
  #[inline(always)]
  pub fn rnck_thd_0(self) -> &'a mut W {
    self.variant(RNCK_THD_A::RNCK_THD_0)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:3 - Receiver Data Threshold Value"]
  #[inline(always)]
  pub fn rdt(&self) -> RDT_R {
    RDT_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 8:11 - Receiver NACK Threshold Value"]
  #[inline(always)]
  pub fn rnck_thd(&self) -> RNCK_THD_R {
    RNCK_THD_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:3 - Receiver Data Threshold Value"]
  #[inline(always)]
  pub fn rdt(&mut self) -> RDT_W {
    RDT_W { w: self }
  }
  #[doc = "Bits 8:11 - Receiver NACK Threshold Value"]
  #[inline(always)]
  pub fn rnck_thd(&mut self) -> RNCK_THD_W {
    RNCK_THD_W { w: self }
  }
}
