#[doc = "Reader of register EARS"]
pub type R = crate::R<u32, super::EARS>;
#[doc = "Writer for register EARS"]
pub type W = crate::W<u32, super::EARS>;
#[doc = "Register EARS `reset()`'s with value 0"]
impl crate::ResetValue for super::EARS {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0_A {
  #[doc = "0: Disable asynchronous DMA request for channel 0."]
  EDREQ_0_0,
  #[doc = "1: Enable asynchronous DMA request for channel 0."]
  EDREQ_0_1,
}
impl From<EDREQ_0_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_0_A) -> Self {
    match variant {
      EDREQ_0_A::EDREQ_0_0 => false,
      EDREQ_0_A::EDREQ_0_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_0`"]
pub type EDREQ_0_R = crate::R<bool, EDREQ_0_A>;
impl EDREQ_0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_0_A {
    match self.bits {
      false => EDREQ_0_A::EDREQ_0_0,
      true => EDREQ_0_A::EDREQ_0_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_0_0`"]
  #[inline(always)]
  pub fn is_edreq_0_0(&self) -> bool {
    *self == EDREQ_0_A::EDREQ_0_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_0_1`"]
  #[inline(always)]
  pub fn is_edreq_0_1(&self) -> bool {
    *self == EDREQ_0_A::EDREQ_0_1
  }
}
#[doc = "Write proxy for field `EDREQ_0`"]
pub struct EDREQ_0_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 0."]
  #[inline(always)]
  pub fn edreq_0_0(self) -> &'a mut W {
    self.variant(EDREQ_0_A::EDREQ_0_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 0."]
  #[inline(always)]
  pub fn edreq_0_1(self) -> &'a mut W {
    self.variant(EDREQ_0_A::EDREQ_0_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1_A {
  #[doc = "0: Disable asynchronous DMA request for channel 1"]
  EDREQ_1_0,
  #[doc = "1: Enable asynchronous DMA request for channel 1."]
  EDREQ_1_1,
}
impl From<EDREQ_1_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_1_A) -> Self {
    match variant {
      EDREQ_1_A::EDREQ_1_0 => false,
      EDREQ_1_A::EDREQ_1_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_1`"]
pub type EDREQ_1_R = crate::R<bool, EDREQ_1_A>;
impl EDREQ_1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_1_A {
    match self.bits {
      false => EDREQ_1_A::EDREQ_1_0,
      true => EDREQ_1_A::EDREQ_1_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_1_0`"]
  #[inline(always)]
  pub fn is_edreq_1_0(&self) -> bool {
    *self == EDREQ_1_A::EDREQ_1_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_1_1`"]
  #[inline(always)]
  pub fn is_edreq_1_1(&self) -> bool {
    *self == EDREQ_1_A::EDREQ_1_1
  }
}
#[doc = "Write proxy for field `EDREQ_1`"]
pub struct EDREQ_1_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 1"]
  #[inline(always)]
  pub fn edreq_1_0(self) -> &'a mut W {
    self.variant(EDREQ_1_A::EDREQ_1_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 1."]
  #[inline(always)]
  pub fn edreq_1_1(self) -> &'a mut W {
    self.variant(EDREQ_1_A::EDREQ_1_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2_A {
  #[doc = "0: Disable asynchronous DMA request for channel 2."]
  EDREQ_2_0,
  #[doc = "1: Enable asynchronous DMA request for channel 2."]
  EDREQ_2_1,
}
impl From<EDREQ_2_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_2_A) -> Self {
    match variant {
      EDREQ_2_A::EDREQ_2_0 => false,
      EDREQ_2_A::EDREQ_2_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_2`"]
pub type EDREQ_2_R = crate::R<bool, EDREQ_2_A>;
impl EDREQ_2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_2_A {
    match self.bits {
      false => EDREQ_2_A::EDREQ_2_0,
      true => EDREQ_2_A::EDREQ_2_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_2_0`"]
  #[inline(always)]
  pub fn is_edreq_2_0(&self) -> bool {
    *self == EDREQ_2_A::EDREQ_2_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_2_1`"]
  #[inline(always)]
  pub fn is_edreq_2_1(&self) -> bool {
    *self == EDREQ_2_A::EDREQ_2_1
  }
}
#[doc = "Write proxy for field `EDREQ_2`"]
pub struct EDREQ_2_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 2."]
  #[inline(always)]
  pub fn edreq_2_0(self) -> &'a mut W {
    self.variant(EDREQ_2_A::EDREQ_2_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 2."]
  #[inline(always)]
  pub fn edreq_2_1(self) -> &'a mut W {
    self.variant(EDREQ_2_A::EDREQ_2_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3_A {
  #[doc = "0: Disable asynchronous DMA request for channel 3."]
  EDREQ_3_0,
  #[doc = "1: Enable asynchronous DMA request for channel 3."]
  EDREQ_3_1,
}
impl From<EDREQ_3_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_3_A) -> Self {
    match variant {
      EDREQ_3_A::EDREQ_3_0 => false,
      EDREQ_3_A::EDREQ_3_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_3`"]
pub type EDREQ_3_R = crate::R<bool, EDREQ_3_A>;
impl EDREQ_3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_3_A {
    match self.bits {
      false => EDREQ_3_A::EDREQ_3_0,
      true => EDREQ_3_A::EDREQ_3_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_3_0`"]
  #[inline(always)]
  pub fn is_edreq_3_0(&self) -> bool {
    *self == EDREQ_3_A::EDREQ_3_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_3_1`"]
  #[inline(always)]
  pub fn is_edreq_3_1(&self) -> bool {
    *self == EDREQ_3_A::EDREQ_3_1
  }
}
#[doc = "Write proxy for field `EDREQ_3`"]
pub struct EDREQ_3_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 3."]
  #[inline(always)]
  pub fn edreq_3_0(self) -> &'a mut W {
    self.variant(EDREQ_3_A::EDREQ_3_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 3."]
  #[inline(always)]
  pub fn edreq_3_1(self) -> &'a mut W {
    self.variant(EDREQ_3_A::EDREQ_3_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_4_A {
  #[doc = "0: Disable asynchronous DMA request for channel 4."]
  EDREQ_4_0,
  #[doc = "1: Enable asynchronous DMA request for channel 4."]
  EDREQ_4_1,
}
impl From<EDREQ_4_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_4_A) -> Self {
    match variant {
      EDREQ_4_A::EDREQ_4_0 => false,
      EDREQ_4_A::EDREQ_4_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_4`"]
pub type EDREQ_4_R = crate::R<bool, EDREQ_4_A>;
impl EDREQ_4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_4_A {
    match self.bits {
      false => EDREQ_4_A::EDREQ_4_0,
      true => EDREQ_4_A::EDREQ_4_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_4_0`"]
  #[inline(always)]
  pub fn is_edreq_4_0(&self) -> bool {
    *self == EDREQ_4_A::EDREQ_4_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_4_1`"]
  #[inline(always)]
  pub fn is_edreq_4_1(&self) -> bool {
    *self == EDREQ_4_A::EDREQ_4_1
  }
}
#[doc = "Write proxy for field `EDREQ_4`"]
pub struct EDREQ_4_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 4."]
  #[inline(always)]
  pub fn edreq_4_0(self) -> &'a mut W {
    self.variant(EDREQ_4_A::EDREQ_4_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 4."]
  #[inline(always)]
  pub fn edreq_4_1(self) -> &'a mut W {
    self.variant(EDREQ_4_A::EDREQ_4_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_5_A {
  #[doc = "0: Disable asynchronous DMA request for channel 5."]
  EDREQ_5_0,
  #[doc = "1: Enable asynchronous DMA request for channel 5."]
  EDREQ_5_1,
}
impl From<EDREQ_5_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_5_A) -> Self {
    match variant {
      EDREQ_5_A::EDREQ_5_0 => false,
      EDREQ_5_A::EDREQ_5_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_5`"]
pub type EDREQ_5_R = crate::R<bool, EDREQ_5_A>;
impl EDREQ_5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_5_A {
    match self.bits {
      false => EDREQ_5_A::EDREQ_5_0,
      true => EDREQ_5_A::EDREQ_5_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_5_0`"]
  #[inline(always)]
  pub fn is_edreq_5_0(&self) -> bool {
    *self == EDREQ_5_A::EDREQ_5_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_5_1`"]
  #[inline(always)]
  pub fn is_edreq_5_1(&self) -> bool {
    *self == EDREQ_5_A::EDREQ_5_1
  }
}
#[doc = "Write proxy for field `EDREQ_5`"]
pub struct EDREQ_5_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 5."]
  #[inline(always)]
  pub fn edreq_5_0(self) -> &'a mut W {
    self.variant(EDREQ_5_A::EDREQ_5_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 5."]
  #[inline(always)]
  pub fn edreq_5_1(self) -> &'a mut W {
    self.variant(EDREQ_5_A::EDREQ_5_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_6_A {
  #[doc = "0: Disable asynchronous DMA request for channel 6."]
  EDREQ_6_0,
  #[doc = "1: Enable asynchronous DMA request for channel 6."]
  EDREQ_6_1,
}
impl From<EDREQ_6_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_6_A) -> Self {
    match variant {
      EDREQ_6_A::EDREQ_6_0 => false,
      EDREQ_6_A::EDREQ_6_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_6`"]
pub type EDREQ_6_R = crate::R<bool, EDREQ_6_A>;
impl EDREQ_6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_6_A {
    match self.bits {
      false => EDREQ_6_A::EDREQ_6_0,
      true => EDREQ_6_A::EDREQ_6_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_6_0`"]
  #[inline(always)]
  pub fn is_edreq_6_0(&self) -> bool {
    *self == EDREQ_6_A::EDREQ_6_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_6_1`"]
  #[inline(always)]
  pub fn is_edreq_6_1(&self) -> bool {
    *self == EDREQ_6_A::EDREQ_6_1
  }
}
#[doc = "Write proxy for field `EDREQ_6`"]
pub struct EDREQ_6_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_6_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 6."]
  #[inline(always)]
  pub fn edreq_6_0(self) -> &'a mut W {
    self.variant(EDREQ_6_A::EDREQ_6_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 6."]
  #[inline(always)]
  pub fn edreq_6_1(self) -> &'a mut W {
    self.variant(EDREQ_6_A::EDREQ_6_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_7_A {
  #[doc = "0: Disable asynchronous DMA request for channel 7."]
  EDREQ_7_0,
  #[doc = "1: Enable asynchronous DMA request for channel 7."]
  EDREQ_7_1,
}
impl From<EDREQ_7_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_7_A) -> Self {
    match variant {
      EDREQ_7_A::EDREQ_7_0 => false,
      EDREQ_7_A::EDREQ_7_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_7`"]
pub type EDREQ_7_R = crate::R<bool, EDREQ_7_A>;
impl EDREQ_7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_7_A {
    match self.bits {
      false => EDREQ_7_A::EDREQ_7_0,
      true => EDREQ_7_A::EDREQ_7_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_7_0`"]
  #[inline(always)]
  pub fn is_edreq_7_0(&self) -> bool {
    *self == EDREQ_7_A::EDREQ_7_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_7_1`"]
  #[inline(always)]
  pub fn is_edreq_7_1(&self) -> bool {
    *self == EDREQ_7_A::EDREQ_7_1
  }
}
#[doc = "Write proxy for field `EDREQ_7`"]
pub struct EDREQ_7_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_7_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 7."]
  #[inline(always)]
  pub fn edreq_7_0(self) -> &'a mut W {
    self.variant(EDREQ_7_A::EDREQ_7_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 7."]
  #[inline(always)]
  pub fn edreq_7_1(self) -> &'a mut W {
    self.variant(EDREQ_7_A::EDREQ_7_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_8_A {
  #[doc = "0: Disable asynchronous DMA request for channel 8."]
  EDREQ_8_0,
  #[doc = "1: Enable asynchronous DMA request for channel 8."]
  EDREQ_8_1,
}
impl From<EDREQ_8_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_8_A) -> Self {
    match variant {
      EDREQ_8_A::EDREQ_8_0 => false,
      EDREQ_8_A::EDREQ_8_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_8`"]
pub type EDREQ_8_R = crate::R<bool, EDREQ_8_A>;
impl EDREQ_8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_8_A {
    match self.bits {
      false => EDREQ_8_A::EDREQ_8_0,
      true => EDREQ_8_A::EDREQ_8_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_8_0`"]
  #[inline(always)]
  pub fn is_edreq_8_0(&self) -> bool {
    *self == EDREQ_8_A::EDREQ_8_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_8_1`"]
  #[inline(always)]
  pub fn is_edreq_8_1(&self) -> bool {
    *self == EDREQ_8_A::EDREQ_8_1
  }
}
#[doc = "Write proxy for field `EDREQ_8`"]
pub struct EDREQ_8_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_8_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_8_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 8."]
  #[inline(always)]
  pub fn edreq_8_0(self) -> &'a mut W {
    self.variant(EDREQ_8_A::EDREQ_8_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 8."]
  #[inline(always)]
  pub fn edreq_8_1(self) -> &'a mut W {
    self.variant(EDREQ_8_A::EDREQ_8_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_9_A {
  #[doc = "0: Disable asynchronous DMA request for channel 9."]
  EDREQ_9_0,
  #[doc = "1: Enable asynchronous DMA request for channel 9."]
  EDREQ_9_1,
}
impl From<EDREQ_9_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_9_A) -> Self {
    match variant {
      EDREQ_9_A::EDREQ_9_0 => false,
      EDREQ_9_A::EDREQ_9_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_9`"]
pub type EDREQ_9_R = crate::R<bool, EDREQ_9_A>;
impl EDREQ_9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_9_A {
    match self.bits {
      false => EDREQ_9_A::EDREQ_9_0,
      true => EDREQ_9_A::EDREQ_9_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_9_0`"]
  #[inline(always)]
  pub fn is_edreq_9_0(&self) -> bool {
    *self == EDREQ_9_A::EDREQ_9_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_9_1`"]
  #[inline(always)]
  pub fn is_edreq_9_1(&self) -> bool {
    *self == EDREQ_9_A::EDREQ_9_1
  }
}
#[doc = "Write proxy for field `EDREQ_9`"]
pub struct EDREQ_9_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_9_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_9_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 9."]
  #[inline(always)]
  pub fn edreq_9_0(self) -> &'a mut W {
    self.variant(EDREQ_9_A::EDREQ_9_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 9."]
  #[inline(always)]
  pub fn edreq_9_1(self) -> &'a mut W {
    self.variant(EDREQ_9_A::EDREQ_9_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_10_A {
  #[doc = "0: Disable asynchronous DMA request for channel 10."]
  EDREQ_10_0,
  #[doc = "1: Enable asynchronous DMA request for channel 10."]
  EDREQ_10_1,
}
impl From<EDREQ_10_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_10_A) -> Self {
    match variant {
      EDREQ_10_A::EDREQ_10_0 => false,
      EDREQ_10_A::EDREQ_10_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_10`"]
pub type EDREQ_10_R = crate::R<bool, EDREQ_10_A>;
impl EDREQ_10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_10_A {
    match self.bits {
      false => EDREQ_10_A::EDREQ_10_0,
      true => EDREQ_10_A::EDREQ_10_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_10_0`"]
  #[inline(always)]
  pub fn is_edreq_10_0(&self) -> bool {
    *self == EDREQ_10_A::EDREQ_10_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_10_1`"]
  #[inline(always)]
  pub fn is_edreq_10_1(&self) -> bool {
    *self == EDREQ_10_A::EDREQ_10_1
  }
}
#[doc = "Write proxy for field `EDREQ_10`"]
pub struct EDREQ_10_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_10_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_10_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 10."]
  #[inline(always)]
  pub fn edreq_10_0(self) -> &'a mut W {
    self.variant(EDREQ_10_A::EDREQ_10_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 10."]
  #[inline(always)]
  pub fn edreq_10_1(self) -> &'a mut W {
    self.variant(EDREQ_10_A::EDREQ_10_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
    self.w
  }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_11_A {
  #[doc = "0: Disable asynchronous DMA request for channel 11."]
  EDREQ_11_0,
  #[doc = "1: Enable asynchronous DMA request for channel 11."]
  EDREQ_11_1,
}
impl From<EDREQ_11_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_11_A) -> Self {
    match variant {
      EDREQ_11_A::EDREQ_11_0 => false,
      EDREQ_11_A::EDREQ_11_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_11`"]
pub type EDREQ_11_R = crate::R<bool, EDREQ_11_A>;
impl EDREQ_11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_11_A {
    match self.bits {
      false => EDREQ_11_A::EDREQ_11_0,
      true => EDREQ_11_A::EDREQ_11_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_11_0`"]
  #[inline(always)]
  pub fn is_edreq_11_0(&self) -> bool {
    *self == EDREQ_11_A::EDREQ_11_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_11_1`"]
  #[inline(always)]
  pub fn is_edreq_11_1(&self) -> bool {
    *self == EDREQ_11_A::EDREQ_11_1
  }
}
#[doc = "Write proxy for field `EDREQ_11`"]
pub struct EDREQ_11_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_11_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_11_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 11."]
  #[inline(always)]
  pub fn edreq_11_0(self) -> &'a mut W {
    self.variant(EDREQ_11_A::EDREQ_11_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 11."]
  #[inline(always)]
  pub fn edreq_11_1(self) -> &'a mut W {
    self.variant(EDREQ_11_A::EDREQ_11_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
    self.w
  }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_12_A {
  #[doc = "0: Disable asynchronous DMA request for channel 12."]
  EDREQ_12_0,
  #[doc = "1: Enable asynchronous DMA request for channel 12."]
  EDREQ_12_1,
}
impl From<EDREQ_12_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_12_A) -> Self {
    match variant {
      EDREQ_12_A::EDREQ_12_0 => false,
      EDREQ_12_A::EDREQ_12_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_12`"]
pub type EDREQ_12_R = crate::R<bool, EDREQ_12_A>;
impl EDREQ_12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_12_A {
    match self.bits {
      false => EDREQ_12_A::EDREQ_12_0,
      true => EDREQ_12_A::EDREQ_12_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_12_0`"]
  #[inline(always)]
  pub fn is_edreq_12_0(&self) -> bool {
    *self == EDREQ_12_A::EDREQ_12_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_12_1`"]
  #[inline(always)]
  pub fn is_edreq_12_1(&self) -> bool {
    *self == EDREQ_12_A::EDREQ_12_1
  }
}
#[doc = "Write proxy for field `EDREQ_12`"]
pub struct EDREQ_12_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_12_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_12_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 12."]
  #[inline(always)]
  pub fn edreq_12_0(self) -> &'a mut W {
    self.variant(EDREQ_12_A::EDREQ_12_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 12."]
  #[inline(always)]
  pub fn edreq_12_1(self) -> &'a mut W {
    self.variant(EDREQ_12_A::EDREQ_12_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_13_A {
  #[doc = "0: Disable asynchronous DMA request for channel 13."]
  EDREQ_13_0,
  #[doc = "1: Enable asynchronous DMA request for channel 13."]
  EDREQ_13_1,
}
impl From<EDREQ_13_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_13_A) -> Self {
    match variant {
      EDREQ_13_A::EDREQ_13_0 => false,
      EDREQ_13_A::EDREQ_13_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_13`"]
pub type EDREQ_13_R = crate::R<bool, EDREQ_13_A>;
impl EDREQ_13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_13_A {
    match self.bits {
      false => EDREQ_13_A::EDREQ_13_0,
      true => EDREQ_13_A::EDREQ_13_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_13_0`"]
  #[inline(always)]
  pub fn is_edreq_13_0(&self) -> bool {
    *self == EDREQ_13_A::EDREQ_13_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_13_1`"]
  #[inline(always)]
  pub fn is_edreq_13_1(&self) -> bool {
    *self == EDREQ_13_A::EDREQ_13_1
  }
}
#[doc = "Write proxy for field `EDREQ_13`"]
pub struct EDREQ_13_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_13_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_13_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 13."]
  #[inline(always)]
  pub fn edreq_13_0(self) -> &'a mut W {
    self.variant(EDREQ_13_A::EDREQ_13_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 13."]
  #[inline(always)]
  pub fn edreq_13_1(self) -> &'a mut W {
    self.variant(EDREQ_13_A::EDREQ_13_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
    self.w
  }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_14_A {
  #[doc = "0: Disable asynchronous DMA request for channel 14."]
  EDREQ_14_0,
  #[doc = "1: Enable asynchronous DMA request for channel 14."]
  EDREQ_14_1,
}
impl From<EDREQ_14_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_14_A) -> Self {
    match variant {
      EDREQ_14_A::EDREQ_14_0 => false,
      EDREQ_14_A::EDREQ_14_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_14`"]
pub type EDREQ_14_R = crate::R<bool, EDREQ_14_A>;
impl EDREQ_14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_14_A {
    match self.bits {
      false => EDREQ_14_A::EDREQ_14_0,
      true => EDREQ_14_A::EDREQ_14_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_14_0`"]
  #[inline(always)]
  pub fn is_edreq_14_0(&self) -> bool {
    *self == EDREQ_14_A::EDREQ_14_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_14_1`"]
  #[inline(always)]
  pub fn is_edreq_14_1(&self) -> bool {
    *self == EDREQ_14_A::EDREQ_14_1
  }
}
#[doc = "Write proxy for field `EDREQ_14`"]
pub struct EDREQ_14_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_14_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_14_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 14."]
  #[inline(always)]
  pub fn edreq_14_0(self) -> &'a mut W {
    self.variant(EDREQ_14_A::EDREQ_14_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 14."]
  #[inline(always)]
  pub fn edreq_14_1(self) -> &'a mut W {
    self.variant(EDREQ_14_A::EDREQ_14_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_15_A {
  #[doc = "0: Disable asynchronous DMA request for channel 15."]
  EDREQ_15_0,
  #[doc = "1: Enable asynchronous DMA request for channel 15."]
  EDREQ_15_1,
}
impl From<EDREQ_15_A> for bool {
  #[inline(always)]
  fn from(variant: EDREQ_15_A) -> Self {
    match variant {
      EDREQ_15_A::EDREQ_15_0 => false,
      EDREQ_15_A::EDREQ_15_1 => true,
    }
  }
}
#[doc = "Reader of field `EDREQ_15`"]
pub type EDREQ_15_R = crate::R<bool, EDREQ_15_A>;
impl EDREQ_15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EDREQ_15_A {
    match self.bits {
      false => EDREQ_15_A::EDREQ_15_0,
      true => EDREQ_15_A::EDREQ_15_1,
    }
  }
  #[doc = "Checks if the value of the field is `EDREQ_15_0`"]
  #[inline(always)]
  pub fn is_edreq_15_0(&self) -> bool {
    *self == EDREQ_15_A::EDREQ_15_0
  }
  #[doc = "Checks if the value of the field is `EDREQ_15_1`"]
  #[inline(always)]
  pub fn is_edreq_15_1(&self) -> bool {
    *self == EDREQ_15_A::EDREQ_15_1
  }
}
#[doc = "Write proxy for field `EDREQ_15`"]
pub struct EDREQ_15_W<'a> {
  w: &'a mut W,
}
impl<'a> EDREQ_15_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EDREQ_15_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable asynchronous DMA request for channel 15."]
  #[inline(always)]
  pub fn edreq_15_0(self) -> &'a mut W {
    self.variant(EDREQ_15_A::EDREQ_15_0)
  }
  #[doc = "Enable asynchronous DMA request for channel 15."]
  #[inline(always)]
  pub fn edreq_15_1(self) -> &'a mut W {
    self.variant(EDREQ_15_A::EDREQ_15_1)
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
impl R {
  #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
  #[inline(always)]
  pub fn edreq_0(&self) -> EDREQ_0_R {
    EDREQ_0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
  #[inline(always)]
  pub fn edreq_1(&self) -> EDREQ_1_R {
    EDREQ_1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
  #[inline(always)]
  pub fn edreq_2(&self) -> EDREQ_2_R {
    EDREQ_2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
  #[inline(always)]
  pub fn edreq_3(&self) -> EDREQ_3_R {
    EDREQ_3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
  #[inline(always)]
  pub fn edreq_4(&self) -> EDREQ_4_R {
    EDREQ_4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
  #[inline(always)]
  pub fn edreq_5(&self) -> EDREQ_5_R {
    EDREQ_5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
  #[inline(always)]
  pub fn edreq_6(&self) -> EDREQ_6_R {
    EDREQ_6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
  #[inline(always)]
  pub fn edreq_7(&self) -> EDREQ_7_R {
    EDREQ_7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
  #[inline(always)]
  pub fn edreq_8(&self) -> EDREQ_8_R {
    EDREQ_8_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
  #[inline(always)]
  pub fn edreq_9(&self) -> EDREQ_9_R {
    EDREQ_9_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
  #[inline(always)]
  pub fn edreq_10(&self) -> EDREQ_10_R {
    EDREQ_10_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
  #[inline(always)]
  pub fn edreq_11(&self) -> EDREQ_11_R {
    EDREQ_11_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
  #[inline(always)]
  pub fn edreq_12(&self) -> EDREQ_12_R {
    EDREQ_12_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
  #[inline(always)]
  pub fn edreq_13(&self) -> EDREQ_13_R {
    EDREQ_13_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
  #[inline(always)]
  pub fn edreq_14(&self) -> EDREQ_14_R {
    EDREQ_14_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
  #[inline(always)]
  pub fn edreq_15(&self) -> EDREQ_15_R {
    EDREQ_15_R::new(((self.bits >> 15) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
  #[inline(always)]
  pub fn edreq_0(&mut self) -> EDREQ_0_W {
    EDREQ_0_W { w: self }
  }
  #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
  #[inline(always)]
  pub fn edreq_1(&mut self) -> EDREQ_1_W {
    EDREQ_1_W { w: self }
  }
  #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
  #[inline(always)]
  pub fn edreq_2(&mut self) -> EDREQ_2_W {
    EDREQ_2_W { w: self }
  }
  #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
  #[inline(always)]
  pub fn edreq_3(&mut self) -> EDREQ_3_W {
    EDREQ_3_W { w: self }
  }
  #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
  #[inline(always)]
  pub fn edreq_4(&mut self) -> EDREQ_4_W {
    EDREQ_4_W { w: self }
  }
  #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
  #[inline(always)]
  pub fn edreq_5(&mut self) -> EDREQ_5_W {
    EDREQ_5_W { w: self }
  }
  #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
  #[inline(always)]
  pub fn edreq_6(&mut self) -> EDREQ_6_W {
    EDREQ_6_W { w: self }
  }
  #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
  #[inline(always)]
  pub fn edreq_7(&mut self) -> EDREQ_7_W {
    EDREQ_7_W { w: self }
  }
  #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
  #[inline(always)]
  pub fn edreq_8(&mut self) -> EDREQ_8_W {
    EDREQ_8_W { w: self }
  }
  #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
  #[inline(always)]
  pub fn edreq_9(&mut self) -> EDREQ_9_W {
    EDREQ_9_W { w: self }
  }
  #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
  #[inline(always)]
  pub fn edreq_10(&mut self) -> EDREQ_10_W {
    EDREQ_10_W { w: self }
  }
  #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
  #[inline(always)]
  pub fn edreq_11(&mut self) -> EDREQ_11_W {
    EDREQ_11_W { w: self }
  }
  #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
  #[inline(always)]
  pub fn edreq_12(&mut self) -> EDREQ_12_W {
    EDREQ_12_W { w: self }
  }
  #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
  #[inline(always)]
  pub fn edreq_13(&mut self) -> EDREQ_13_W {
    EDREQ_13_W { w: self }
  }
  #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
  #[inline(always)]
  pub fn edreq_14(&mut self) -> EDREQ_14_W {
    EDREQ_14_W { w: self }
  }
  #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
  #[inline(always)]
  pub fn edreq_15(&mut self) -> EDREQ_15_W {
    EDREQ_15_W { w: self }
  }
}
