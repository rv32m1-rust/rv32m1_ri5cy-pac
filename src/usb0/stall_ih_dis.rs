#[doc = "Reader of register STALL_IH_DIS"]
pub type R = crate::R<u8, super::STALL_IH_DIS>;
#[doc = "Writer for register STALL_IH_DIS"]
pub type W = crate::W<u8, super::STALL_IH_DIS>;
#[doc = "Register STALL_IH_DIS `reset()`'s with value 0"]
impl crate::ResetValue for super::STALL_IH_DIS {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "STALL_I_DIS8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS8_A {
  #[doc = "0: Endpoint 8 IN direction stall is enabled."]
  STALL_I_DIS8_0,
  #[doc = "1: Endpoint 8 IN direction stall is disabled."]
  STALL_I_DIS8_1,
}
impl From<STALL_I_DIS8_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS8_A) -> Self {
    match variant {
      STALL_I_DIS8_A::STALL_I_DIS8_0 => false,
      STALL_I_DIS8_A::STALL_I_DIS8_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS8`"]
pub type STALL_I_DIS8_R = crate::R<bool, STALL_I_DIS8_A>;
impl STALL_I_DIS8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS8_A {
    match self.bits {
      false => STALL_I_DIS8_A::STALL_I_DIS8_0,
      true => STALL_I_DIS8_A::STALL_I_DIS8_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS8_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis8_0(&self) -> bool {
    *self == STALL_I_DIS8_A::STALL_I_DIS8_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS8_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis8_1(&self) -> bool {
    *self == STALL_I_DIS8_A::STALL_I_DIS8_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS8`"]
pub struct STALL_I_DIS8_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS8_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS8_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 8 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis8_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS8_A::STALL_I_DIS8_0)
  }
  #[doc = "Endpoint 8 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis8_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS8_A::STALL_I_DIS8_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
    self.w
  }
}
#[doc = "STALL_I_DIS9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS9_A {
  #[doc = "0: Endpoint 9 IN direction stall is enabled."]
  STALL_I_DIS9_0,
  #[doc = "1: Endpoint 9 IN direction stall is disabled."]
  STALL_I_DIS9_1,
}
impl From<STALL_I_DIS9_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS9_A) -> Self {
    match variant {
      STALL_I_DIS9_A::STALL_I_DIS9_0 => false,
      STALL_I_DIS9_A::STALL_I_DIS9_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS9`"]
pub type STALL_I_DIS9_R = crate::R<bool, STALL_I_DIS9_A>;
impl STALL_I_DIS9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS9_A {
    match self.bits {
      false => STALL_I_DIS9_A::STALL_I_DIS9_0,
      true => STALL_I_DIS9_A::STALL_I_DIS9_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS9_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis9_0(&self) -> bool {
    *self == STALL_I_DIS9_A::STALL_I_DIS9_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS9_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis9_1(&self) -> bool {
    *self == STALL_I_DIS9_A::STALL_I_DIS9_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS9`"]
pub struct STALL_I_DIS9_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS9_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS9_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 9 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis9_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS9_A::STALL_I_DIS9_0)
  }
  #[doc = "Endpoint 9 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis9_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS9_A::STALL_I_DIS9_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
    self.w
  }
}
#[doc = "STALL_I_DIS10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS10_A {
  #[doc = "0: Endpoint 10 IN direction stall is enabled."]
  STALL_I_DIS10_0,
  #[doc = "1: Endpoint 10 IN direction stall is disabled."]
  STALL_I_DIS10_1,
}
impl From<STALL_I_DIS10_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS10_A) -> Self {
    match variant {
      STALL_I_DIS10_A::STALL_I_DIS10_0 => false,
      STALL_I_DIS10_A::STALL_I_DIS10_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS10`"]
pub type STALL_I_DIS10_R = crate::R<bool, STALL_I_DIS10_A>;
impl STALL_I_DIS10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS10_A {
    match self.bits {
      false => STALL_I_DIS10_A::STALL_I_DIS10_0,
      true => STALL_I_DIS10_A::STALL_I_DIS10_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS10_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis10_0(&self) -> bool {
    *self == STALL_I_DIS10_A::STALL_I_DIS10_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS10_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis10_1(&self) -> bool {
    *self == STALL_I_DIS10_A::STALL_I_DIS10_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS10`"]
pub struct STALL_I_DIS10_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS10_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS10_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 10 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis10_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS10_A::STALL_I_DIS10_0)
  }
  #[doc = "Endpoint 10 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis10_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS10_A::STALL_I_DIS10_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
    self.w
  }
}
#[doc = "STALL_I_DIS11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS11_A {
  #[doc = "0: Endpoint 11 IN direction stall is enabled."]
  STALL_I_DIS11_0,
  #[doc = "1: Endpoint 11 IN direction stall is disabled."]
  STALL_I_DIS11_1,
}
impl From<STALL_I_DIS11_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS11_A) -> Self {
    match variant {
      STALL_I_DIS11_A::STALL_I_DIS11_0 => false,
      STALL_I_DIS11_A::STALL_I_DIS11_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS11`"]
pub type STALL_I_DIS11_R = crate::R<bool, STALL_I_DIS11_A>;
impl STALL_I_DIS11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS11_A {
    match self.bits {
      false => STALL_I_DIS11_A::STALL_I_DIS11_0,
      true => STALL_I_DIS11_A::STALL_I_DIS11_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS11_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis11_0(&self) -> bool {
    *self == STALL_I_DIS11_A::STALL_I_DIS11_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS11_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis11_1(&self) -> bool {
    *self == STALL_I_DIS11_A::STALL_I_DIS11_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS11`"]
pub struct STALL_I_DIS11_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS11_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS11_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 11 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis11_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS11_A::STALL_I_DIS11_0)
  }
  #[doc = "Endpoint 11 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis11_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS11_A::STALL_I_DIS11_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
    self.w
  }
}
#[doc = "STALL_I_DIS12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS12_A {
  #[doc = "0: Endpoint 12 IN direction stall is enabled."]
  STALL_I_DIS12_0,
  #[doc = "1: Endpoint 12 IN direction stall is disabled."]
  STALL_I_DIS12_1,
}
impl From<STALL_I_DIS12_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS12_A) -> Self {
    match variant {
      STALL_I_DIS12_A::STALL_I_DIS12_0 => false,
      STALL_I_DIS12_A::STALL_I_DIS12_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS12`"]
pub type STALL_I_DIS12_R = crate::R<bool, STALL_I_DIS12_A>;
impl STALL_I_DIS12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS12_A {
    match self.bits {
      false => STALL_I_DIS12_A::STALL_I_DIS12_0,
      true => STALL_I_DIS12_A::STALL_I_DIS12_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS12_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis12_0(&self) -> bool {
    *self == STALL_I_DIS12_A::STALL_I_DIS12_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS12_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis12_1(&self) -> bool {
    *self == STALL_I_DIS12_A::STALL_I_DIS12_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS12`"]
pub struct STALL_I_DIS12_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS12_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS12_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 12 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis12_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS12_A::STALL_I_DIS12_0)
  }
  #[doc = "Endpoint 12 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis12_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS12_A::STALL_I_DIS12_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
    self.w
  }
}
#[doc = "STALL_I_DIS13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS13_A {
  #[doc = "0: Endpoint 13 IN direction stall is enabled."]
  STALL_I_DIS13_0,
  #[doc = "1: Endpoint 13 IN direction stall is disabled."]
  STALL_I_DIS13_1,
}
impl From<STALL_I_DIS13_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS13_A) -> Self {
    match variant {
      STALL_I_DIS13_A::STALL_I_DIS13_0 => false,
      STALL_I_DIS13_A::STALL_I_DIS13_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS13`"]
pub type STALL_I_DIS13_R = crate::R<bool, STALL_I_DIS13_A>;
impl STALL_I_DIS13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS13_A {
    match self.bits {
      false => STALL_I_DIS13_A::STALL_I_DIS13_0,
      true => STALL_I_DIS13_A::STALL_I_DIS13_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS13_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis13_0(&self) -> bool {
    *self == STALL_I_DIS13_A::STALL_I_DIS13_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS13_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis13_1(&self) -> bool {
    *self == STALL_I_DIS13_A::STALL_I_DIS13_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS13`"]
pub struct STALL_I_DIS13_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS13_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS13_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 13 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis13_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS13_A::STALL_I_DIS13_0)
  }
  #[doc = "Endpoint 13 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis13_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS13_A::STALL_I_DIS13_1)
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
#[doc = "STALL_I_DIS14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS14_A {
  #[doc = "0: Endpoint 14 IN direction stall is enabled."]
  STALL_I_DIS14_0,
  #[doc = "1: Endpoint 14 IN direction stall is disabled."]
  STALL_I_DIS14_1,
}
impl From<STALL_I_DIS14_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS14_A) -> Self {
    match variant {
      STALL_I_DIS14_A::STALL_I_DIS14_0 => false,
      STALL_I_DIS14_A::STALL_I_DIS14_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS14`"]
pub type STALL_I_DIS14_R = crate::R<bool, STALL_I_DIS14_A>;
impl STALL_I_DIS14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS14_A {
    match self.bits {
      false => STALL_I_DIS14_A::STALL_I_DIS14_0,
      true => STALL_I_DIS14_A::STALL_I_DIS14_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS14_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis14_0(&self) -> bool {
    *self == STALL_I_DIS14_A::STALL_I_DIS14_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS14_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis14_1(&self) -> bool {
    *self == STALL_I_DIS14_A::STALL_I_DIS14_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS14`"]
pub struct STALL_I_DIS14_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS14_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS14_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 14 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis14_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS14_A::STALL_I_DIS14_0)
  }
  #[doc = "Endpoint 14 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis14_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS14_A::STALL_I_DIS14_1)
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
#[doc = "STALL_I_DIS15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS15_A {
  #[doc = "0: Endpoint 15 IN direction stall is enabled."]
  STALL_I_DIS15_0,
  #[doc = "1: Endpoint 15 IN direction stall is disabled."]
  STALL_I_DIS15_1,
}
impl From<STALL_I_DIS15_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS15_A) -> Self {
    match variant {
      STALL_I_DIS15_A::STALL_I_DIS15_0 => false,
      STALL_I_DIS15_A::STALL_I_DIS15_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS15`"]
pub type STALL_I_DIS15_R = crate::R<bool, STALL_I_DIS15_A>;
impl STALL_I_DIS15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS15_A {
    match self.bits {
      false => STALL_I_DIS15_A::STALL_I_DIS15_0,
      true => STALL_I_DIS15_A::STALL_I_DIS15_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS15_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis15_0(&self) -> bool {
    *self == STALL_I_DIS15_A::STALL_I_DIS15_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS15_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis15_1(&self) -> bool {
    *self == STALL_I_DIS15_A::STALL_I_DIS15_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS15`"]
pub struct STALL_I_DIS15_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS15_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS15_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 15 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis15_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS15_A::STALL_I_DIS15_0)
  }
  #[doc = "Endpoint 15 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis15_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS15_A::STALL_I_DIS15_1)
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
  #[doc = "Bit 0 - STALL_I_DIS8"]
  #[inline(always)]
  pub fn stall_i_dis8(&self) -> STALL_I_DIS8_R {
    STALL_I_DIS8_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - STALL_I_DIS9"]
  #[inline(always)]
  pub fn stall_i_dis9(&self) -> STALL_I_DIS9_R {
    STALL_I_DIS9_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - STALL_I_DIS10"]
  #[inline(always)]
  pub fn stall_i_dis10(&self) -> STALL_I_DIS10_R {
    STALL_I_DIS10_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - STALL_I_DIS11"]
  #[inline(always)]
  pub fn stall_i_dis11(&self) -> STALL_I_DIS11_R {
    STALL_I_DIS11_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - STALL_I_DIS12"]
  #[inline(always)]
  pub fn stall_i_dis12(&self) -> STALL_I_DIS12_R {
    STALL_I_DIS12_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - STALL_I_DIS13"]
  #[inline(always)]
  pub fn stall_i_dis13(&self) -> STALL_I_DIS13_R {
    STALL_I_DIS13_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - STALL_I_DIS14"]
  #[inline(always)]
  pub fn stall_i_dis14(&self) -> STALL_I_DIS14_R {
    STALL_I_DIS14_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - STALL_I_DIS15"]
  #[inline(always)]
  pub fn stall_i_dis15(&self) -> STALL_I_DIS15_R {
    STALL_I_DIS15_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - STALL_I_DIS8"]
  #[inline(always)]
  pub fn stall_i_dis8(&mut self) -> STALL_I_DIS8_W {
    STALL_I_DIS8_W { w: self }
  }
  #[doc = "Bit 1 - STALL_I_DIS9"]
  #[inline(always)]
  pub fn stall_i_dis9(&mut self) -> STALL_I_DIS9_W {
    STALL_I_DIS9_W { w: self }
  }
  #[doc = "Bit 2 - STALL_I_DIS10"]
  #[inline(always)]
  pub fn stall_i_dis10(&mut self) -> STALL_I_DIS10_W {
    STALL_I_DIS10_W { w: self }
  }
  #[doc = "Bit 3 - STALL_I_DIS11"]
  #[inline(always)]
  pub fn stall_i_dis11(&mut self) -> STALL_I_DIS11_W {
    STALL_I_DIS11_W { w: self }
  }
  #[doc = "Bit 4 - STALL_I_DIS12"]
  #[inline(always)]
  pub fn stall_i_dis12(&mut self) -> STALL_I_DIS12_W {
    STALL_I_DIS12_W { w: self }
  }
  #[doc = "Bit 5 - STALL_I_DIS13"]
  #[inline(always)]
  pub fn stall_i_dis13(&mut self) -> STALL_I_DIS13_W {
    STALL_I_DIS13_W { w: self }
  }
  #[doc = "Bit 6 - STALL_I_DIS14"]
  #[inline(always)]
  pub fn stall_i_dis14(&mut self) -> STALL_I_DIS14_W {
    STALL_I_DIS14_W { w: self }
  }
  #[doc = "Bit 7 - STALL_I_DIS15"]
  #[inline(always)]
  pub fn stall_i_dis15(&mut self) -> STALL_I_DIS15_W {
    STALL_I_DIS15_W { w: self }
  }
}
