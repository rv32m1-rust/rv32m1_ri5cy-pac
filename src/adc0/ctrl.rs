#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "ADC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
  #[doc = "0: ADC is disabled."]
  ADCEN_0,
  #[doc = "1: ADC is enabled."]
  ADCEN_1,
}
impl From<ADCEN_A> for bool {
  #[inline(always)]
  fn from(variant: ADCEN_A) -> Self {
    match variant {
      ADCEN_A::ADCEN_0 => false,
      ADCEN_A::ADCEN_1 => true,
    }
  }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, ADCEN_A>;
impl ADCEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ADCEN_A {
    match self.bits {
      false => ADCEN_A::ADCEN_0,
      true => ADCEN_A::ADCEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `ADCEN_0`"]
  #[inline(always)]
  pub fn is_adcen_0(&self) -> bool {
    *self == ADCEN_A::ADCEN_0
  }
  #[doc = "Checks if the value of the field is `ADCEN_1`"]
  #[inline(always)]
  pub fn is_adcen_1(&self) -> bool {
    *self == ADCEN_A::ADCEN_1
  }
}
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
  w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "ADC is disabled."]
  #[inline(always)]
  pub fn adcen_0(self) -> &'a mut W {
    self.variant(ADCEN_A::ADCEN_0)
  }
  #[doc = "ADC is enabled."]
  #[inline(always)]
  pub fn adcen_1(self) -> &'a mut W {
    self.variant(ADCEN_A::ADCEN_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
  #[doc = "0: ADC logic is not reset."]
  RST_0,
  #[doc = "1: ADC logic is reset."]
  RST_1,
}
impl From<RST_A> for bool {
  #[inline(always)]
  fn from(variant: RST_A) -> Self {
    match variant {
      RST_A::RST_0 => false,
      RST_A::RST_1 => true,
    }
  }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, RST_A>;
impl RST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RST_A {
    match self.bits {
      false => RST_A::RST_0,
      true => RST_A::RST_1,
    }
  }
  #[doc = "Checks if the value of the field is `RST_0`"]
  #[inline(always)]
  pub fn is_rst_0(&self) -> bool {
    *self == RST_A::RST_0
  }
  #[doc = "Checks if the value of the field is `RST_1`"]
  #[inline(always)]
  pub fn is_rst_1(&self) -> bool {
    *self == RST_A::RST_1
  }
}
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
  w: &'a mut W,
}
impl<'a> RST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RST_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "ADC logic is not reset."]
  #[inline(always)]
  pub fn rst_0(self) -> &'a mut W {
    self.variant(RST_A::RST_0)
  }
  #[doc = "ADC logic is reset."]
  #[inline(always)]
  pub fn rst_1(self) -> &'a mut W {
    self.variant(RST_A::RST_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEN_A {
  #[doc = "0: ADC is enabled in Doze mode."]
  DOZEN_0,
  #[doc = "1: ADC is disabled in Doze mode."]
  DOZEN_1,
}
impl From<DOZEN_A> for bool {
  #[inline(always)]
  fn from(variant: DOZEN_A) -> Self {
    match variant {
      DOZEN_A::DOZEN_0 => false,
      DOZEN_A::DOZEN_1 => true,
    }
  }
}
#[doc = "Reader of field `DOZEN`"]
pub type DOZEN_R = crate::R<bool, DOZEN_A>;
impl DOZEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DOZEN_A {
    match self.bits {
      false => DOZEN_A::DOZEN_0,
      true => DOZEN_A::DOZEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DOZEN_0`"]
  #[inline(always)]
  pub fn is_dozen_0(&self) -> bool {
    *self == DOZEN_A::DOZEN_0
  }
  #[doc = "Checks if the value of the field is `DOZEN_1`"]
  #[inline(always)]
  pub fn is_dozen_1(&self) -> bool {
    *self == DOZEN_A::DOZEN_1
  }
}
#[doc = "Write proxy for field `DOZEN`"]
pub struct DOZEN_W<'a> {
  w: &'a mut W,
}
impl<'a> DOZEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DOZEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "ADC is enabled in Doze mode."]
  #[inline(always)]
  pub fn dozen_0(self) -> &'a mut W {
    self.variant(DOZEN_A::DOZEN_0)
  }
  #[doc = "ADC is disabled in Doze mode."]
  #[inline(always)]
  pub fn dozen_1(self) -> &'a mut W {
    self.variant(DOZEN_A::DOZEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Reset FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFIFO_AW {
  #[doc = "0: No effect."]
  RSTFIFO_0,
  #[doc = "1: FIFO is reset."]
  RSTFIFO_1,
}
impl From<RSTFIFO_AW> for bool {
  #[inline(always)]
  fn from(variant: RSTFIFO_AW) -> Self {
    match variant {
      RSTFIFO_AW::RSTFIFO_0 => false,
      RSTFIFO_AW::RSTFIFO_1 => true,
    }
  }
}
#[doc = "Write proxy for field `RSTFIFO`"]
pub struct RSTFIFO_W<'a> {
  w: &'a mut W,
}
impl<'a> RSTFIFO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSTFIFO_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn rstfifo_0(self) -> &'a mut W {
    self.variant(RSTFIFO_AW::RSTFIFO_0)
  }
  #[doc = "FIFO is reset."]
  #[inline(always)]
  pub fn rstfifo_1(self) -> &'a mut W {
    self.variant(RSTFIFO_AW::RSTFIFO_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - ADC Enable"]
  #[inline(always)]
  pub fn adcen(&self) -> ADCEN_R {
    ADCEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Software Reset"]
  #[inline(always)]
  pub fn rst(&self) -> RST_R {
    RST_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Doze Enable"]
  #[inline(always)]
  pub fn dozen(&self) -> DOZEN_R {
    DOZEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - ADC Enable"]
  #[inline(always)]
  pub fn adcen(&mut self) -> ADCEN_W {
    ADCEN_W { w: self }
  }
  #[doc = "Bit 1 - Software Reset"]
  #[inline(always)]
  pub fn rst(&mut self) -> RST_W {
    RST_W { w: self }
  }
  #[doc = "Bit 2 - Doze Enable"]
  #[inline(always)]
  pub fn dozen(&mut self) -> DOZEN_W {
    DOZEN_W { w: self }
  }
  #[doc = "Bit 8 - Reset FIFO"]
  #[inline(always)]
  pub fn rstfifo(&mut self) -> RSTFIFO_W {
    RSTFIFO_W { w: self }
  }
}
