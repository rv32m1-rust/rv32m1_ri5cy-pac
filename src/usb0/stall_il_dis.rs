#[doc = "Reader of register STALL_IL_DIS"]
pub type R = crate::R<u8, super::STALL_IL_DIS>;
#[doc = "Writer for register STALL_IL_DIS"]
pub type W = crate::W<u8, super::STALL_IL_DIS>;
#[doc = "Register STALL_IL_DIS `reset()`'s with value 0"]
impl crate::ResetValue for super::STALL_IL_DIS {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "STALL_I_DIS0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS0_A {
  #[doc = "0: Endpoint 0 IN direction stall is enabled."]
  STALL_I_DIS0_0,
  #[doc = "1: Endpoint 0 IN direction stall is disabled."]
  STALL_I_DIS0_1,
}
impl From<STALL_I_DIS0_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS0_A) -> Self {
    match variant {
      STALL_I_DIS0_A::STALL_I_DIS0_0 => false,
      STALL_I_DIS0_A::STALL_I_DIS0_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS0`"]
pub type STALL_I_DIS0_R = crate::R<bool, STALL_I_DIS0_A>;
impl STALL_I_DIS0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS0_A {
    match self.bits {
      false => STALL_I_DIS0_A::STALL_I_DIS0_0,
      true => STALL_I_DIS0_A::STALL_I_DIS0_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS0_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis0_0(&self) -> bool {
    *self == STALL_I_DIS0_A::STALL_I_DIS0_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS0_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis0_1(&self) -> bool {
    *self == STALL_I_DIS0_A::STALL_I_DIS0_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS0`"]
pub struct STALL_I_DIS0_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 0 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis0_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS0_A::STALL_I_DIS0_0)
  }
  #[doc = "Endpoint 0 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis0_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS0_A::STALL_I_DIS0_1)
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
#[doc = "STALL_I_DIS1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS1_A {
  #[doc = "0: Endpoint 1 IN direction stall is enabled."]
  STALL_I_DIS1_0,
  #[doc = "1: Endpoint 1 IN direction stall is disabled."]
  STALL_I_DIS1_1,
}
impl From<STALL_I_DIS1_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS1_A) -> Self {
    match variant {
      STALL_I_DIS1_A::STALL_I_DIS1_0 => false,
      STALL_I_DIS1_A::STALL_I_DIS1_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS1`"]
pub type STALL_I_DIS1_R = crate::R<bool, STALL_I_DIS1_A>;
impl STALL_I_DIS1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS1_A {
    match self.bits {
      false => STALL_I_DIS1_A::STALL_I_DIS1_0,
      true => STALL_I_DIS1_A::STALL_I_DIS1_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS1_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis1_0(&self) -> bool {
    *self == STALL_I_DIS1_A::STALL_I_DIS1_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS1_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis1_1(&self) -> bool {
    *self == STALL_I_DIS1_A::STALL_I_DIS1_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS1`"]
pub struct STALL_I_DIS1_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 1 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis1_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS1_A::STALL_I_DIS1_0)
  }
  #[doc = "Endpoint 1 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis1_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS1_A::STALL_I_DIS1_1)
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
#[doc = "STALL_I_DIS2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS2_A {
  #[doc = "0: Endpoint 2 IN direction stall is enabled."]
  STALL_I_DIS2_0,
  #[doc = "1: Endpoint 2 IN direction stall is disabled."]
  STALL_I_DIS2_1,
}
impl From<STALL_I_DIS2_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS2_A) -> Self {
    match variant {
      STALL_I_DIS2_A::STALL_I_DIS2_0 => false,
      STALL_I_DIS2_A::STALL_I_DIS2_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS2`"]
pub type STALL_I_DIS2_R = crate::R<bool, STALL_I_DIS2_A>;
impl STALL_I_DIS2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS2_A {
    match self.bits {
      false => STALL_I_DIS2_A::STALL_I_DIS2_0,
      true => STALL_I_DIS2_A::STALL_I_DIS2_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS2_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis2_0(&self) -> bool {
    *self == STALL_I_DIS2_A::STALL_I_DIS2_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS2_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis2_1(&self) -> bool {
    *self == STALL_I_DIS2_A::STALL_I_DIS2_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS2`"]
pub struct STALL_I_DIS2_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 2 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis2_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS2_A::STALL_I_DIS2_0)
  }
  #[doc = "Endpoint 2 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis2_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS2_A::STALL_I_DIS2_1)
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
#[doc = "STALL_I_DIS3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS3_A {
  #[doc = "0: Endpoint 3 IN direction stall is enabled."]
  STALL_I_DIS3_0,
  #[doc = "1: Endpoint 3 IN direction stall is disabled."]
  STALL_I_DIS3_1,
}
impl From<STALL_I_DIS3_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS3_A) -> Self {
    match variant {
      STALL_I_DIS3_A::STALL_I_DIS3_0 => false,
      STALL_I_DIS3_A::STALL_I_DIS3_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS3`"]
pub type STALL_I_DIS3_R = crate::R<bool, STALL_I_DIS3_A>;
impl STALL_I_DIS3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS3_A {
    match self.bits {
      false => STALL_I_DIS3_A::STALL_I_DIS3_0,
      true => STALL_I_DIS3_A::STALL_I_DIS3_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS3_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis3_0(&self) -> bool {
    *self == STALL_I_DIS3_A::STALL_I_DIS3_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS3_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis3_1(&self) -> bool {
    *self == STALL_I_DIS3_A::STALL_I_DIS3_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS3`"]
pub struct STALL_I_DIS3_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 3 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis3_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS3_A::STALL_I_DIS3_0)
  }
  #[doc = "Endpoint 3 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis3_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS3_A::STALL_I_DIS3_1)
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
#[doc = "STALL_I_DIS4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS4_A {
  #[doc = "0: Endpoint 4 IN direction stall is enabled."]
  STALL_I_DIS4_0,
  #[doc = "1: Endpoint 4 IN direction stall is disabled."]
  STALL_I_DIS4_1,
}
impl From<STALL_I_DIS4_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS4_A) -> Self {
    match variant {
      STALL_I_DIS4_A::STALL_I_DIS4_0 => false,
      STALL_I_DIS4_A::STALL_I_DIS4_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS4`"]
pub type STALL_I_DIS4_R = crate::R<bool, STALL_I_DIS4_A>;
impl STALL_I_DIS4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS4_A {
    match self.bits {
      false => STALL_I_DIS4_A::STALL_I_DIS4_0,
      true => STALL_I_DIS4_A::STALL_I_DIS4_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS4_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis4_0(&self) -> bool {
    *self == STALL_I_DIS4_A::STALL_I_DIS4_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS4_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis4_1(&self) -> bool {
    *self == STALL_I_DIS4_A::STALL_I_DIS4_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS4`"]
pub struct STALL_I_DIS4_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 4 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis4_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS4_A::STALL_I_DIS4_0)
  }
  #[doc = "Endpoint 4 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis4_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS4_A::STALL_I_DIS4_1)
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
#[doc = "STALL_I_DIS5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS5_A {
  #[doc = "0: Endpoint 5 IN direction stall is enabled."]
  STALL_I_DIS5_0,
  #[doc = "1: Endpoint 5 IN direction stall is disabled."]
  STALL_I_DIS5_1,
}
impl From<STALL_I_DIS5_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS5_A) -> Self {
    match variant {
      STALL_I_DIS5_A::STALL_I_DIS5_0 => false,
      STALL_I_DIS5_A::STALL_I_DIS5_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS5`"]
pub type STALL_I_DIS5_R = crate::R<bool, STALL_I_DIS5_A>;
impl STALL_I_DIS5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS5_A {
    match self.bits {
      false => STALL_I_DIS5_A::STALL_I_DIS5_0,
      true => STALL_I_DIS5_A::STALL_I_DIS5_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS5_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis5_0(&self) -> bool {
    *self == STALL_I_DIS5_A::STALL_I_DIS5_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS5_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis5_1(&self) -> bool {
    *self == STALL_I_DIS5_A::STALL_I_DIS5_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS5`"]
pub struct STALL_I_DIS5_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 5 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis5_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS5_A::STALL_I_DIS5_0)
  }
  #[doc = "Endpoint 5 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis5_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS5_A::STALL_I_DIS5_1)
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
#[doc = "STALL_I_DIS6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS6_A {
  #[doc = "0: Endpoint 6 IN direction stall is enabled."]
  STALL_I_DIS6_0,
  #[doc = "1: Endpoint 6 IN direction stall is disabled."]
  STALL_I_DIS6_1,
}
impl From<STALL_I_DIS6_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS6_A) -> Self {
    match variant {
      STALL_I_DIS6_A::STALL_I_DIS6_0 => false,
      STALL_I_DIS6_A::STALL_I_DIS6_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS6`"]
pub type STALL_I_DIS6_R = crate::R<bool, STALL_I_DIS6_A>;
impl STALL_I_DIS6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS6_A {
    match self.bits {
      false => STALL_I_DIS6_A::STALL_I_DIS6_0,
      true => STALL_I_DIS6_A::STALL_I_DIS6_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS6_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis6_0(&self) -> bool {
    *self == STALL_I_DIS6_A::STALL_I_DIS6_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS6_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis6_1(&self) -> bool {
    *self == STALL_I_DIS6_A::STALL_I_DIS6_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS6`"]
pub struct STALL_I_DIS6_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS6_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 6 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis6_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS6_A::STALL_I_DIS6_0)
  }
  #[doc = "Endpoint 6 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis6_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS6_A::STALL_I_DIS6_1)
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
#[doc = "STALL_I_DIS7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS7_A {
  #[doc = "0: Endpoint 7 IN direction stall is enabled."]
  STALL_I_DIS7_0,
  #[doc = "1: Endpoint 7 IN direction stall is disabled."]
  STALL_I_DIS7_1,
}
impl From<STALL_I_DIS7_A> for bool {
  #[inline(always)]
  fn from(variant: STALL_I_DIS7_A) -> Self {
    match variant {
      STALL_I_DIS7_A::STALL_I_DIS7_0 => false,
      STALL_I_DIS7_A::STALL_I_DIS7_1 => true,
    }
  }
}
#[doc = "Reader of field `STALL_I_DIS7`"]
pub type STALL_I_DIS7_R = crate::R<bool, STALL_I_DIS7_A>;
impl STALL_I_DIS7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STALL_I_DIS7_A {
    match self.bits {
      false => STALL_I_DIS7_A::STALL_I_DIS7_0,
      true => STALL_I_DIS7_A::STALL_I_DIS7_1,
    }
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS7_0`"]
  #[inline(always)]
  pub fn is_stall_i_dis7_0(&self) -> bool {
    *self == STALL_I_DIS7_A::STALL_I_DIS7_0
  }
  #[doc = "Checks if the value of the field is `STALL_I_DIS7_1`"]
  #[inline(always)]
  pub fn is_stall_i_dis7_1(&self) -> bool {
    *self == STALL_I_DIS7_A::STALL_I_DIS7_1
  }
}
#[doc = "Write proxy for field `STALL_I_DIS7`"]
pub struct STALL_I_DIS7_W<'a> {
  w: &'a mut W,
}
impl<'a> STALL_I_DIS7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STALL_I_DIS7_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Endpoint 7 IN direction stall is enabled."]
  #[inline(always)]
  pub fn stall_i_dis7_0(self) -> &'a mut W {
    self.variant(STALL_I_DIS7_A::STALL_I_DIS7_0)
  }
  #[doc = "Endpoint 7 IN direction stall is disabled."]
  #[inline(always)]
  pub fn stall_i_dis7_1(self) -> &'a mut W {
    self.variant(STALL_I_DIS7_A::STALL_I_DIS7_1)
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
  #[doc = "Bit 0 - STALL_I_DIS0"]
  #[inline(always)]
  pub fn stall_i_dis0(&self) -> STALL_I_DIS0_R {
    STALL_I_DIS0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - STALL_I_DIS1"]
  #[inline(always)]
  pub fn stall_i_dis1(&self) -> STALL_I_DIS1_R {
    STALL_I_DIS1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - STALL_I_DIS2"]
  #[inline(always)]
  pub fn stall_i_dis2(&self) -> STALL_I_DIS2_R {
    STALL_I_DIS2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - STALL_I_DIS3"]
  #[inline(always)]
  pub fn stall_i_dis3(&self) -> STALL_I_DIS3_R {
    STALL_I_DIS3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - STALL_I_DIS4"]
  #[inline(always)]
  pub fn stall_i_dis4(&self) -> STALL_I_DIS4_R {
    STALL_I_DIS4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - STALL_I_DIS5"]
  #[inline(always)]
  pub fn stall_i_dis5(&self) -> STALL_I_DIS5_R {
    STALL_I_DIS5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - STALL_I_DIS6"]
  #[inline(always)]
  pub fn stall_i_dis6(&self) -> STALL_I_DIS6_R {
    STALL_I_DIS6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - STALL_I_DIS7"]
  #[inline(always)]
  pub fn stall_i_dis7(&self) -> STALL_I_DIS7_R {
    STALL_I_DIS7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - STALL_I_DIS0"]
  #[inline(always)]
  pub fn stall_i_dis0(&mut self) -> STALL_I_DIS0_W {
    STALL_I_DIS0_W { w: self }
  }
  #[doc = "Bit 1 - STALL_I_DIS1"]
  #[inline(always)]
  pub fn stall_i_dis1(&mut self) -> STALL_I_DIS1_W {
    STALL_I_DIS1_W { w: self }
  }
  #[doc = "Bit 2 - STALL_I_DIS2"]
  #[inline(always)]
  pub fn stall_i_dis2(&mut self) -> STALL_I_DIS2_W {
    STALL_I_DIS2_W { w: self }
  }
  #[doc = "Bit 3 - STALL_I_DIS3"]
  #[inline(always)]
  pub fn stall_i_dis3(&mut self) -> STALL_I_DIS3_W {
    STALL_I_DIS3_W { w: self }
  }
  #[doc = "Bit 4 - STALL_I_DIS4"]
  #[inline(always)]
  pub fn stall_i_dis4(&mut self) -> STALL_I_DIS4_W {
    STALL_I_DIS4_W { w: self }
  }
  #[doc = "Bit 5 - STALL_I_DIS5"]
  #[inline(always)]
  pub fn stall_i_dis5(&mut self) -> STALL_I_DIS5_W {
    STALL_I_DIS5_W { w: self }
  }
  #[doc = "Bit 6 - STALL_I_DIS6"]
  #[inline(always)]
  pub fn stall_i_dis6(&mut self) -> STALL_I_DIS6_W {
    STALL_I_DIS6_W { w: self }
  }
  #[doc = "Bit 7 - STALL_I_DIS7"]
  #[inline(always)]
  pub fn stall_i_dis7(&mut self) -> STALL_I_DIS7_W {
    STALL_I_DIS7_W { w: self }
  }
}
