#[doc = "Reader of register QDCTRL"]
pub type R = crate::R<u32, super::QDCTRL>;
#[doc = "Writer for register QDCTRL"]
pub type W = crate::W<u32, super::QDCTRL>;
#[doc = "Register QDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::QDCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "QUADEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADEN_A {
  #[doc = "0: Quadrature decoder mode is disabled."]
  QUADEN_0,
  #[doc = "1: Quadrature decoder mode is enabled."]
  QUADEN_1,
}
impl From<QUADEN_A> for bool {
  #[inline(always)]
  fn from(variant: QUADEN_A) -> Self {
    match variant {
      QUADEN_A::QUADEN_0 => false,
      QUADEN_A::QUADEN_1 => true,
    }
  }
}
#[doc = "Reader of field `QUADEN`"]
pub type QUADEN_R = crate::R<bool, QUADEN_A>;
impl QUADEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> QUADEN_A {
    match self.bits {
      false => QUADEN_A::QUADEN_0,
      true => QUADEN_A::QUADEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `QUADEN_0`"]
  #[inline(always)]
  pub fn is_quaden_0(&self) -> bool {
    *self == QUADEN_A::QUADEN_0
  }
  #[doc = "Checks if the value of the field is `QUADEN_1`"]
  #[inline(always)]
  pub fn is_quaden_1(&self) -> bool {
    *self == QUADEN_A::QUADEN_1
  }
}
#[doc = "Write proxy for field `QUADEN`"]
pub struct QUADEN_W<'a> {
  w: &'a mut W,
}
impl<'a> QUADEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: QUADEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Quadrature decoder mode is disabled."]
  #[inline(always)]
  pub fn quaden_0(self) -> &'a mut W {
    self.variant(QUADEN_A::QUADEN_0)
  }
  #[doc = "Quadrature decoder mode is enabled."]
  #[inline(always)]
  pub fn quaden_1(self) -> &'a mut W {
    self.variant(QUADEN_A::QUADEN_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "TOFDIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFDIR_A {
  #[doc = "0: TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (zero) to its maximum value (MOD register)."]
  TOFDIR_0,
  #[doc = "1: TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (zero)."]
  TOFDIR_1,
}
impl From<TOFDIR_A> for bool {
  #[inline(always)]
  fn from(variant: TOFDIR_A) -> Self {
    match variant {
      TOFDIR_A::TOFDIR_0 => false,
      TOFDIR_A::TOFDIR_1 => true,
    }
  }
}
#[doc = "Reader of field `TOFDIR`"]
pub type TOFDIR_R = crate::R<bool, TOFDIR_A>;
impl TOFDIR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TOFDIR_A {
    match self.bits {
      false => TOFDIR_A::TOFDIR_0,
      true => TOFDIR_A::TOFDIR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TOFDIR_0`"]
  #[inline(always)]
  pub fn is_tofdir_0(&self) -> bool {
    *self == TOFDIR_A::TOFDIR_0
  }
  #[doc = "Checks if the value of the field is `TOFDIR_1`"]
  #[inline(always)]
  pub fn is_tofdir_1(&self) -> bool {
    *self == TOFDIR_A::TOFDIR_1
  }
}
#[doc = "Counter Direction in Quadrature Decode Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIR_A {
  #[doc = "0: Counter direction is decreasing (counter decrement)."]
  QUADIR_0,
  #[doc = "1: Counter direction is increasing (counter increment)."]
  QUADIR_1,
}
impl From<QUADIR_A> for bool {
  #[inline(always)]
  fn from(variant: QUADIR_A) -> Self {
    match variant {
      QUADIR_A::QUADIR_0 => false,
      QUADIR_A::QUADIR_1 => true,
    }
  }
}
#[doc = "Reader of field `QUADIR`"]
pub type QUADIR_R = crate::R<bool, QUADIR_A>;
impl QUADIR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> QUADIR_A {
    match self.bits {
      false => QUADIR_A::QUADIR_0,
      true => QUADIR_A::QUADIR_1,
    }
  }
  #[doc = "Checks if the value of the field is `QUADIR_0`"]
  #[inline(always)]
  pub fn is_quadir_0(&self) -> bool {
    *self == QUADIR_A::QUADIR_0
  }
  #[doc = "Checks if the value of the field is `QUADIR_1`"]
  #[inline(always)]
  pub fn is_quadir_1(&self) -> bool {
    *self == QUADIR_A::QUADIR_1
  }
}
#[doc = "Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADMODE_A {
  #[doc = "0: Phase encoding mode."]
  QUADMODE_0,
  #[doc = "1: Count and direction encoding mode."]
  QUADMODE_1,
}
impl From<QUADMODE_A> for bool {
  #[inline(always)]
  fn from(variant: QUADMODE_A) -> Self {
    match variant {
      QUADMODE_A::QUADMODE_0 => false,
      QUADMODE_A::QUADMODE_1 => true,
    }
  }
}
#[doc = "Reader of field `QUADMODE`"]
pub type QUADMODE_R = crate::R<bool, QUADMODE_A>;
impl QUADMODE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> QUADMODE_A {
    match self.bits {
      false => QUADMODE_A::QUADMODE_0,
      true => QUADMODE_A::QUADMODE_1,
    }
  }
  #[doc = "Checks if the value of the field is `QUADMODE_0`"]
  #[inline(always)]
  pub fn is_quadmode_0(&self) -> bool {
    *self == QUADMODE_A::QUADMODE_0
  }
  #[doc = "Checks if the value of the field is `QUADMODE_1`"]
  #[inline(always)]
  pub fn is_quadmode_1(&self) -> bool {
    *self == QUADMODE_A::QUADMODE_1
  }
}
#[doc = "Write proxy for field `QUADMODE`"]
pub struct QUADMODE_W<'a> {
  w: &'a mut W,
}
impl<'a> QUADMODE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: QUADMODE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Phase encoding mode."]
  #[inline(always)]
  pub fn quadmode_0(self) -> &'a mut W {
    self.variant(QUADMODE_A::QUADMODE_0)
  }
  #[doc = "Count and direction encoding mode."]
  #[inline(always)]
  pub fn quadmode_1(self) -> &'a mut W {
    self.variant(QUADMODE_A::QUADMODE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - QUADEN"]
  #[inline(always)]
  pub fn quaden(&self) -> QUADEN_R {
    QUADEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - TOFDIR"]
  #[inline(always)]
  pub fn tofdir(&self) -> TOFDIR_R {
    TOFDIR_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Counter Direction in Quadrature Decode Mode"]
  #[inline(always)]
  pub fn quadir(&self) -> QUADIR_R {
    QUADIR_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Quadrature Decoder Mode"]
  #[inline(always)]
  pub fn quadmode(&self) -> QUADMODE_R {
    QUADMODE_R::new(((self.bits >> 3) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - QUADEN"]
  #[inline(always)]
  pub fn quaden(&mut self) -> QUADEN_W {
    QUADEN_W { w: self }
  }
  #[doc = "Bit 3 - Quadrature Decoder Mode"]
  #[inline(always)]
  pub fn quadmode(&mut self) -> QUADMODE_W {
    QUADMODE_W { w: self }
  }
}
