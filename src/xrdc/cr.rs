#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::CR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0c
  }
}
#[doc = "Global Valid MDACs(XRDC global enable/disable).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GVLDM_A {
  #[doc = "0: XRDC MDACs are disabled."]
  GVLDM_0,
  #[doc = "1: XRDC MDACs are enabled."]
  GVLDM_1,
}
impl From<GVLDM_A> for bool {
  #[inline(always)]
  fn from(variant: GVLDM_A) -> Self {
    match variant {
      GVLDM_A::GVLDM_0 => false,
      GVLDM_A::GVLDM_1 => true,
    }
  }
}
#[doc = "Reader of field `GVLDM`"]
pub type GVLDM_R = crate::R<bool, GVLDM_A>;
impl GVLDM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GVLDM_A {
    match self.bits {
      false => GVLDM_A::GVLDM_0,
      true => GVLDM_A::GVLDM_1,
    }
  }
  #[doc = "Checks if the value of the field is `GVLDM_0`"]
  #[inline(always)]
  pub fn is_gvldm_0(&self) -> bool {
    *self == GVLDM_A::GVLDM_0
  }
  #[doc = "Checks if the value of the field is `GVLDM_1`"]
  #[inline(always)]
  pub fn is_gvldm_1(&self) -> bool {
    *self == GVLDM_A::GVLDM_1
  }
}
#[doc = "Write proxy for field `GVLDM`"]
pub struct GVLDM_W<'a> {
  w: &'a mut W,
}
impl<'a> GVLDM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GVLDM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "XRDC MDACs are disabled."]
  #[inline(always)]
  pub fn gvldm_0(self) -> &'a mut W {
    self.variant(GVLDM_A::GVLDM_0)
  }
  #[doc = "XRDC MDACs are enabled."]
  #[inline(always)]
  pub fn gvldm_1(self) -> &'a mut W {
    self.variant(GVLDM_A::GVLDM_1)
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
#[doc = "Reader of field `HRL`"]
pub type HRL_R = crate::R<u8, u8>;
#[doc = "Virtualization aware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VAW_A {
  #[doc = "0: Implementation is not virtualization aware."]
  VAW_0,
  #[doc = "1: Implementation is virtualization aware."]
  VAW_1,
}
impl From<VAW_A> for bool {
  #[inline(always)]
  fn from(variant: VAW_A) -> Self {
    match variant {
      VAW_A::VAW_0 => false,
      VAW_A::VAW_1 => true,
    }
  }
}
#[doc = "Reader of field `VAW`"]
pub type VAW_R = crate::R<bool, VAW_A>;
impl VAW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VAW_A {
    match self.bits {
      false => VAW_A::VAW_0,
      true => VAW_A::VAW_1,
    }
  }
  #[doc = "Checks if the value of the field is `VAW_0`"]
  #[inline(always)]
  pub fn is_vaw_0(&self) -> bool {
    *self == VAW_A::VAW_0
  }
  #[doc = "Checks if the value of the field is `VAW_1`"]
  #[inline(always)]
  pub fn is_vaw_1(&self) -> bool {
    *self == VAW_A::VAW_1
  }
}
#[doc = "Global Valid for PACs/MSCs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GVLDP_A {
  #[doc = "0: XRDC PACs/MSCs are disabled."]
  GVLDP_0,
  #[doc = "1: XRDC PACs/MSCs are enabled."]
  GVLDP_1,
}
impl From<GVLDP_A> for bool {
  #[inline(always)]
  fn from(variant: GVLDP_A) -> Self {
    match variant {
      GVLDP_A::GVLDP_0 => false,
      GVLDP_A::GVLDP_1 => true,
    }
  }
}
#[doc = "Reader of field `GVLDP`"]
pub type GVLDP_R = crate::R<bool, GVLDP_A>;
impl GVLDP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GVLDP_A {
    match self.bits {
      false => GVLDP_A::GVLDP_0,
      true => GVLDP_A::GVLDP_1,
    }
  }
  #[doc = "Checks if the value of the field is `GVLDP_0`"]
  #[inline(always)]
  pub fn is_gvldp_0(&self) -> bool {
    *self == GVLDP_A::GVLDP_0
  }
  #[doc = "Checks if the value of the field is `GVLDP_1`"]
  #[inline(always)]
  pub fn is_gvldp_1(&self) -> bool {
    *self == GVLDP_A::GVLDP_1
  }
}
#[doc = "Write proxy for field `GVLDP`"]
pub struct GVLDP_W<'a> {
  w: &'a mut W,
}
impl<'a> GVLDP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GVLDP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "XRDC PACs/MSCs are disabled."]
  #[inline(always)]
  pub fn gvldp_0(self) -> &'a mut W {
    self.variant(GVLDP_A::GVLDP_0)
  }
  #[doc = "XRDC PACs/MSCs are enabled."]
  #[inline(always)]
  pub fn gvldp_1(self) -> &'a mut W {
    self.variant(GVLDP_A::GVLDP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
    self.w
  }
}
#[doc = "Global Valid for MRCs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GVLDC_A {
  #[doc = "0: XRDC MRCs are disabled."]
  GVLDC_0,
  #[doc = "1: XRDC MRCs are enabled."]
  GVLDC_1,
}
impl From<GVLDC_A> for bool {
  #[inline(always)]
  fn from(variant: GVLDC_A) -> Self {
    match variant {
      GVLDC_A::GVLDC_0 => false,
      GVLDC_A::GVLDC_1 => true,
    }
  }
}
#[doc = "Reader of field `GVLDC`"]
pub type GVLDC_R = crate::R<bool, GVLDC_A>;
impl GVLDC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GVLDC_A {
    match self.bits {
      false => GVLDC_A::GVLDC_0,
      true => GVLDC_A::GVLDC_1,
    }
  }
  #[doc = "Checks if the value of the field is `GVLDC_0`"]
  #[inline(always)]
  pub fn is_gvldc_0(&self) -> bool {
    *self == GVLDC_A::GVLDC_0
  }
  #[doc = "Checks if the value of the field is `GVLDC_1`"]
  #[inline(always)]
  pub fn is_gvldc_1(&self) -> bool {
    *self == GVLDC_A::GVLDC_1
  }
}
#[doc = "Write proxy for field `GVLDC`"]
pub struct GVLDC_W<'a> {
  w: &'a mut W,
}
impl<'a> GVLDC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GVLDC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "XRDC MRCs are disabled."]
  #[inline(always)]
  pub fn gvldc_0(self) -> &'a mut W {
    self.variant(GVLDC_A::GVLDC_0)
  }
  #[doc = "XRDC MRCs are enabled."]
  #[inline(always)]
  pub fn gvldc_1(self) -> &'a mut W {
    self.variant(GVLDC_A::GVLDC_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
#[doc = "1-bit Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK1_A {
  #[doc = "0: Register can be written by any secure privileged write."]
  LK1_0,
  #[doc = "1: Register is locked (read-only) until the next reset."]
  LK1_1,
}
impl From<LK1_A> for bool {
  #[inline(always)]
  fn from(variant: LK1_A) -> Self {
    match variant {
      LK1_A::LK1_0 => false,
      LK1_A::LK1_1 => true,
    }
  }
}
#[doc = "Reader of field `LK1`"]
pub type LK1_R = crate::R<bool, LK1_A>;
impl LK1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK1_A {
    match self.bits {
      false => LK1_A::LK1_0,
      true => LK1_A::LK1_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK1_0`"]
  #[inline(always)]
  pub fn is_lk1_0(&self) -> bool {
    *self == LK1_A::LK1_0
  }
  #[doc = "Checks if the value of the field is `LK1_1`"]
  #[inline(always)]
  pub fn is_lk1_1(&self) -> bool {
    *self == LK1_A::LK1_1
  }
}
#[doc = "Write proxy for field `LK1`"]
pub struct LK1_W<'a> {
  w: &'a mut W,
}
impl<'a> LK1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Register can be written by any secure privileged write."]
  #[inline(always)]
  pub fn lk1_0(self) -> &'a mut W {
    self.variant(LK1_A::LK1_0)
  }
  #[doc = "Register is locked (read-only) until the next reset."]
  #[inline(always)]
  pub fn lk1_1(self) -> &'a mut W {
    self.variant(LK1_A::LK1_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Global Valid MDACs(XRDC global enable/disable)."]
  #[inline(always)]
  pub fn gvldm(&self) -> GVLDM_R {
    GVLDM_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bits 1:4 - Hardware Revision Level"]
  #[inline(always)]
  pub fn hrl(&self) -> HRL_R {
    HRL_R::new(((self.bits >> 1) & 0x0f) as u8)
  }
  #[doc = "Bit 8 - Virtualization aware"]
  #[inline(always)]
  pub fn vaw(&self) -> VAW_R {
    VAW_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Global Valid for PACs/MSCs"]
  #[inline(always)]
  pub fn gvldp(&self) -> GVLDP_R {
    GVLDP_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Global Valid for MRCs"]
  #[inline(always)]
  pub fn gvldc(&self) -> GVLDC_R {
    GVLDC_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 30 - 1-bit Lock"]
  #[inline(always)]
  pub fn lk1(&self) -> LK1_R {
    LK1_R::new(((self.bits >> 30) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Global Valid MDACs(XRDC global enable/disable)."]
  #[inline(always)]
  pub fn gvldm(&mut self) -> GVLDM_W {
    GVLDM_W { w: self }
  }
  #[doc = "Bit 14 - Global Valid for PACs/MSCs"]
  #[inline(always)]
  pub fn gvldp(&mut self) -> GVLDP_W {
    GVLDP_W { w: self }
  }
  #[doc = "Bit 15 - Global Valid for MRCs"]
  #[inline(always)]
  pub fn gvldc(&mut self) -> GVLDC_W {
    GVLDC_W { w: self }
  }
  #[doc = "Bit 30 - 1-bit Lock"]
  #[inline(always)]
  pub fn lk1(&mut self) -> LK1_W {
    LK1_W { w: self }
  }
}
