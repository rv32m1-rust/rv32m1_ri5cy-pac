#[doc = "Reader of register TRM4"]
pub type R = crate::R<u8, super::TRM4>;
#[doc = "Writer for register TRM4"]
pub type W = crate::W<u8, super::TRM4>;
#[doc = "Register TRM4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRM4 {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "VREF 2.1V Trim Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM2V1_A {
  #[doc = "0: Max"]
  TRIM2V1_0,
  #[doc = "63: Min"]
  TRIM2V1_63,
}
impl From<TRIM2V1_A> for u8 {
  #[inline(always)]
  fn from(variant: TRIM2V1_A) -> Self {
    match variant {
      TRIM2V1_A::TRIM2V1_0 => 0,
      TRIM2V1_A::TRIM2V1_63 => 63,
    }
  }
}
#[doc = "Reader of field `TRIM2V1`"]
pub type TRIM2V1_R = crate::R<u8, TRIM2V1_A>;
impl TRIM2V1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TRIM2V1_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TRIM2V1_A::TRIM2V1_0),
      63 => Val(TRIM2V1_A::TRIM2V1_63),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TRIM2V1_0`"]
  #[inline(always)]
  pub fn is_trim2v1_0(&self) -> bool {
    *self == TRIM2V1_A::TRIM2V1_0
  }
  #[doc = "Checks if the value of the field is `TRIM2V1_63`"]
  #[inline(always)]
  pub fn is_trim2v1_63(&self) -> bool {
    *self == TRIM2V1_A::TRIM2V1_63
  }
}
#[doc = "Write proxy for field `TRIM2V1`"]
pub struct TRIM2V1_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIM2V1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIM2V1_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Max"]
  #[inline(always)]
  pub fn trim2v1_0(self) -> &'a mut W {
    self.variant(TRIM2V1_A::TRIM2V1_0)
  }
  #[doc = "Min"]
  #[inline(always)]
  pub fn trim2v1_63(self) -> &'a mut W {
    self.variant(TRIM2V1_A::TRIM2V1_63)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
    self.w
  }
}
#[doc = "Internal Voltage Reference (2.1V) Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF2V1_EN_A {
  #[doc = "0: VREF 2.1V is enabled"]
  VREF2V1_EN_0,
  #[doc = "1: VREF 2.1V is disabled"]
  VREF2V1_EN_1,
}
impl From<VREF2V1_EN_A> for bool {
  #[inline(always)]
  fn from(variant: VREF2V1_EN_A) -> Self {
    match variant {
      VREF2V1_EN_A::VREF2V1_EN_0 => false,
      VREF2V1_EN_A::VREF2V1_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `VREF2V1_EN`"]
pub type VREF2V1_EN_R = crate::R<bool, VREF2V1_EN_A>;
impl VREF2V1_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VREF2V1_EN_A {
    match self.bits {
      false => VREF2V1_EN_A::VREF2V1_EN_0,
      true => VREF2V1_EN_A::VREF2V1_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `VREF2V1_EN_0`"]
  #[inline(always)]
  pub fn is_vref2v1_en_0(&self) -> bool {
    *self == VREF2V1_EN_A::VREF2V1_EN_0
  }
  #[doc = "Checks if the value of the field is `VREF2V1_EN_1`"]
  #[inline(always)]
  pub fn is_vref2v1_en_1(&self) -> bool {
    *self == VREF2V1_EN_A::VREF2V1_EN_1
  }
}
#[doc = "Write proxy for field `VREF2V1_EN`"]
pub struct VREF2V1_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> VREF2V1_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VREF2V1_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VREF 2.1V is enabled"]
  #[inline(always)]
  pub fn vref2v1_en_0(self) -> &'a mut W {
    self.variant(VREF2V1_EN_A::VREF2V1_EN_0)
  }
  #[doc = "VREF 2.1V is disabled"]
  #[inline(always)]
  pub fn vref2v1_en_1(self) -> &'a mut W {
    self.variant(VREF2V1_EN_A::VREF2V1_EN_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:5 - VREF 2.1V Trim Bits"]
  #[inline(always)]
  pub fn trim2v1(&self) -> TRIM2V1_R {
    TRIM2V1_R::new((self.bits & 0x3f) as u8)
  }
  #[doc = "Bit 7 - Internal Voltage Reference (2.1V) Enable"]
  #[inline(always)]
  pub fn vref2v1_en(&self) -> VREF2V1_EN_R {
    VREF2V1_EN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:5 - VREF 2.1V Trim Bits"]
  #[inline(always)]
  pub fn trim2v1(&mut self) -> TRIM2V1_W {
    TRIM2V1_W { w: self }
  }
  #[doc = "Bit 7 - Internal Voltage Reference (2.1V) Enable"]
  #[inline(always)]
  pub fn vref2v1_en(&mut self) -> VREF2V1_EN_W {
    VREF2V1_EN_W { w: self }
  }
}
