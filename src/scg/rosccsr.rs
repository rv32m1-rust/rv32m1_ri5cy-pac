#[doc = "Reader of register ROSCCSR"]
pub type R = crate::R<u32, super::ROSCCSR>;
#[doc = "Writer for register ROSCCSR"]
pub type W = crate::W<u32, super::ROSCCSR>;
#[doc = "Register ROSCCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ROSCCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "RTC OSC Clock Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROSCCM_A {
  #[doc = "0: RTC OSC Clock Monitor is disabled"]
  ROSCCM_0,
  #[doc = "1: RTC OSC Clock Monitor is enabled"]
  ROSCCM_1,
}
impl From<ROSCCM_A> for bool {
  #[inline(always)]
  fn from(variant: ROSCCM_A) -> Self {
    match variant {
      ROSCCM_A::ROSCCM_0 => false,
      ROSCCM_A::ROSCCM_1 => true,
    }
  }
}
#[doc = "Reader of field `ROSCCM`"]
pub type ROSCCM_R = crate::R<bool, ROSCCM_A>;
impl ROSCCM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ROSCCM_A {
    match self.bits {
      false => ROSCCM_A::ROSCCM_0,
      true => ROSCCM_A::ROSCCM_1,
    }
  }
  #[doc = "Checks if the value of the field is `ROSCCM_0`"]
  #[inline(always)]
  pub fn is_rosccm_0(&self) -> bool {
    *self == ROSCCM_A::ROSCCM_0
  }
  #[doc = "Checks if the value of the field is `ROSCCM_1`"]
  #[inline(always)]
  pub fn is_rosccm_1(&self) -> bool {
    *self == ROSCCM_A::ROSCCM_1
  }
}
#[doc = "Write proxy for field `ROSCCM`"]
pub struct ROSCCM_W<'a> {
  w: &'a mut W,
}
impl<'a> ROSCCM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ROSCCM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC OSC Clock Monitor is disabled"]
  #[inline(always)]
  pub fn rosccm_0(self) -> &'a mut W {
    self.variant(ROSCCM_A::ROSCCM_0)
  }
  #[doc = "RTC OSC Clock Monitor is enabled"]
  #[inline(always)]
  pub fn rosccm_1(self) -> &'a mut W {
    self.variant(ROSCCM_A::ROSCCM_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "RTC OSC Clock Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROSCCMRE_A {
  #[doc = "0: Clock Monitor generates interrupt when error detected"]
  ROSCCMRE_0,
  #[doc = "1: Clock Monitor generates reset when error detected"]
  ROSCCMRE_1,
}
impl From<ROSCCMRE_A> for bool {
  #[inline(always)]
  fn from(variant: ROSCCMRE_A) -> Self {
    match variant {
      ROSCCMRE_A::ROSCCMRE_0 => false,
      ROSCCMRE_A::ROSCCMRE_1 => true,
    }
  }
}
#[doc = "Reader of field `ROSCCMRE`"]
pub type ROSCCMRE_R = crate::R<bool, ROSCCMRE_A>;
impl ROSCCMRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ROSCCMRE_A {
    match self.bits {
      false => ROSCCMRE_A::ROSCCMRE_0,
      true => ROSCCMRE_A::ROSCCMRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `ROSCCMRE_0`"]
  #[inline(always)]
  pub fn is_rosccmre_0(&self) -> bool {
    *self == ROSCCMRE_A::ROSCCMRE_0
  }
  #[doc = "Checks if the value of the field is `ROSCCMRE_1`"]
  #[inline(always)]
  pub fn is_rosccmre_1(&self) -> bool {
    *self == ROSCCMRE_A::ROSCCMRE_1
  }
}
#[doc = "Write proxy for field `ROSCCMRE`"]
pub struct ROSCCMRE_W<'a> {
  w: &'a mut W,
}
impl<'a> ROSCCMRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ROSCCMRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock Monitor generates interrupt when error detected"]
  #[inline(always)]
  pub fn rosccmre_0(self) -> &'a mut W {
    self.variant(ROSCCMRE_A::ROSCCMRE_0)
  }
  #[doc = "Clock Monitor generates reset when error detected"]
  #[inline(always)]
  pub fn rosccmre_1(self) -> &'a mut W {
    self.variant(ROSCCMRE_A::ROSCCMRE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: Control Status Register can be written."]
  LK_0,
  #[doc = "1: Control Status Register cannot be written."]
  LK_1,
}
impl From<LK_A> for bool {
  #[inline(always)]
  fn from(variant: LK_A) -> Self {
    match variant {
      LK_A::LK_0 => false,
      LK_A::LK_1 => true,
    }
  }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK_A {
    match self.bits {
      false => LK_A::LK_0,
      true => LK_A::LK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK_0`"]
  #[inline(always)]
  pub fn is_lk_0(&self) -> bool {
    *self == LK_A::LK_0
  }
  #[doc = "Checks if the value of the field is `LK_1`"]
  #[inline(always)]
  pub fn is_lk_1(&self) -> bool {
    *self == LK_A::LK_1
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Control Status Register can be written."]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_A::LK_0)
  }
  #[doc = "Control Status Register cannot be written."]
  #[inline(always)]
  pub fn lk_1(self) -> &'a mut W {
    self.variant(LK_A::LK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "RTC OSC Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROSCVLD_A {
  #[doc = "0: RTC OSC is not enabled or clock is not valid"]
  ROSCVLD_0,
  #[doc = "1: RTC OSC is enabled and output clock is valid"]
  ROSCVLD_1,
}
impl From<ROSCVLD_A> for bool {
  #[inline(always)]
  fn from(variant: ROSCVLD_A) -> Self {
    match variant {
      ROSCVLD_A::ROSCVLD_0 => false,
      ROSCVLD_A::ROSCVLD_1 => true,
    }
  }
}
#[doc = "Reader of field `ROSCVLD`"]
pub type ROSCVLD_R = crate::R<bool, ROSCVLD_A>;
impl ROSCVLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ROSCVLD_A {
    match self.bits {
      false => ROSCVLD_A::ROSCVLD_0,
      true => ROSCVLD_A::ROSCVLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `ROSCVLD_0`"]
  #[inline(always)]
  pub fn is_roscvld_0(&self) -> bool {
    *self == ROSCVLD_A::ROSCVLD_0
  }
  #[doc = "Checks if the value of the field is `ROSCVLD_1`"]
  #[inline(always)]
  pub fn is_roscvld_1(&self) -> bool {
    *self == ROSCVLD_A::ROSCVLD_1
  }
}
#[doc = "RTC OSC Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROSCSEL_A {
  #[doc = "0: RTC OSC is not the system clock source"]
  ROSCSEL_0,
  #[doc = "1: RTC OSC is the system clock source"]
  ROSCSEL_1,
}
impl From<ROSCSEL_A> for bool {
  #[inline(always)]
  fn from(variant: ROSCSEL_A) -> Self {
    match variant {
      ROSCSEL_A::ROSCSEL_0 => false,
      ROSCSEL_A::ROSCSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `ROSCSEL`"]
pub type ROSCSEL_R = crate::R<bool, ROSCSEL_A>;
impl ROSCSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ROSCSEL_A {
    match self.bits {
      false => ROSCSEL_A::ROSCSEL_0,
      true => ROSCSEL_A::ROSCSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `ROSCSEL_0`"]
  #[inline(always)]
  pub fn is_roscsel_0(&self) -> bool {
    *self == ROSCSEL_A::ROSCSEL_0
  }
  #[doc = "Checks if the value of the field is `ROSCSEL_1`"]
  #[inline(always)]
  pub fn is_roscsel_1(&self) -> bool {
    *self == ROSCSEL_A::ROSCSEL_1
  }
}
#[doc = "RTC OSC Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROSCERR_A {
  #[doc = "0: RTC OSC Clock Monitor is disabled or has not detected an error"]
  ROSCERR_0,
  #[doc = "1: RTC OSC Clock Monitor is enabled and detected an RTC loss of clock error"]
  ROSCERR_1,
}
impl From<ROSCERR_A> for bool {
  #[inline(always)]
  fn from(variant: ROSCERR_A) -> Self {
    match variant {
      ROSCERR_A::ROSCERR_0 => false,
      ROSCERR_A::ROSCERR_1 => true,
    }
  }
}
#[doc = "Reader of field `ROSCERR`"]
pub type ROSCERR_R = crate::R<bool, ROSCERR_A>;
impl ROSCERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ROSCERR_A {
    match self.bits {
      false => ROSCERR_A::ROSCERR_0,
      true => ROSCERR_A::ROSCERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `ROSCERR_0`"]
  #[inline(always)]
  pub fn is_roscerr_0(&self) -> bool {
    *self == ROSCERR_A::ROSCERR_0
  }
  #[doc = "Checks if the value of the field is `ROSCERR_1`"]
  #[inline(always)]
  pub fn is_roscerr_1(&self) -> bool {
    *self == ROSCERR_A::ROSCERR_1
  }
}
#[doc = "Write proxy for field `ROSCERR`"]
pub struct ROSCERR_W<'a> {
  w: &'a mut W,
}
impl<'a> ROSCERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ROSCERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC OSC Clock Monitor is disabled or has not detected an error"]
  #[inline(always)]
  pub fn roscerr_0(self) -> &'a mut W {
    self.variant(ROSCERR_A::ROSCERR_0)
  }
  #[doc = "RTC OSC Clock Monitor is enabled and detected an RTC loss of clock error"]
  #[inline(always)]
  pub fn roscerr_1(self) -> &'a mut W {
    self.variant(ROSCERR_A::ROSCERR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
    self.w
  }
}
impl R {
  #[doc = "Bit 16 - RTC OSC Clock Monitor"]
  #[inline(always)]
  pub fn rosccm(&self) -> ROSCCM_R {
    ROSCCM_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - RTC OSC Clock Monitor Reset Enable"]
  #[inline(always)]
  pub fn rosccmre(&self) -> ROSCCMRE_R {
    ROSCCMRE_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - RTC OSC Valid"]
  #[inline(always)]
  pub fn roscvld(&self) -> ROSCVLD_R {
    ROSCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - RTC OSC Selected"]
  #[inline(always)]
  pub fn roscsel(&self) -> ROSCSEL_R {
    ROSCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - RTC OSC Clock Error"]
  #[inline(always)]
  pub fn roscerr(&self) -> ROSCERR_R {
    ROSCERR_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 16 - RTC OSC Clock Monitor"]
  #[inline(always)]
  pub fn rosccm(&mut self) -> ROSCCM_W {
    ROSCCM_W { w: self }
  }
  #[doc = "Bit 17 - RTC OSC Clock Monitor Reset Enable"]
  #[inline(always)]
  pub fn rosccmre(&mut self) -> ROSCCMRE_W {
    ROSCCMRE_W { w: self }
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
  #[doc = "Bit 26 - RTC OSC Clock Error"]
  #[inline(always)]
  pub fn roscerr(&mut self) -> ROSCERR_W {
    ROSCERR_W { w: self }
  }
}
