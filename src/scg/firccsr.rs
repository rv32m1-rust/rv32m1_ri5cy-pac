#[doc = "Reader of register FIRCCSR"]
pub type R = crate::R<u32, super::FIRCCSR>;
#[doc = "Writer for register FIRCCSR"]
pub type W = crate::W<u32, super::FIRCCSR>;
#[doc = "Register FIRCCSR `reset()`'s with value 0x0300_0001"]
impl crate::ResetValue for super::FIRCCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0300_0001
  }
}
#[doc = "Fast IRC Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCEN_A {
  #[doc = "0: Fast IRC is disabled"]
  FIRCEN_0,
  #[doc = "1: Fast IRC is enabled"]
  FIRCEN_1,
}
impl From<FIRCEN_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCEN_A) -> Self {
    match variant {
      FIRCEN_A::FIRCEN_0 => false,
      FIRCEN_A::FIRCEN_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCEN`"]
pub type FIRCEN_R = crate::R<bool, FIRCEN_A>;
impl FIRCEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCEN_A {
    match self.bits {
      false => FIRCEN_A::FIRCEN_0,
      true => FIRCEN_A::FIRCEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCEN_0`"]
  #[inline(always)]
  pub fn is_fircen_0(&self) -> bool {
    *self == FIRCEN_A::FIRCEN_0
  }
  #[doc = "Checks if the value of the field is `FIRCEN_1`"]
  #[inline(always)]
  pub fn is_fircen_1(&self) -> bool {
    *self == FIRCEN_A::FIRCEN_1
  }
}
#[doc = "Write proxy for field `FIRCEN`"]
pub struct FIRCEN_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fast IRC is disabled"]
  #[inline(always)]
  pub fn fircen_0(self) -> &'a mut W {
    self.variant(FIRCEN_A::FIRCEN_0)
  }
  #[doc = "Fast IRC is enabled"]
  #[inline(always)]
  pub fn fircen_1(self) -> &'a mut W {
    self.variant(FIRCEN_A::FIRCEN_1)
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
#[doc = "Fast IRC Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCSTEN_A {
  #[doc = "0: Fast IRC is disabled in Stop modes."]
  FIRCSTEN_0,
  #[doc = "1: Fast IRC is enabled in Stop modes"]
  FIRCSTEN_1,
}
impl From<FIRCSTEN_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCSTEN_A) -> Self {
    match variant {
      FIRCSTEN_A::FIRCSTEN_0 => false,
      FIRCSTEN_A::FIRCSTEN_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCSTEN`"]
pub type FIRCSTEN_R = crate::R<bool, FIRCSTEN_A>;
impl FIRCSTEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCSTEN_A {
    match self.bits {
      false => FIRCSTEN_A::FIRCSTEN_0,
      true => FIRCSTEN_A::FIRCSTEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCSTEN_0`"]
  #[inline(always)]
  pub fn is_fircsten_0(&self) -> bool {
    *self == FIRCSTEN_A::FIRCSTEN_0
  }
  #[doc = "Checks if the value of the field is `FIRCSTEN_1`"]
  #[inline(always)]
  pub fn is_fircsten_1(&self) -> bool {
    *self == FIRCSTEN_A::FIRCSTEN_1
  }
}
#[doc = "Write proxy for field `FIRCSTEN`"]
pub struct FIRCSTEN_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCSTEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCSTEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fast IRC is disabled in Stop modes."]
  #[inline(always)]
  pub fn fircsten_0(self) -> &'a mut W {
    self.variant(FIRCSTEN_A::FIRCSTEN_0)
  }
  #[doc = "Fast IRC is enabled in Stop modes"]
  #[inline(always)]
  pub fn fircsten_1(self) -> &'a mut W {
    self.variant(FIRCSTEN_A::FIRCSTEN_1)
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
#[doc = "Fast IRC Low Power Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCLPEN_A {
  #[doc = "0: Fast IRC is disabled in VLP modes"]
  FIRCLPEN_0,
  #[doc = "1: Fast IRC is enabled in VLP modes"]
  FIRCLPEN_1,
}
impl From<FIRCLPEN_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCLPEN_A) -> Self {
    match variant {
      FIRCLPEN_A::FIRCLPEN_0 => false,
      FIRCLPEN_A::FIRCLPEN_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCLPEN`"]
pub type FIRCLPEN_R = crate::R<bool, FIRCLPEN_A>;
impl FIRCLPEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCLPEN_A {
    match self.bits {
      false => FIRCLPEN_A::FIRCLPEN_0,
      true => FIRCLPEN_A::FIRCLPEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCLPEN_0`"]
  #[inline(always)]
  pub fn is_firclpen_0(&self) -> bool {
    *self == FIRCLPEN_A::FIRCLPEN_0
  }
  #[doc = "Checks if the value of the field is `FIRCLPEN_1`"]
  #[inline(always)]
  pub fn is_firclpen_1(&self) -> bool {
    *self == FIRCLPEN_A::FIRCLPEN_1
  }
}
#[doc = "Write proxy for field `FIRCLPEN`"]
pub struct FIRCLPEN_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCLPEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCLPEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fast IRC is disabled in VLP modes"]
  #[inline(always)]
  pub fn firclpen_0(self) -> &'a mut W {
    self.variant(FIRCLPEN_A::FIRCLPEN_0)
  }
  #[doc = "Fast IRC is enabled in VLP modes"]
  #[inline(always)]
  pub fn firclpen_1(self) -> &'a mut W {
    self.variant(FIRCLPEN_A::FIRCLPEN_1)
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
#[doc = "Fast IRC Regulator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCREGOFF_A {
  #[doc = "0: Fast IRC Regulator is enabled."]
  FIRCREGOFF_0,
  #[doc = "1: Fast IRC Regulator is disabled."]
  FIRCREGOFF_1,
}
impl From<FIRCREGOFF_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCREGOFF_A) -> Self {
    match variant {
      FIRCREGOFF_A::FIRCREGOFF_0 => false,
      FIRCREGOFF_A::FIRCREGOFF_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCREGOFF`"]
pub type FIRCREGOFF_R = crate::R<bool, FIRCREGOFF_A>;
impl FIRCREGOFF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCREGOFF_A {
    match self.bits {
      false => FIRCREGOFF_A::FIRCREGOFF_0,
      true => FIRCREGOFF_A::FIRCREGOFF_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCREGOFF_0`"]
  #[inline(always)]
  pub fn is_fircregoff_0(&self) -> bool {
    *self == FIRCREGOFF_A::FIRCREGOFF_0
  }
  #[doc = "Checks if the value of the field is `FIRCREGOFF_1`"]
  #[inline(always)]
  pub fn is_fircregoff_1(&self) -> bool {
    *self == FIRCREGOFF_A::FIRCREGOFF_1
  }
}
#[doc = "Write proxy for field `FIRCREGOFF`"]
pub struct FIRCREGOFF_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCREGOFF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCREGOFF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fast IRC Regulator is enabled."]
  #[inline(always)]
  pub fn fircregoff_0(self) -> &'a mut W {
    self.variant(FIRCREGOFF_A::FIRCREGOFF_0)
  }
  #[doc = "Fast IRC Regulator is disabled."]
  #[inline(always)]
  pub fn fircregoff_1(self) -> &'a mut W {
    self.variant(FIRCREGOFF_A::FIRCREGOFF_1)
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
#[doc = "Fast IRC Trim Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCTREN_A {
  #[doc = "0: Disable trimming Fast IRC to an external clock source"]
  FIRCTREN_0,
  #[doc = "1: Enable trimming Fast IRC to an external clock source"]
  FIRCTREN_1,
}
impl From<FIRCTREN_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCTREN_A) -> Self {
    match variant {
      FIRCTREN_A::FIRCTREN_0 => false,
      FIRCTREN_A::FIRCTREN_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCTREN`"]
pub type FIRCTREN_R = crate::R<bool, FIRCTREN_A>;
impl FIRCTREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCTREN_A {
    match self.bits {
      false => FIRCTREN_A::FIRCTREN_0,
      true => FIRCTREN_A::FIRCTREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCTREN_0`"]
  #[inline(always)]
  pub fn is_firctren_0(&self) -> bool {
    *self == FIRCTREN_A::FIRCTREN_0
  }
  #[doc = "Checks if the value of the field is `FIRCTREN_1`"]
  #[inline(always)]
  pub fn is_firctren_1(&self) -> bool {
    *self == FIRCTREN_A::FIRCTREN_1
  }
}
#[doc = "Write proxy for field `FIRCTREN`"]
pub struct FIRCTREN_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCTREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCTREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable trimming Fast IRC to an external clock source"]
  #[inline(always)]
  pub fn firctren_0(self) -> &'a mut W {
    self.variant(FIRCTREN_A::FIRCTREN_0)
  }
  #[doc = "Enable trimming Fast IRC to an external clock source"]
  #[inline(always)]
  pub fn firctren_1(self) -> &'a mut W {
    self.variant(FIRCTREN_A::FIRCTREN_1)
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
#[doc = "Fast IRC Trim Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCTRUP_A {
  #[doc = "0: Disable Fast IRC trimming updates"]
  FIRCTRUP_0,
  #[doc = "1: Enable Fast IRC trimming updates"]
  FIRCTRUP_1,
}
impl From<FIRCTRUP_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCTRUP_A) -> Self {
    match variant {
      FIRCTRUP_A::FIRCTRUP_0 => false,
      FIRCTRUP_A::FIRCTRUP_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCTRUP`"]
pub type FIRCTRUP_R = crate::R<bool, FIRCTRUP_A>;
impl FIRCTRUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCTRUP_A {
    match self.bits {
      false => FIRCTRUP_A::FIRCTRUP_0,
      true => FIRCTRUP_A::FIRCTRUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCTRUP_0`"]
  #[inline(always)]
  pub fn is_firctrup_0(&self) -> bool {
    *self == FIRCTRUP_A::FIRCTRUP_0
  }
  #[doc = "Checks if the value of the field is `FIRCTRUP_1`"]
  #[inline(always)]
  pub fn is_firctrup_1(&self) -> bool {
    *self == FIRCTRUP_A::FIRCTRUP_1
  }
}
#[doc = "Write proxy for field `FIRCTRUP`"]
pub struct FIRCTRUP_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCTRUP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCTRUP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable Fast IRC trimming updates"]
  #[inline(always)]
  pub fn firctrup_0(self) -> &'a mut W {
    self.variant(FIRCTRUP_A::FIRCTRUP_0)
  }
  #[doc = "Enable Fast IRC trimming updates"]
  #[inline(always)]
  pub fn firctrup_1(self) -> &'a mut W {
    self.variant(FIRCTRUP_A::FIRCTRUP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
#[doc = "Fast IRC Valid status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCVLD_A {
  #[doc = "0: Fast IRC is not enabled or clock is not valid."]
  FIRCVLD_0,
  #[doc = "1: Fast IRC is enabled and output clock is valid. The clock is valid once there is an output clock from the FIRC analog."]
  FIRCVLD_1,
}
impl From<FIRCVLD_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCVLD_A) -> Self {
    match variant {
      FIRCVLD_A::FIRCVLD_0 => false,
      FIRCVLD_A::FIRCVLD_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCVLD`"]
pub type FIRCVLD_R = crate::R<bool, FIRCVLD_A>;
impl FIRCVLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCVLD_A {
    match self.bits {
      false => FIRCVLD_A::FIRCVLD_0,
      true => FIRCVLD_A::FIRCVLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCVLD_0`"]
  #[inline(always)]
  pub fn is_fircvld_0(&self) -> bool {
    *self == FIRCVLD_A::FIRCVLD_0
  }
  #[doc = "Checks if the value of the field is `FIRCVLD_1`"]
  #[inline(always)]
  pub fn is_fircvld_1(&self) -> bool {
    *self == FIRCVLD_A::FIRCVLD_1
  }
}
#[doc = "Fast IRC Selected status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCSEL_A {
  #[doc = "0: Fast IRC is not the system clock source"]
  FIRCSEL_0,
  #[doc = "1: Fast IRC is the system clock source"]
  FIRCSEL_1,
}
impl From<FIRCSEL_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCSEL_A) -> Self {
    match variant {
      FIRCSEL_A::FIRCSEL_0 => false,
      FIRCSEL_A::FIRCSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCSEL`"]
pub type FIRCSEL_R = crate::R<bool, FIRCSEL_A>;
impl FIRCSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCSEL_A {
    match self.bits {
      false => FIRCSEL_A::FIRCSEL_0,
      true => FIRCSEL_A::FIRCSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCSEL_0`"]
  #[inline(always)]
  pub fn is_fircsel_0(&self) -> bool {
    *self == FIRCSEL_A::FIRCSEL_0
  }
  #[doc = "Checks if the value of the field is `FIRCSEL_1`"]
  #[inline(always)]
  pub fn is_fircsel_1(&self) -> bool {
    *self == FIRCSEL_A::FIRCSEL_1
  }
}
#[doc = "Fast IRC Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCERR_A {
  #[doc = "0: Error not detected with the Fast IRC trimming."]
  FIRCERR_0,
  #[doc = "1: Error detected with the Fast IRC trimming."]
  FIRCERR_1,
}
impl From<FIRCERR_A> for bool {
  #[inline(always)]
  fn from(variant: FIRCERR_A) -> Self {
    match variant {
      FIRCERR_A::FIRCERR_0 => false,
      FIRCERR_A::FIRCERR_1 => true,
    }
  }
}
#[doc = "Reader of field `FIRCERR`"]
pub type FIRCERR_R = crate::R<bool, FIRCERR_A>;
impl FIRCERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCERR_A {
    match self.bits {
      false => FIRCERR_A::FIRCERR_0,
      true => FIRCERR_A::FIRCERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIRCERR_0`"]
  #[inline(always)]
  pub fn is_fircerr_0(&self) -> bool {
    *self == FIRCERR_A::FIRCERR_0
  }
  #[doc = "Checks if the value of the field is `FIRCERR_1`"]
  #[inline(always)]
  pub fn is_fircerr_1(&self) -> bool {
    *self == FIRCERR_A::FIRCERR_1
  }
}
#[doc = "Write proxy for field `FIRCERR`"]
pub struct FIRCERR_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Error not detected with the Fast IRC trimming."]
  #[inline(always)]
  pub fn fircerr_0(self) -> &'a mut W {
    self.variant(FIRCERR_A::FIRCERR_0)
  }
  #[doc = "Error detected with the Fast IRC trimming."]
  #[inline(always)]
  pub fn fircerr_1(self) -> &'a mut W {
    self.variant(FIRCERR_A::FIRCERR_1)
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
  #[doc = "Bit 0 - Fast IRC Enable"]
  #[inline(always)]
  pub fn fircen(&self) -> FIRCEN_R {
    FIRCEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Fast IRC Stop Enable"]
  #[inline(always)]
  pub fn fircsten(&self) -> FIRCSTEN_R {
    FIRCSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Fast IRC Low Power Enable"]
  #[inline(always)]
  pub fn firclpen(&self) -> FIRCLPEN_R {
    FIRCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Fast IRC Regulator Enable"]
  #[inline(always)]
  pub fn fircregoff(&self) -> FIRCREGOFF_R {
    FIRCREGOFF_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Fast IRC Trim Enable"]
  #[inline(always)]
  pub fn firctren(&self) -> FIRCTREN_R {
    FIRCTREN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Fast IRC Trim Update"]
  #[inline(always)]
  pub fn firctrup(&self) -> FIRCTRUP_R {
    FIRCTRUP_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Fast IRC Valid status"]
  #[inline(always)]
  pub fn fircvld(&self) -> FIRCVLD_R {
    FIRCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Fast IRC Selected status"]
  #[inline(always)]
  pub fn fircsel(&self) -> FIRCSEL_R {
    FIRCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Fast IRC Clock Error"]
  #[inline(always)]
  pub fn fircerr(&self) -> FIRCERR_R {
    FIRCERR_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Fast IRC Enable"]
  #[inline(always)]
  pub fn fircen(&mut self) -> FIRCEN_W {
    FIRCEN_W { w: self }
  }
  #[doc = "Bit 1 - Fast IRC Stop Enable"]
  #[inline(always)]
  pub fn fircsten(&mut self) -> FIRCSTEN_W {
    FIRCSTEN_W { w: self }
  }
  #[doc = "Bit 2 - Fast IRC Low Power Enable"]
  #[inline(always)]
  pub fn firclpen(&mut self) -> FIRCLPEN_W {
    FIRCLPEN_W { w: self }
  }
  #[doc = "Bit 3 - Fast IRC Regulator Enable"]
  #[inline(always)]
  pub fn fircregoff(&mut self) -> FIRCREGOFF_W {
    FIRCREGOFF_W { w: self }
  }
  #[doc = "Bit 8 - Fast IRC Trim Enable"]
  #[inline(always)]
  pub fn firctren(&mut self) -> FIRCTREN_W {
    FIRCTREN_W { w: self }
  }
  #[doc = "Bit 9 - Fast IRC Trim Update"]
  #[inline(always)]
  pub fn firctrup(&mut self) -> FIRCTRUP_W {
    FIRCTRUP_W { w: self }
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
  #[doc = "Bit 26 - Fast IRC Clock Error"]
  #[inline(always)]
  pub fn fircerr(&mut self) -> FIRCERR_W {
    FIRCERR_W { w: self }
  }
}
