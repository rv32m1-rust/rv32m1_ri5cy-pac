#[doc = "Reader of register TX_GETU"]
pub type R = crate::R<u32, super::TX_GETU>;
#[doc = "Writer for register TX_GETU"]
pub type W = crate::W<u32, super::TX_GETU>;
#[doc = "Register TX_GETU `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_GETU {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Transmitter Guard Time Value in ETU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GETU_A {
  #[doc = "0: no additional ETUs inserted (default)"]
  GETU_0,
  #[doc = "1: 1 additional ETU inserted"]
  GETU_1,
  #[doc = "254: 254 additional ETUs inserted"]
  GETU_254,
  #[doc = "255: Subtracts one ETU by reducing the number of STOP bits from two to one"]
  GETU_255,
}
impl From<GETU_A> for u8 {
  #[inline(always)]
  fn from(variant: GETU_A) -> Self {
    match variant {
      GETU_A::GETU_0 => 0,
      GETU_A::GETU_1 => 1,
      GETU_A::GETU_254 => 254,
      GETU_A::GETU_255 => 255,
    }
  }
}
#[doc = "Reader of field `GETU`"]
pub type GETU_R = crate::R<u8, GETU_A>;
impl GETU_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GETU_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GETU_A::GETU_0),
      1 => Val(GETU_A::GETU_1),
      254 => Val(GETU_A::GETU_254),
      255 => Val(GETU_A::GETU_255),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GETU_0`"]
  #[inline(always)]
  pub fn is_getu_0(&self) -> bool {
    *self == GETU_A::GETU_0
  }
  #[doc = "Checks if the value of the field is `GETU_1`"]
  #[inline(always)]
  pub fn is_getu_1(&self) -> bool {
    *self == GETU_A::GETU_1
  }
  #[doc = "Checks if the value of the field is `GETU_254`"]
  #[inline(always)]
  pub fn is_getu_254(&self) -> bool {
    *self == GETU_A::GETU_254
  }
  #[doc = "Checks if the value of the field is `GETU_255`"]
  #[inline(always)]
  pub fn is_getu_255(&self) -> bool {
    *self == GETU_A::GETU_255
  }
}
#[doc = "Write proxy for field `GETU`"]
pub struct GETU_W<'a> {
  w: &'a mut W,
}
impl<'a> GETU_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GETU_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no additional ETUs inserted (default)"]
  #[inline(always)]
  pub fn getu_0(self) -> &'a mut W {
    self.variant(GETU_A::GETU_0)
  }
  #[doc = "1 additional ETU inserted"]
  #[inline(always)]
  pub fn getu_1(self) -> &'a mut W {
    self.variant(GETU_A::GETU_1)
  }
  #[doc = "254 additional ETUs inserted"]
  #[inline(always)]
  pub fn getu_254(self) -> &'a mut W {
    self.variant(GETU_A::GETU_254)
  }
  #[doc = "Subtracts one ETU by reducing the number of STOP bits from two to one"]
  #[inline(always)]
  pub fn getu_255(self) -> &'a mut W {
    self.variant(GETU_A::GETU_255)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
  #[inline(always)]
  pub fn getu(&self) -> GETU_R {
    GETU_R::new((self.bits & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
  #[inline(always)]
  pub fn getu(&mut self) -> GETU_W {
    GETU_W { w: self }
  }
}
