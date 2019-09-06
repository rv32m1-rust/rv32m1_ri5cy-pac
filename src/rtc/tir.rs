#[doc = "Reader of register TIR"]
pub type R = crate::R<u32, super::TIR>;
#[doc = "Writer for register TIR"]
pub type W = crate::W<u32, super::TIR>;
#[doc = "Register TIR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Loss of Clock Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCIE_A {
  #[doc = "0: Interupt disabled."]
  LCIE_0,
  #[doc = "1: An interrupt is generated when the loss of clock flag is set."]
  LCIE_1,
}
impl From<LCIE_A> for bool {
  #[inline(always)]
  fn from(variant: LCIE_A) -> Self {
    match variant {
      LCIE_A::LCIE_0 => false,
      LCIE_A::LCIE_1 => true,
    }
  }
}
#[doc = "Reader of field `LCIE`"]
pub type LCIE_R = crate::R<bool, LCIE_A>;
impl LCIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LCIE_A {
    match self.bits {
      false => LCIE_A::LCIE_0,
      true => LCIE_A::LCIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `LCIE_0`"]
  #[inline(always)]
  pub fn is_lcie_0(&self) -> bool {
    *self == LCIE_A::LCIE_0
  }
  #[doc = "Checks if the value of the field is `LCIE_1`"]
  #[inline(always)]
  pub fn is_lcie_1(&self) -> bool {
    *self == LCIE_A::LCIE_1
  }
}
#[doc = "Write proxy for field `LCIE`"]
pub struct LCIE_W<'a> {
  w: &'a mut W,
}
impl<'a> LCIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LCIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Interupt disabled."]
  #[inline(always)]
  pub fn lcie_0(self) -> &'a mut W {
    self.variant(LCIE_A::LCIE_0)
  }
  #[doc = "An interrupt is generated when the loss of clock flag is set."]
  #[inline(always)]
  pub fn lcie_1(self) -> &'a mut W {
    self.variant(LCIE_A::LCIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "Security Module Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIE_A {
  #[doc = "0: Interupt disabled."]
  SIE_0,
  #[doc = "1: An interrupt is generated when the security module flag is set."]
  SIE_1,
}
impl From<SIE_A> for bool {
  #[inline(always)]
  fn from(variant: SIE_A) -> Self {
    match variant {
      SIE_A::SIE_0 => false,
      SIE_A::SIE_1 => true,
    }
  }
}
#[doc = "Reader of field `SIE`"]
pub type SIE_R = crate::R<bool, SIE_A>;
impl SIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIE_A {
    match self.bits {
      false => SIE_A::SIE_0,
      true => SIE_A::SIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIE_0`"]
  #[inline(always)]
  pub fn is_sie_0(&self) -> bool {
    *self == SIE_A::SIE_0
  }
  #[doc = "Checks if the value of the field is `SIE_1`"]
  #[inline(always)]
  pub fn is_sie_1(&self) -> bool {
    *self == SIE_A::SIE_1
  }
}
#[doc = "Write proxy for field `SIE`"]
pub struct SIE_W<'a> {
  w: &'a mut W,
}
impl<'a> SIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Interupt disabled."]
  #[inline(always)]
  pub fn sie_0(self) -> &'a mut W {
    self.variant(SIE_A::SIE_0)
  }
  #[doc = "An interrupt is generated when the security module flag is set."]
  #[inline(always)]
  pub fn sie_1(self) -> &'a mut W {
    self.variant(SIE_A::SIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "Flash Security Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSIE_A {
  #[doc = "0: Interupt disabled."]
  FSIE_0,
  #[doc = "1: An interrupt is generated when the flash security flag is set."]
  FSIE_1,
}
impl From<FSIE_A> for bool {
  #[inline(always)]
  fn from(variant: FSIE_A) -> Self {
    match variant {
      FSIE_A::FSIE_0 => false,
      FSIE_A::FSIE_1 => true,
    }
  }
}
#[doc = "Reader of field `FSIE`"]
pub type FSIE_R = crate::R<bool, FSIE_A>;
impl FSIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FSIE_A {
    match self.bits {
      false => FSIE_A::FSIE_0,
      true => FSIE_A::FSIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FSIE_0`"]
  #[inline(always)]
  pub fn is_fsie_0(&self) -> bool {
    *self == FSIE_A::FSIE_0
  }
  #[doc = "Checks if the value of the field is `FSIE_1`"]
  #[inline(always)]
  pub fn is_fsie_1(&self) -> bool {
    *self == FSIE_A::FSIE_1
  }
}
#[doc = "Write proxy for field `FSIE`"]
pub struct FSIE_W<'a> {
  w: &'a mut W,
}
impl<'a> FSIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FSIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Interupt disabled."]
  #[inline(always)]
  pub fn fsie_0(self) -> &'a mut W {
    self.variant(FSIE_A::FSIE_0)
  }
  #[doc = "An interrupt is generated when the flash security flag is set."]
  #[inline(always)]
  pub fn fsie_1(self) -> &'a mut W {
    self.variant(FSIE_A::FSIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "Test Mode Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMIE_A {
  #[doc = "0: Interupt disabled."]
  TMIE_0,
  #[doc = "1: An interrupt is generated when the test mode flag is set."]
  TMIE_1,
}
impl From<TMIE_A> for bool {
  #[inline(always)]
  fn from(variant: TMIE_A) -> Self {
    match variant {
      TMIE_A::TMIE_0 => false,
      TMIE_A::TMIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TMIE`"]
pub type TMIE_R = crate::R<bool, TMIE_A>;
impl TMIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TMIE_A {
    match self.bits {
      false => TMIE_A::TMIE_0,
      true => TMIE_A::TMIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TMIE_0`"]
  #[inline(always)]
  pub fn is_tmie_0(&self) -> bool {
    *self == TMIE_A::TMIE_0
  }
  #[doc = "Checks if the value of the field is `TMIE_1`"]
  #[inline(always)]
  pub fn is_tmie_1(&self) -> bool {
    *self == TMIE_A::TMIE_1
  }
}
#[doc = "Write proxy for field `TMIE`"]
pub struct TMIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TMIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TMIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Interupt disabled."]
  #[inline(always)]
  pub fn tmie_0(self) -> &'a mut W {
    self.variant(TMIE_A::TMIE_0)
  }
  #[doc = "An interrupt is generated when the test mode flag is set."]
  #[inline(always)]
  pub fn tmie_1(self) -> &'a mut W {
    self.variant(TMIE_A::TMIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
#[doc = "Reader of field `TPIE`"]
pub type TPIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPIE`"]
pub struct TPIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TPIE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 4 - Loss of Clock Interrupt Enable"]
  #[inline(always)]
  pub fn lcie(&self) -> LCIE_R {
    LCIE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Security Module Interrupt Enable"]
  #[inline(always)]
  pub fn sie(&self) -> SIE_R {
    SIE_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Flash Security Interrupt Enable"]
  #[inline(always)]
  pub fn fsie(&self) -> FSIE_R {
    FSIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Test Mode Interrupt Enable"]
  #[inline(always)]
  pub fn tmie(&self) -> TMIE_R {
    TMIE_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - Tamper Pin Interrupt Enable"]
  #[inline(always)]
  pub fn tpie(&self) -> TPIE_R {
    TPIE_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 4 - Loss of Clock Interrupt Enable"]
  #[inline(always)]
  pub fn lcie(&mut self) -> LCIE_W {
    LCIE_W { w: self }
  }
  #[doc = "Bit 5 - Security Module Interrupt Enable"]
  #[inline(always)]
  pub fn sie(&mut self) -> SIE_W {
    SIE_W { w: self }
  }
  #[doc = "Bit 6 - Flash Security Interrupt Enable"]
  #[inline(always)]
  pub fn fsie(&mut self) -> FSIE_W {
    FSIE_W { w: self }
  }
  #[doc = "Bit 7 - Test Mode Interrupt Enable"]
  #[inline(always)]
  pub fn tmie(&mut self) -> TMIE_W {
    TMIE_W { w: self }
  }
  #[doc = "Bits 16:19 - Tamper Pin Interrupt Enable"]
  #[inline(always)]
  pub fn tpie(&mut self) -> TPIE_W {
    TPIE_W { w: self }
  }
}
