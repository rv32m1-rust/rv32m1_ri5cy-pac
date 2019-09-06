#[doc = "Reader of register LPFLLCFG"]
pub type R = crate::R<u32, super::LPFLLCFG>;
#[doc = "Writer for register LPFLLCFG"]
pub type W = crate::W<u32, super::LPFLLCFG>;
#[doc = "Register LPFLLCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::LPFLLCFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSEL_A {
  #[doc = "0: LPFLL is trimmed to 48 MHz."]
  FSEL_0,
  #[doc = "1: LPFLL is trimmed to 72 MHz."]
  FSEL_1,
}
impl From<FSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: FSEL_A) -> Self {
    match variant {
      FSEL_A::FSEL_0 => 0,
      FSEL_A::FSEL_1 => 1,
    }
  }
}
#[doc = "Reader of field `FSEL`"]
pub type FSEL_R = crate::R<u8, FSEL_A>;
impl FSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FSEL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FSEL_A::FSEL_0),
      1 => Val(FSEL_A::FSEL_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FSEL_0`"]
  #[inline(always)]
  pub fn is_fsel_0(&self) -> bool {
    *self == FSEL_A::FSEL_0
  }
  #[doc = "Checks if the value of the field is `FSEL_1`"]
  #[inline(always)]
  pub fn is_fsel_1(&self) -> bool {
    *self == FSEL_A::FSEL_1
  }
}
#[doc = "Write proxy for field `FSEL`"]
pub struct FSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> FSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FSEL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "LPFLL is trimmed to 48 MHz."]
  #[inline(always)]
  pub fn fsel_0(self) -> &'a mut W {
    self.variant(FSEL_A::FSEL_0)
  }
  #[doc = "LPFLL is trimmed to 72 MHz."]
  #[inline(always)]
  pub fn fsel_1(self) -> &'a mut W {
    self.variant(FSEL_A::FSEL_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Frequency Select"]
  #[inline(always)]
  pub fn fsel(&self) -> FSEL_R {
    FSEL_R::new((self.bits & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Frequency Select"]
  #[inline(always)]
  pub fn fsel(&mut self) -> FSEL_W {
    FSEL_W { w: self }
  }
}
