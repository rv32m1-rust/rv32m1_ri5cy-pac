#[doc = "Reader of register TRM"]
pub type R = crate::R<u8, super::TRM>;
#[doc = "Writer for register TRM"]
pub type W = crate::W<u8, super::TRM>;
#[doc = "Register TRM `reset()`'s with value 0"]
impl crate::ResetValue for super::TRM {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Trim bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_A {
  #[doc = "0: Min"]
  TRIM_0,
  #[doc = "63: Max"]
  TRIM_63,
}
impl From<TRIM_A> for u8 {
  #[inline(always)]
  fn from(variant: TRIM_A) -> Self {
    match variant {
      TRIM_A::TRIM_0 => 0,
      TRIM_A::TRIM_63 => 63,
    }
  }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u8, TRIM_A>;
impl TRIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TRIM_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TRIM_A::TRIM_0),
      63 => Val(TRIM_A::TRIM_63),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TRIM_0`"]
  #[inline(always)]
  pub fn is_trim_0(&self) -> bool {
    *self == TRIM_A::TRIM_0
  }
  #[doc = "Checks if the value of the field is `TRIM_63`"]
  #[inline(always)]
  pub fn is_trim_63(&self) -> bool {
    *self == TRIM_A::TRIM_63
  }
}
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIM_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Min"]
  #[inline(always)]
  pub fn trim_0(self) -> &'a mut W {
    self.variant(TRIM_A::TRIM_0)
  }
  #[doc = "Max"]
  #[inline(always)]
  pub fn trim_63(self) -> &'a mut W {
    self.variant(TRIM_A::TRIM_63)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
    self.w
  }
}
#[doc = "Chop oscillator enable. When set, the internal chopping operation is enabled and the internal analog offset will be minimized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHOPEN_A {
  #[doc = "0: Chop oscillator is disabled."]
  CHOPEN_0,
  #[doc = "1: Chop oscillator is enabled."]
  CHOPEN_1,
}
impl From<CHOPEN_A> for bool {
  #[inline(always)]
  fn from(variant: CHOPEN_A) -> Self {
    match variant {
      CHOPEN_A::CHOPEN_0 => false,
      CHOPEN_A::CHOPEN_1 => true,
    }
  }
}
#[doc = "Reader of field `CHOPEN`"]
pub type CHOPEN_R = crate::R<bool, CHOPEN_A>;
impl CHOPEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CHOPEN_A {
    match self.bits {
      false => CHOPEN_A::CHOPEN_0,
      true => CHOPEN_A::CHOPEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CHOPEN_0`"]
  #[inline(always)]
  pub fn is_chopen_0(&self) -> bool {
    *self == CHOPEN_A::CHOPEN_0
  }
  #[doc = "Checks if the value of the field is `CHOPEN_1`"]
  #[inline(always)]
  pub fn is_chopen_1(&self) -> bool {
    *self == CHOPEN_A::CHOPEN_1
  }
}
#[doc = "Write proxy for field `CHOPEN`"]
pub struct CHOPEN_W<'a> {
  w: &'a mut W,
}
impl<'a> CHOPEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CHOPEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Chop oscillator is disabled."]
  #[inline(always)]
  pub fn chopen_0(self) -> &'a mut W {
    self.variant(CHOPEN_A::CHOPEN_0)
  }
  #[doc = "Chop oscillator is enabled."]
  #[inline(always)]
  pub fn chopen_1(self) -> &'a mut W {
    self.variant(CHOPEN_A::CHOPEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:5 - Trim bits"]
  #[inline(always)]
  pub fn trim(&self) -> TRIM_R {
    TRIM_R::new((self.bits & 0x3f) as u8)
  }
  #[doc = "Bit 6 - Chop oscillator enable. When set, the internal chopping operation is enabled and the internal analog offset will be minimized."]
  #[inline(always)]
  pub fn chopen(&self) -> CHOPEN_R {
    CHOPEN_R::new(((self.bits >> 6) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:5 - Trim bits"]
  #[inline(always)]
  pub fn trim(&mut self) -> TRIM_W {
    TRIM_W { w: self }
  }
  #[doc = "Bit 6 - Chop oscillator enable. When set, the internal chopping operation is enabled and the internal analog offset will be minimized."]
  #[inline(always)]
  pub fn chopen(&mut self) -> CHOPEN_W {
    CHOPEN_W { w: self }
  }
}
