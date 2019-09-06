#[doc = "Reader of register ISCR"]
pub type R = crate::R<u32, super::ISCR>;
#[doc = "Writer for register ISCR"]
pub type W = crate::W<u32, super::ISCR>;
#[doc = "Register ISCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FPU invalid operation interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIOC_A {
  #[doc = "0: No interrupt"]
  FIOC_0,
  #[doc = "1: Interrupt occurred"]
  FIOC_1,
}
impl From<FIOC_A> for bool {
  #[inline(always)]
  fn from(variant: FIOC_A) -> Self {
    match variant {
      FIOC_A::FIOC_0 => false,
      FIOC_A::FIOC_1 => true,
    }
  }
}
#[doc = "Reader of field `FIOC`"]
pub type FIOC_R = crate::R<bool, FIOC_A>;
impl FIOC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIOC_A {
    match self.bits {
      false => FIOC_A::FIOC_0,
      true => FIOC_A::FIOC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIOC_0`"]
  #[inline(always)]
  pub fn is_fioc_0(&self) -> bool {
    *self == FIOC_A::FIOC_0
  }
  #[doc = "Checks if the value of the field is `FIOC_1`"]
  #[inline(always)]
  pub fn is_fioc_1(&self) -> bool {
    *self == FIOC_A::FIOC_1
  }
}
#[doc = "FPU divide-by-zero interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDZC_A {
  #[doc = "0: No interrupt"]
  FDZC_0,
  #[doc = "1: Interrupt occurred"]
  FDZC_1,
}
impl From<FDZC_A> for bool {
  #[inline(always)]
  fn from(variant: FDZC_A) -> Self {
    match variant {
      FDZC_A::FDZC_0 => false,
      FDZC_A::FDZC_1 => true,
    }
  }
}
#[doc = "Reader of field `FDZC`"]
pub type FDZC_R = crate::R<bool, FDZC_A>;
impl FDZC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FDZC_A {
    match self.bits {
      false => FDZC_A::FDZC_0,
      true => FDZC_A::FDZC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FDZC_0`"]
  #[inline(always)]
  pub fn is_fdzc_0(&self) -> bool {
    *self == FDZC_A::FDZC_0
  }
  #[doc = "Checks if the value of the field is `FDZC_1`"]
  #[inline(always)]
  pub fn is_fdzc_1(&self) -> bool {
    *self == FDZC_A::FDZC_1
  }
}
#[doc = "FPU overflow interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFC_A {
  #[doc = "0: No interrupt"]
  FOFC_0,
  #[doc = "1: Interrupt occurred"]
  FOFC_1,
}
impl From<FOFC_A> for bool {
  #[inline(always)]
  fn from(variant: FOFC_A) -> Self {
    match variant {
      FOFC_A::FOFC_0 => false,
      FOFC_A::FOFC_1 => true,
    }
  }
}
#[doc = "Reader of field `FOFC`"]
pub type FOFC_R = crate::R<bool, FOFC_A>;
impl FOFC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FOFC_A {
    match self.bits {
      false => FOFC_A::FOFC_0,
      true => FOFC_A::FOFC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FOFC_0`"]
  #[inline(always)]
  pub fn is_fofc_0(&self) -> bool {
    *self == FOFC_A::FOFC_0
  }
  #[doc = "Checks if the value of the field is `FOFC_1`"]
  #[inline(always)]
  pub fn is_fofc_1(&self) -> bool {
    *self == FOFC_A::FOFC_1
  }
}
#[doc = "FPU underflow interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUFC_A {
  #[doc = "0: No interrupt"]
  FUFC_0,
  #[doc = "1: Interrupt occurred"]
  FUFC_1,
}
impl From<FUFC_A> for bool {
  #[inline(always)]
  fn from(variant: FUFC_A) -> Self {
    match variant {
      FUFC_A::FUFC_0 => false,
      FUFC_A::FUFC_1 => true,
    }
  }
}
#[doc = "Reader of field `FUFC`"]
pub type FUFC_R = crate::R<bool, FUFC_A>;
impl FUFC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FUFC_A {
    match self.bits {
      false => FUFC_A::FUFC_0,
      true => FUFC_A::FUFC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FUFC_0`"]
  #[inline(always)]
  pub fn is_fufc_0(&self) -> bool {
    *self == FUFC_A::FUFC_0
  }
  #[doc = "Checks if the value of the field is `FUFC_1`"]
  #[inline(always)]
  pub fn is_fufc_1(&self) -> bool {
    *self == FUFC_A::FUFC_1
  }
}
#[doc = "FPU inexact interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXC_A {
  #[doc = "0: No interrupt"]
  FIXC_0,
  #[doc = "1: Interrupt occurred"]
  FIXC_1,
}
impl From<FIXC_A> for bool {
  #[inline(always)]
  fn from(variant: FIXC_A) -> Self {
    match variant {
      FIXC_A::FIXC_0 => false,
      FIXC_A::FIXC_1 => true,
    }
  }
}
#[doc = "Reader of field `FIXC`"]
pub type FIXC_R = crate::R<bool, FIXC_A>;
impl FIXC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIXC_A {
    match self.bits {
      false => FIXC_A::FIXC_0,
      true => FIXC_A::FIXC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIXC_0`"]
  #[inline(always)]
  pub fn is_fixc_0(&self) -> bool {
    *self == FIXC_A::FIXC_0
  }
  #[doc = "Checks if the value of the field is `FIXC_1`"]
  #[inline(always)]
  pub fn is_fixc_1(&self) -> bool {
    *self == FIXC_A::FIXC_1
  }
}
#[doc = "FPU input denormal interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIDC_A {
  #[doc = "0: No interrupt"]
  FIDC_0,
  #[doc = "1: Interrupt occurred"]
  FIDC_1,
}
impl From<FIDC_A> for bool {
  #[inline(always)]
  fn from(variant: FIDC_A) -> Self {
    match variant {
      FIDC_A::FIDC_0 => false,
      FIDC_A::FIDC_1 => true,
    }
  }
}
#[doc = "Reader of field `FIDC`"]
pub type FIDC_R = crate::R<bool, FIDC_A>;
impl FIDC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIDC_A {
    match self.bits {
      false => FIDC_A::FIDC_0,
      true => FIDC_A::FIDC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIDC_0`"]
  #[inline(always)]
  pub fn is_fidc_0(&self) -> bool {
    *self == FIDC_A::FIDC_0
  }
  #[doc = "Checks if the value of the field is `FIDC_1`"]
  #[inline(always)]
  pub fn is_fidc_1(&self) -> bool {
    *self == FIDC_A::FIDC_1
  }
}
#[doc = "FPU invalid operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIOCE_A {
  #[doc = "0: Disable interrupt"]
  FIOCE_0,
  #[doc = "1: Enable interrupt"]
  FIOCE_1,
}
impl From<FIOCE_A> for bool {
  #[inline(always)]
  fn from(variant: FIOCE_A) -> Self {
    match variant {
      FIOCE_A::FIOCE_0 => false,
      FIOCE_A::FIOCE_1 => true,
    }
  }
}
#[doc = "Reader of field `FIOCE`"]
pub type FIOCE_R = crate::R<bool, FIOCE_A>;
impl FIOCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIOCE_A {
    match self.bits {
      false => FIOCE_A::FIOCE_0,
      true => FIOCE_A::FIOCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIOCE_0`"]
  #[inline(always)]
  pub fn is_fioce_0(&self) -> bool {
    *self == FIOCE_A::FIOCE_0
  }
  #[doc = "Checks if the value of the field is `FIOCE_1`"]
  #[inline(always)]
  pub fn is_fioce_1(&self) -> bool {
    *self == FIOCE_A::FIOCE_1
  }
}
#[doc = "Write proxy for field `FIOCE`"]
pub struct FIOCE_W<'a> {
  w: &'a mut W,
}
impl<'a> FIOCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIOCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable interrupt"]
  #[inline(always)]
  pub fn fioce_0(self) -> &'a mut W {
    self.variant(FIOCE_A::FIOCE_0)
  }
  #[doc = "Enable interrupt"]
  #[inline(always)]
  pub fn fioce_1(self) -> &'a mut W {
    self.variant(FIOCE_A::FIOCE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
    self.w
  }
}
#[doc = "FPU divide-by-zero interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDZCE_A {
  #[doc = "0: Disable interrupt"]
  FDZCE_0,
  #[doc = "1: Enable interrupt"]
  FDZCE_1,
}
impl From<FDZCE_A> for bool {
  #[inline(always)]
  fn from(variant: FDZCE_A) -> Self {
    match variant {
      FDZCE_A::FDZCE_0 => false,
      FDZCE_A::FDZCE_1 => true,
    }
  }
}
#[doc = "Reader of field `FDZCE`"]
pub type FDZCE_R = crate::R<bool, FDZCE_A>;
impl FDZCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FDZCE_A {
    match self.bits {
      false => FDZCE_A::FDZCE_0,
      true => FDZCE_A::FDZCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FDZCE_0`"]
  #[inline(always)]
  pub fn is_fdzce_0(&self) -> bool {
    *self == FDZCE_A::FDZCE_0
  }
  #[doc = "Checks if the value of the field is `FDZCE_1`"]
  #[inline(always)]
  pub fn is_fdzce_1(&self) -> bool {
    *self == FDZCE_A::FDZCE_1
  }
}
#[doc = "Write proxy for field `FDZCE`"]
pub struct FDZCE_W<'a> {
  w: &'a mut W,
}
impl<'a> FDZCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FDZCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable interrupt"]
  #[inline(always)]
  pub fn fdzce_0(self) -> &'a mut W {
    self.variant(FDZCE_A::FDZCE_0)
  }
  #[doc = "Enable interrupt"]
  #[inline(always)]
  pub fn fdzce_1(self) -> &'a mut W {
    self.variant(FDZCE_A::FDZCE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
    self.w
  }
}
#[doc = "FPU overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFCE_A {
  #[doc = "0: Disable interrupt"]
  FOFCE_0,
  #[doc = "1: Enable interrupt"]
  FOFCE_1,
}
impl From<FOFCE_A> for bool {
  #[inline(always)]
  fn from(variant: FOFCE_A) -> Self {
    match variant {
      FOFCE_A::FOFCE_0 => false,
      FOFCE_A::FOFCE_1 => true,
    }
  }
}
#[doc = "Reader of field `FOFCE`"]
pub type FOFCE_R = crate::R<bool, FOFCE_A>;
impl FOFCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FOFCE_A {
    match self.bits {
      false => FOFCE_A::FOFCE_0,
      true => FOFCE_A::FOFCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FOFCE_0`"]
  #[inline(always)]
  pub fn is_fofce_0(&self) -> bool {
    *self == FOFCE_A::FOFCE_0
  }
  #[doc = "Checks if the value of the field is `FOFCE_1`"]
  #[inline(always)]
  pub fn is_fofce_1(&self) -> bool {
    *self == FOFCE_A::FOFCE_1
  }
}
#[doc = "Write proxy for field `FOFCE`"]
pub struct FOFCE_W<'a> {
  w: &'a mut W,
}
impl<'a> FOFCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FOFCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable interrupt"]
  #[inline(always)]
  pub fn fofce_0(self) -> &'a mut W {
    self.variant(FOFCE_A::FOFCE_0)
  }
  #[doc = "Enable interrupt"]
  #[inline(always)]
  pub fn fofce_1(self) -> &'a mut W {
    self.variant(FOFCE_A::FOFCE_1)
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
#[doc = "FPU underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUFCE_A {
  #[doc = "0: Disable interrupt"]
  FUFCE_0,
  #[doc = "1: Enable interrupt"]
  FUFCE_1,
}
impl From<FUFCE_A> for bool {
  #[inline(always)]
  fn from(variant: FUFCE_A) -> Self {
    match variant {
      FUFCE_A::FUFCE_0 => false,
      FUFCE_A::FUFCE_1 => true,
    }
  }
}
#[doc = "Reader of field `FUFCE`"]
pub type FUFCE_R = crate::R<bool, FUFCE_A>;
impl FUFCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FUFCE_A {
    match self.bits {
      false => FUFCE_A::FUFCE_0,
      true => FUFCE_A::FUFCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FUFCE_0`"]
  #[inline(always)]
  pub fn is_fufce_0(&self) -> bool {
    *self == FUFCE_A::FUFCE_0
  }
  #[doc = "Checks if the value of the field is `FUFCE_1`"]
  #[inline(always)]
  pub fn is_fufce_1(&self) -> bool {
    *self == FUFCE_A::FUFCE_1
  }
}
#[doc = "Write proxy for field `FUFCE`"]
pub struct FUFCE_W<'a> {
  w: &'a mut W,
}
impl<'a> FUFCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FUFCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable interrupt"]
  #[inline(always)]
  pub fn fufce_0(self) -> &'a mut W {
    self.variant(FUFCE_A::FUFCE_0)
  }
  #[doc = "Enable interrupt"]
  #[inline(always)]
  pub fn fufce_1(self) -> &'a mut W {
    self.variant(FUFCE_A::FUFCE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
    self.w
  }
}
#[doc = "FPU inexact interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXCE_A {
  #[doc = "0: Disable interrupt"]
  FIXCE_0,
  #[doc = "1: Enable interrupt"]
  FIXCE_1,
}
impl From<FIXCE_A> for bool {
  #[inline(always)]
  fn from(variant: FIXCE_A) -> Self {
    match variant {
      FIXCE_A::FIXCE_0 => false,
      FIXCE_A::FIXCE_1 => true,
    }
  }
}
#[doc = "Reader of field `FIXCE`"]
pub type FIXCE_R = crate::R<bool, FIXCE_A>;
impl FIXCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIXCE_A {
    match self.bits {
      false => FIXCE_A::FIXCE_0,
      true => FIXCE_A::FIXCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIXCE_0`"]
  #[inline(always)]
  pub fn is_fixce_0(&self) -> bool {
    *self == FIXCE_A::FIXCE_0
  }
  #[doc = "Checks if the value of the field is `FIXCE_1`"]
  #[inline(always)]
  pub fn is_fixce_1(&self) -> bool {
    *self == FIXCE_A::FIXCE_1
  }
}
#[doc = "Write proxy for field `FIXCE`"]
pub struct FIXCE_W<'a> {
  w: &'a mut W,
}
impl<'a> FIXCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIXCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable interrupt"]
  #[inline(always)]
  pub fn fixce_0(self) -> &'a mut W {
    self.variant(FIXCE_A::FIXCE_0)
  }
  #[doc = "Enable interrupt"]
  #[inline(always)]
  pub fn fixce_1(self) -> &'a mut W {
    self.variant(FIXCE_A::FIXCE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
    self.w
  }
}
#[doc = "FPU input denormal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIDCE_A {
  #[doc = "0: Disable interrupt"]
  FIDCE_0,
  #[doc = "1: Enable interrupt"]
  FIDCE_1,
}
impl From<FIDCE_A> for bool {
  #[inline(always)]
  fn from(variant: FIDCE_A) -> Self {
    match variant {
      FIDCE_A::FIDCE_0 => false,
      FIDCE_A::FIDCE_1 => true,
    }
  }
}
#[doc = "Reader of field `FIDCE`"]
pub type FIDCE_R = crate::R<bool, FIDCE_A>;
impl FIDCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIDCE_A {
    match self.bits {
      false => FIDCE_A::FIDCE_0,
      true => FIDCE_A::FIDCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIDCE_0`"]
  #[inline(always)]
  pub fn is_fidce_0(&self) -> bool {
    *self == FIDCE_A::FIDCE_0
  }
  #[doc = "Checks if the value of the field is `FIDCE_1`"]
  #[inline(always)]
  pub fn is_fidce_1(&self) -> bool {
    *self == FIDCE_A::FIDCE_1
  }
}
#[doc = "Write proxy for field `FIDCE`"]
pub struct FIDCE_W<'a> {
  w: &'a mut W,
}
impl<'a> FIDCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIDCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable interrupt"]
  #[inline(always)]
  pub fn fidce_0(self) -> &'a mut W {
    self.variant(FIDCE_A::FIDCE_0)
  }
  #[doc = "Enable interrupt"]
  #[inline(always)]
  pub fn fidce_1(self) -> &'a mut W {
    self.variant(FIDCE_A::FIDCE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bit 8 - FPU invalid operation interrupt status"]
  #[inline(always)]
  pub fn fioc(&self) -> FIOC_R {
    FIOC_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - FPU divide-by-zero interrupt status"]
  #[inline(always)]
  pub fn fdzc(&self) -> FDZC_R {
    FDZC_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - FPU overflow interrupt status"]
  #[inline(always)]
  pub fn fofc(&self) -> FOFC_R {
    FOFC_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - FPU underflow interrupt status"]
  #[inline(always)]
  pub fn fufc(&self) -> FUFC_R {
    FUFC_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - FPU inexact interrupt status"]
  #[inline(always)]
  pub fn fixc(&self) -> FIXC_R {
    FIXC_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 15 - FPU input denormal interrupt status"]
  #[inline(always)]
  pub fn fidc(&self) -> FIDC_R {
    FIDC_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 24 - FPU invalid operation interrupt enable"]
  #[inline(always)]
  pub fn fioce(&self) -> FIOCE_R {
    FIOCE_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - FPU divide-by-zero interrupt enable"]
  #[inline(always)]
  pub fn fdzce(&self) -> FDZCE_R {
    FDZCE_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - FPU overflow interrupt enable"]
  #[inline(always)]
  pub fn fofce(&self) -> FOFCE_R {
    FOFCE_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - FPU underflow interrupt enable"]
  #[inline(always)]
  pub fn fufce(&self) -> FUFCE_R {
    FUFCE_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - FPU inexact interrupt enable"]
  #[inline(always)]
  pub fn fixce(&self) -> FIXCE_R {
    FIXCE_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 31 - FPU input denormal interrupt enable"]
  #[inline(always)]
  pub fn fidce(&self) -> FIDCE_R {
    FIDCE_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 24 - FPU invalid operation interrupt enable"]
  #[inline(always)]
  pub fn fioce(&mut self) -> FIOCE_W {
    FIOCE_W { w: self }
  }
  #[doc = "Bit 25 - FPU divide-by-zero interrupt enable"]
  #[inline(always)]
  pub fn fdzce(&mut self) -> FDZCE_W {
    FDZCE_W { w: self }
  }
  #[doc = "Bit 26 - FPU overflow interrupt enable"]
  #[inline(always)]
  pub fn fofce(&mut self) -> FOFCE_W {
    FOFCE_W { w: self }
  }
  #[doc = "Bit 27 - FPU underflow interrupt enable"]
  #[inline(always)]
  pub fn fufce(&mut self) -> FUFCE_W {
    FUFCE_W { w: self }
  }
  #[doc = "Bit 28 - FPU inexact interrupt enable"]
  #[inline(always)]
  pub fn fixce(&mut self) -> FIXCE_W {
    FIXCE_W { w: self }
  }
  #[doc = "Bit 31 - FPU input denormal interrupt enable"]
  #[inline(always)]
  pub fn fidce(&mut self) -> FIDCE_W {
    FIDCE_W { w: self }
  }
}
