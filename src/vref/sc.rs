#[doc = "Reader of register SC"]
pub type R = crate::R<u8, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u8, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Buffer Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_LV_A {
  #[doc = "0: Bandgap on only, for stabilization and startup"]
  MODE_LV_0,
  #[doc = "1: High power buffer mode enabled"]
  MODE_LV_1,
  #[doc = "2: no description available"]
  MODE_LV_2,
}
impl From<MODE_LV_A> for u8 {
  #[inline(always)]
  fn from(variant: MODE_LV_A) -> Self {
    match variant {
      MODE_LV_A::MODE_LV_0 => 0,
      MODE_LV_A::MODE_LV_1 => 1,
      MODE_LV_A::MODE_LV_2 => 2,
    }
  }
}
#[doc = "Reader of field `MODE_LV`"]
pub type MODE_LV_R = crate::R<u8, MODE_LV_A>;
impl MODE_LV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, MODE_LV_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(MODE_LV_A::MODE_LV_0),
      1 => Val(MODE_LV_A::MODE_LV_1),
      2 => Val(MODE_LV_A::MODE_LV_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `MODE_LV_0`"]
  #[inline(always)]
  pub fn is_mode_lv_0(&self) -> bool {
    *self == MODE_LV_A::MODE_LV_0
  }
  #[doc = "Checks if the value of the field is `MODE_LV_1`"]
  #[inline(always)]
  pub fn is_mode_lv_1(&self) -> bool {
    *self == MODE_LV_A::MODE_LV_1
  }
  #[doc = "Checks if the value of the field is `MODE_LV_2`"]
  #[inline(always)]
  pub fn is_mode_lv_2(&self) -> bool {
    *self == MODE_LV_A::MODE_LV_2
  }
}
#[doc = "Write proxy for field `MODE_LV`"]
pub struct MODE_LV_W<'a> {
  w: &'a mut W,
}
impl<'a> MODE_LV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MODE_LV_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Bandgap on only, for stabilization and startup"]
  #[inline(always)]
  pub fn mode_lv_0(self) -> &'a mut W {
    self.variant(MODE_LV_A::MODE_LV_0)
  }
  #[doc = "High power buffer mode enabled"]
  #[inline(always)]
  pub fn mode_lv_1(self) -> &'a mut W {
    self.variant(MODE_LV_A::MODE_LV_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn mode_lv_2(self) -> &'a mut W {
    self.variant(MODE_LV_A::MODE_LV_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
    self.w
  }
}
#[doc = "Internal Voltage Reference stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFST_A {
  #[doc = "0: The module is disabled or not stable."]
  VREFST_0,
  #[doc = "1: The module is stable."]
  VREFST_1,
}
impl From<VREFST_A> for bool {
  #[inline(always)]
  fn from(variant: VREFST_A) -> Self {
    match variant {
      VREFST_A::VREFST_0 => false,
      VREFST_A::VREFST_1 => true,
    }
  }
}
#[doc = "Reader of field `VREFST`"]
pub type VREFST_R = crate::R<bool, VREFST_A>;
impl VREFST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VREFST_A {
    match self.bits {
      false => VREFST_A::VREFST_0,
      true => VREFST_A::VREFST_1,
    }
  }
  #[doc = "Checks if the value of the field is `VREFST_0`"]
  #[inline(always)]
  pub fn is_vrefst_0(&self) -> bool {
    *self == VREFST_A::VREFST_0
  }
  #[doc = "Checks if the value of the field is `VREFST_1`"]
  #[inline(always)]
  pub fn is_vrefst_1(&self) -> bool {
    *self == VREFST_A::VREFST_1
  }
}
#[doc = "Second order curvature compensation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICOMPEN_A {
  #[doc = "0: Disabled"]
  ICOMPEN_0,
  #[doc = "1: Enabled"]
  ICOMPEN_1,
}
impl From<ICOMPEN_A> for bool {
  #[inline(always)]
  fn from(variant: ICOMPEN_A) -> Self {
    match variant {
      ICOMPEN_A::ICOMPEN_0 => false,
      ICOMPEN_A::ICOMPEN_1 => true,
    }
  }
}
#[doc = "Reader of field `ICOMPEN`"]
pub type ICOMPEN_R = crate::R<bool, ICOMPEN_A>;
impl ICOMPEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ICOMPEN_A {
    match self.bits {
      false => ICOMPEN_A::ICOMPEN_0,
      true => ICOMPEN_A::ICOMPEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `ICOMPEN_0`"]
  #[inline(always)]
  pub fn is_icompen_0(&self) -> bool {
    *self == ICOMPEN_A::ICOMPEN_0
  }
  #[doc = "Checks if the value of the field is `ICOMPEN_1`"]
  #[inline(always)]
  pub fn is_icompen_1(&self) -> bool {
    *self == ICOMPEN_A::ICOMPEN_1
  }
}
#[doc = "Write proxy for field `ICOMPEN`"]
pub struct ICOMPEN_W<'a> {
  w: &'a mut W,
}
impl<'a> ICOMPEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ICOMPEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disabled"]
  #[inline(always)]
  pub fn icompen_0(self) -> &'a mut W {
    self.variant(ICOMPEN_A::ICOMPEN_0)
  }
  #[doc = "Enabled"]
  #[inline(always)]
  pub fn icompen_1(self) -> &'a mut W {
    self.variant(ICOMPEN_A::ICOMPEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
    self.w
  }
}
#[doc = "Regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGEN_A {
  #[doc = "0: Internal 1.75 V regulator is disabled."]
  REGEN_0,
  #[doc = "1: Internal 1.75 V regulator is enabled."]
  REGEN_1,
}
impl From<REGEN_A> for bool {
  #[inline(always)]
  fn from(variant: REGEN_A) -> Self {
    match variant {
      REGEN_A::REGEN_0 => false,
      REGEN_A::REGEN_1 => true,
    }
  }
}
#[doc = "Reader of field `REGEN`"]
pub type REGEN_R = crate::R<bool, REGEN_A>;
impl REGEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> REGEN_A {
    match self.bits {
      false => REGEN_A::REGEN_0,
      true => REGEN_A::REGEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `REGEN_0`"]
  #[inline(always)]
  pub fn is_regen_0(&self) -> bool {
    *self == REGEN_A::REGEN_0
  }
  #[doc = "Checks if the value of the field is `REGEN_1`"]
  #[inline(always)]
  pub fn is_regen_1(&self) -> bool {
    *self == REGEN_A::REGEN_1
  }
}
#[doc = "Write proxy for field `REGEN`"]
pub struct REGEN_W<'a> {
  w: &'a mut W,
}
impl<'a> REGEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: REGEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal 1.75 V regulator is disabled."]
  #[inline(always)]
  pub fn regen_0(self) -> &'a mut W {
    self.variant(REGEN_A::REGEN_0)
  }
  #[doc = "Internal 1.75 V regulator is enabled."]
  #[inline(always)]
  pub fn regen_1(self) -> &'a mut W {
    self.variant(REGEN_A::REGEN_1)
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
#[doc = "Internal Voltage Reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
  #[doc = "0: The module is disabled."]
  VREFEN_0,
  #[doc = "1: The module is enabled."]
  VREFEN_1,
}
impl From<VREFEN_A> for bool {
  #[inline(always)]
  fn from(variant: VREFEN_A) -> Self {
    match variant {
      VREFEN_A::VREFEN_0 => false,
      VREFEN_A::VREFEN_1 => true,
    }
  }
}
#[doc = "Reader of field `VREFEN`"]
pub type VREFEN_R = crate::R<bool, VREFEN_A>;
impl VREFEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VREFEN_A {
    match self.bits {
      false => VREFEN_A::VREFEN_0,
      true => VREFEN_A::VREFEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `VREFEN_0`"]
  #[inline(always)]
  pub fn is_vrefen_0(&self) -> bool {
    *self == VREFEN_A::VREFEN_0
  }
  #[doc = "Checks if the value of the field is `VREFEN_1`"]
  #[inline(always)]
  pub fn is_vrefen_1(&self) -> bool {
    *self == VREFEN_A::VREFEN_1
  }
}
#[doc = "Write proxy for field `VREFEN`"]
pub struct VREFEN_W<'a> {
  w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The module is disabled."]
  #[inline(always)]
  pub fn vrefen_0(self) -> &'a mut W {
    self.variant(VREFEN_A::VREFEN_0)
  }
  #[doc = "The module is enabled."]
  #[inline(always)]
  pub fn vrefen_1(self) -> &'a mut W {
    self.variant(VREFEN_A::VREFEN_1)
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
  #[doc = "Bits 0:1 - Buffer Mode selection"]
  #[inline(always)]
  pub fn mode_lv(&self) -> MODE_LV_R {
    MODE_LV_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bit 2 - Internal Voltage Reference stable"]
  #[inline(always)]
  pub fn vrefst(&self) -> VREFST_R {
    VREFST_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Second order curvature compensation enable"]
  #[inline(always)]
  pub fn icompen(&self) -> ICOMPEN_R {
    ICOMPEN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Regulator enable"]
  #[inline(always)]
  pub fn regen(&self) -> REGEN_R {
    REGEN_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Internal Voltage Reference enable"]
  #[inline(always)]
  pub fn vrefen(&self) -> VREFEN_R {
    VREFEN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:1 - Buffer Mode selection"]
  #[inline(always)]
  pub fn mode_lv(&mut self) -> MODE_LV_W {
    MODE_LV_W { w: self }
  }
  #[doc = "Bit 5 - Second order curvature compensation enable"]
  #[inline(always)]
  pub fn icompen(&mut self) -> ICOMPEN_W {
    ICOMPEN_W { w: self }
  }
  #[doc = "Bit 6 - Regulator enable"]
  #[inline(always)]
  pub fn regen(&mut self) -> REGEN_W {
    REGEN_W { w: self }
  }
  #[doc = "Bit 7 - Internal Voltage Reference enable"]
  #[inline(always)]
  pub fn vrefen(&mut self) -> VREFEN_W {
    VREFEN_W { w: self }
  }
}
