#[doc = "Reader of register PMC"]
pub type R = crate::R<u32, super::PMC>;
#[doc = "Writer for register PMC"]
pub type W = crate::W<u32, super::PMC>;
#[doc = "Register PMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC0_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC0_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC0_1,
}
impl From<WUPMC0_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC0_A) -> Self {
    match variant {
      WUPMC0_A::WUPMC0_0 => false,
      WUPMC0_A::WUPMC0_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC0`"]
pub type WUPMC0_R = crate::R<bool, WUPMC0_A>;
impl WUPMC0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC0_A {
    match self.bits {
      false => WUPMC0_A::WUPMC0_0,
      true => WUPMC0_A::WUPMC0_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC0_0`"]
  #[inline(always)]
  pub fn is_wupmc0_0(&self) -> bool {
    *self == WUPMC0_A::WUPMC0_0
  }
  #[doc = "Checks if the value of the field is `WUPMC0_1`"]
  #[inline(always)]
  pub fn is_wupmc0_1(&self) -> bool {
    *self == WUPMC0_A::WUPMC0_1
  }
}
#[doc = "Write proxy for field `WUPMC0`"]
pub struct WUPMC0_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc0_0(self) -> &'a mut W {
    self.variant(WUPMC0_A::WUPMC0_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc0_1(self) -> &'a mut W {
    self.variant(WUPMC0_A::WUPMC0_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC1_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC1_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC1_1,
}
impl From<WUPMC1_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC1_A) -> Self {
    match variant {
      WUPMC1_A::WUPMC1_0 => false,
      WUPMC1_A::WUPMC1_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC1`"]
pub type WUPMC1_R = crate::R<bool, WUPMC1_A>;
impl WUPMC1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC1_A {
    match self.bits {
      false => WUPMC1_A::WUPMC1_0,
      true => WUPMC1_A::WUPMC1_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC1_0`"]
  #[inline(always)]
  pub fn is_wupmc1_0(&self) -> bool {
    *self == WUPMC1_A::WUPMC1_0
  }
  #[doc = "Checks if the value of the field is `WUPMC1_1`"]
  #[inline(always)]
  pub fn is_wupmc1_1(&self) -> bool {
    *self == WUPMC1_A::WUPMC1_1
  }
}
#[doc = "Write proxy for field `WUPMC1`"]
pub struct WUPMC1_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc1_0(self) -> &'a mut W {
    self.variant(WUPMC1_A::WUPMC1_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc1_1(self) -> &'a mut W {
    self.variant(WUPMC1_A::WUPMC1_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC2_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC2_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC2_1,
}
impl From<WUPMC2_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC2_A) -> Self {
    match variant {
      WUPMC2_A::WUPMC2_0 => false,
      WUPMC2_A::WUPMC2_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC2`"]
pub type WUPMC2_R = crate::R<bool, WUPMC2_A>;
impl WUPMC2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC2_A {
    match self.bits {
      false => WUPMC2_A::WUPMC2_0,
      true => WUPMC2_A::WUPMC2_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC2_0`"]
  #[inline(always)]
  pub fn is_wupmc2_0(&self) -> bool {
    *self == WUPMC2_A::WUPMC2_0
  }
  #[doc = "Checks if the value of the field is `WUPMC2_1`"]
  #[inline(always)]
  pub fn is_wupmc2_1(&self) -> bool {
    *self == WUPMC2_A::WUPMC2_1
  }
}
#[doc = "Write proxy for field `WUPMC2`"]
pub struct WUPMC2_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc2_0(self) -> &'a mut W {
    self.variant(WUPMC2_A::WUPMC2_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc2_1(self) -> &'a mut W {
    self.variant(WUPMC2_A::WUPMC2_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC3_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC3_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC3_1,
}
impl From<WUPMC3_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC3_A) -> Self {
    match variant {
      WUPMC3_A::WUPMC3_0 => false,
      WUPMC3_A::WUPMC3_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC3`"]
pub type WUPMC3_R = crate::R<bool, WUPMC3_A>;
impl WUPMC3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC3_A {
    match self.bits {
      false => WUPMC3_A::WUPMC3_0,
      true => WUPMC3_A::WUPMC3_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC3_0`"]
  #[inline(always)]
  pub fn is_wupmc3_0(&self) -> bool {
    *self == WUPMC3_A::WUPMC3_0
  }
  #[doc = "Checks if the value of the field is `WUPMC3_1`"]
  #[inline(always)]
  pub fn is_wupmc3_1(&self) -> bool {
    *self == WUPMC3_A::WUPMC3_1
  }
}
#[doc = "Write proxy for field `WUPMC3`"]
pub struct WUPMC3_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc3_0(self) -> &'a mut W {
    self.variant(WUPMC3_A::WUPMC3_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc3_1(self) -> &'a mut W {
    self.variant(WUPMC3_A::WUPMC3_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC4_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC4_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC4_1,
}
impl From<WUPMC4_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC4_A) -> Self {
    match variant {
      WUPMC4_A::WUPMC4_0 => false,
      WUPMC4_A::WUPMC4_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC4`"]
pub type WUPMC4_R = crate::R<bool, WUPMC4_A>;
impl WUPMC4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC4_A {
    match self.bits {
      false => WUPMC4_A::WUPMC4_0,
      true => WUPMC4_A::WUPMC4_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC4_0`"]
  #[inline(always)]
  pub fn is_wupmc4_0(&self) -> bool {
    *self == WUPMC4_A::WUPMC4_0
  }
  #[doc = "Checks if the value of the field is `WUPMC4_1`"]
  #[inline(always)]
  pub fn is_wupmc4_1(&self) -> bool {
    *self == WUPMC4_A::WUPMC4_1
  }
}
#[doc = "Write proxy for field `WUPMC4`"]
pub struct WUPMC4_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc4_0(self) -> &'a mut W {
    self.variant(WUPMC4_A::WUPMC4_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc4_1(self) -> &'a mut W {
    self.variant(WUPMC4_A::WUPMC4_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC5_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC5_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC5_1,
}
impl From<WUPMC5_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC5_A) -> Self {
    match variant {
      WUPMC5_A::WUPMC5_0 => false,
      WUPMC5_A::WUPMC5_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC5`"]
pub type WUPMC5_R = crate::R<bool, WUPMC5_A>;
impl WUPMC5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC5_A {
    match self.bits {
      false => WUPMC5_A::WUPMC5_0,
      true => WUPMC5_A::WUPMC5_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC5_0`"]
  #[inline(always)]
  pub fn is_wupmc5_0(&self) -> bool {
    *self == WUPMC5_A::WUPMC5_0
  }
  #[doc = "Checks if the value of the field is `WUPMC5_1`"]
  #[inline(always)]
  pub fn is_wupmc5_1(&self) -> bool {
    *self == WUPMC5_A::WUPMC5_1
  }
}
#[doc = "Write proxy for field `WUPMC5`"]
pub struct WUPMC5_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc5_0(self) -> &'a mut W {
    self.variant(WUPMC5_A::WUPMC5_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc5_1(self) -> &'a mut W {
    self.variant(WUPMC5_A::WUPMC5_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC6_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC6_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC6_1,
}
impl From<WUPMC6_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC6_A) -> Self {
    match variant {
      WUPMC6_A::WUPMC6_0 => false,
      WUPMC6_A::WUPMC6_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC6`"]
pub type WUPMC6_R = crate::R<bool, WUPMC6_A>;
impl WUPMC6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC6_A {
    match self.bits {
      false => WUPMC6_A::WUPMC6_0,
      true => WUPMC6_A::WUPMC6_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC6_0`"]
  #[inline(always)]
  pub fn is_wupmc6_0(&self) -> bool {
    *self == WUPMC6_A::WUPMC6_0
  }
  #[doc = "Checks if the value of the field is `WUPMC6_1`"]
  #[inline(always)]
  pub fn is_wupmc6_1(&self) -> bool {
    *self == WUPMC6_A::WUPMC6_1
  }
}
#[doc = "Write proxy for field `WUPMC6`"]
pub struct WUPMC6_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC6_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc6_0(self) -> &'a mut W {
    self.variant(WUPMC6_A::WUPMC6_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc6_1(self) -> &'a mut W {
    self.variant(WUPMC6_A::WUPMC6_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC7_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC7_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC7_1,
}
impl From<WUPMC7_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC7_A) -> Self {
    match variant {
      WUPMC7_A::WUPMC7_0 => false,
      WUPMC7_A::WUPMC7_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC7`"]
pub type WUPMC7_R = crate::R<bool, WUPMC7_A>;
impl WUPMC7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC7_A {
    match self.bits {
      false => WUPMC7_A::WUPMC7_0,
      true => WUPMC7_A::WUPMC7_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC7_0`"]
  #[inline(always)]
  pub fn is_wupmc7_0(&self) -> bool {
    *self == WUPMC7_A::WUPMC7_0
  }
  #[doc = "Checks if the value of the field is `WUPMC7_1`"]
  #[inline(always)]
  pub fn is_wupmc7_1(&self) -> bool {
    *self == WUPMC7_A::WUPMC7_1
  }
}
#[doc = "Write proxy for field `WUPMC7`"]
pub struct WUPMC7_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC7_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc7_0(self) -> &'a mut W {
    self.variant(WUPMC7_A::WUPMC7_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc7_1(self) -> &'a mut W {
    self.variant(WUPMC7_A::WUPMC7_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC8_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC8_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC8_1,
}
impl From<WUPMC8_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC8_A) -> Self {
    match variant {
      WUPMC8_A::WUPMC8_0 => false,
      WUPMC8_A::WUPMC8_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC8`"]
pub type WUPMC8_R = crate::R<bool, WUPMC8_A>;
impl WUPMC8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC8_A {
    match self.bits {
      false => WUPMC8_A::WUPMC8_0,
      true => WUPMC8_A::WUPMC8_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC8_0`"]
  #[inline(always)]
  pub fn is_wupmc8_0(&self) -> bool {
    *self == WUPMC8_A::WUPMC8_0
  }
  #[doc = "Checks if the value of the field is `WUPMC8_1`"]
  #[inline(always)]
  pub fn is_wupmc8_1(&self) -> bool {
    *self == WUPMC8_A::WUPMC8_1
  }
}
#[doc = "Write proxy for field `WUPMC8`"]
pub struct WUPMC8_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC8_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC8_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc8_0(self) -> &'a mut W {
    self.variant(WUPMC8_A::WUPMC8_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc8_1(self) -> &'a mut W {
    self.variant(WUPMC8_A::WUPMC8_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC9_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC9_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC9_1,
}
impl From<WUPMC9_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC9_A) -> Self {
    match variant {
      WUPMC9_A::WUPMC9_0 => false,
      WUPMC9_A::WUPMC9_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC9`"]
pub type WUPMC9_R = crate::R<bool, WUPMC9_A>;
impl WUPMC9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC9_A {
    match self.bits {
      false => WUPMC9_A::WUPMC9_0,
      true => WUPMC9_A::WUPMC9_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC9_0`"]
  #[inline(always)]
  pub fn is_wupmc9_0(&self) -> bool {
    *self == WUPMC9_A::WUPMC9_0
  }
  #[doc = "Checks if the value of the field is `WUPMC9_1`"]
  #[inline(always)]
  pub fn is_wupmc9_1(&self) -> bool {
    *self == WUPMC9_A::WUPMC9_1
  }
}
#[doc = "Write proxy for field `WUPMC9`"]
pub struct WUPMC9_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC9_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC9_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc9_0(self) -> &'a mut W {
    self.variant(WUPMC9_A::WUPMC9_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc9_1(self) -> &'a mut W {
    self.variant(WUPMC9_A::WUPMC9_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC10_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC10_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC10_1,
}
impl From<WUPMC10_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC10_A) -> Self {
    match variant {
      WUPMC10_A::WUPMC10_0 => false,
      WUPMC10_A::WUPMC10_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC10`"]
pub type WUPMC10_R = crate::R<bool, WUPMC10_A>;
impl WUPMC10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC10_A {
    match self.bits {
      false => WUPMC10_A::WUPMC10_0,
      true => WUPMC10_A::WUPMC10_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC10_0`"]
  #[inline(always)]
  pub fn is_wupmc10_0(&self) -> bool {
    *self == WUPMC10_A::WUPMC10_0
  }
  #[doc = "Checks if the value of the field is `WUPMC10_1`"]
  #[inline(always)]
  pub fn is_wupmc10_1(&self) -> bool {
    *self == WUPMC10_A::WUPMC10_1
  }
}
#[doc = "Write proxy for field `WUPMC10`"]
pub struct WUPMC10_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC10_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC10_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc10_0(self) -> &'a mut W {
    self.variant(WUPMC10_A::WUPMC10_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc10_1(self) -> &'a mut W {
    self.variant(WUPMC10_A::WUPMC10_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC11_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC11_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC11_1,
}
impl From<WUPMC11_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC11_A) -> Self {
    match variant {
      WUPMC11_A::WUPMC11_0 => false,
      WUPMC11_A::WUPMC11_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC11`"]
pub type WUPMC11_R = crate::R<bool, WUPMC11_A>;
impl WUPMC11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC11_A {
    match self.bits {
      false => WUPMC11_A::WUPMC11_0,
      true => WUPMC11_A::WUPMC11_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC11_0`"]
  #[inline(always)]
  pub fn is_wupmc11_0(&self) -> bool {
    *self == WUPMC11_A::WUPMC11_0
  }
  #[doc = "Checks if the value of the field is `WUPMC11_1`"]
  #[inline(always)]
  pub fn is_wupmc11_1(&self) -> bool {
    *self == WUPMC11_A::WUPMC11_1
  }
}
#[doc = "Write proxy for field `WUPMC11`"]
pub struct WUPMC11_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC11_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC11_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc11_0(self) -> &'a mut W {
    self.variant(WUPMC11_A::WUPMC11_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc11_1(self) -> &'a mut W {
    self.variant(WUPMC11_A::WUPMC11_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC12_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC12_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC12_1,
}
impl From<WUPMC12_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC12_A) -> Self {
    match variant {
      WUPMC12_A::WUPMC12_0 => false,
      WUPMC12_A::WUPMC12_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC12`"]
pub type WUPMC12_R = crate::R<bool, WUPMC12_A>;
impl WUPMC12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC12_A {
    match self.bits {
      false => WUPMC12_A::WUPMC12_0,
      true => WUPMC12_A::WUPMC12_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC12_0`"]
  #[inline(always)]
  pub fn is_wupmc12_0(&self) -> bool {
    *self == WUPMC12_A::WUPMC12_0
  }
  #[doc = "Checks if the value of the field is `WUPMC12_1`"]
  #[inline(always)]
  pub fn is_wupmc12_1(&self) -> bool {
    *self == WUPMC12_A::WUPMC12_1
  }
}
#[doc = "Write proxy for field `WUPMC12`"]
pub struct WUPMC12_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC12_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC12_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc12_0(self) -> &'a mut W {
    self.variant(WUPMC12_A::WUPMC12_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc12_1(self) -> &'a mut W {
    self.variant(WUPMC12_A::WUPMC12_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC13_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC13_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC13_1,
}
impl From<WUPMC13_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC13_A) -> Self {
    match variant {
      WUPMC13_A::WUPMC13_0 => false,
      WUPMC13_A::WUPMC13_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC13`"]
pub type WUPMC13_R = crate::R<bool, WUPMC13_A>;
impl WUPMC13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC13_A {
    match self.bits {
      false => WUPMC13_A::WUPMC13_0,
      true => WUPMC13_A::WUPMC13_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC13_0`"]
  #[inline(always)]
  pub fn is_wupmc13_0(&self) -> bool {
    *self == WUPMC13_A::WUPMC13_0
  }
  #[doc = "Checks if the value of the field is `WUPMC13_1`"]
  #[inline(always)]
  pub fn is_wupmc13_1(&self) -> bool {
    *self == WUPMC13_A::WUPMC13_1
  }
}
#[doc = "Write proxy for field `WUPMC13`"]
pub struct WUPMC13_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC13_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC13_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc13_0(self) -> &'a mut W {
    self.variant(WUPMC13_A::WUPMC13_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc13_1(self) -> &'a mut W {
    self.variant(WUPMC13_A::WUPMC13_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC14_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC14_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC14_1,
}
impl From<WUPMC14_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC14_A) -> Self {
    match variant {
      WUPMC14_A::WUPMC14_0 => false,
      WUPMC14_A::WUPMC14_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC14`"]
pub type WUPMC14_R = crate::R<bool, WUPMC14_A>;
impl WUPMC14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC14_A {
    match self.bits {
      false => WUPMC14_A::WUPMC14_0,
      true => WUPMC14_A::WUPMC14_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC14_0`"]
  #[inline(always)]
  pub fn is_wupmc14_0(&self) -> bool {
    *self == WUPMC14_A::WUPMC14_0
  }
  #[doc = "Checks if the value of the field is `WUPMC14_1`"]
  #[inline(always)]
  pub fn is_wupmc14_1(&self) -> bool {
    *self == WUPMC14_A::WUPMC14_1
  }
}
#[doc = "Write proxy for field `WUPMC14`"]
pub struct WUPMC14_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC14_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC14_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc14_0(self) -> &'a mut W {
    self.variant(WUPMC14_A::WUPMC14_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc14_1(self) -> &'a mut W {
    self.variant(WUPMC14_A::WUPMC14_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC15_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC15_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC15_1,
}
impl From<WUPMC15_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC15_A) -> Self {
    match variant {
      WUPMC15_A::WUPMC15_0 => false,
      WUPMC15_A::WUPMC15_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC15`"]
pub type WUPMC15_R = crate::R<bool, WUPMC15_A>;
impl WUPMC15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC15_A {
    match self.bits {
      false => WUPMC15_A::WUPMC15_0,
      true => WUPMC15_A::WUPMC15_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC15_0`"]
  #[inline(always)]
  pub fn is_wupmc15_0(&self) -> bool {
    *self == WUPMC15_A::WUPMC15_0
  }
  #[doc = "Checks if the value of the field is `WUPMC15_1`"]
  #[inline(always)]
  pub fn is_wupmc15_1(&self) -> bool {
    *self == WUPMC15_A::WUPMC15_1
  }
}
#[doc = "Write proxy for field `WUPMC15`"]
pub struct WUPMC15_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC15_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC15_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc15_0(self) -> &'a mut W {
    self.variant(WUPMC15_A::WUPMC15_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc15_1(self) -> &'a mut W {
    self.variant(WUPMC15_A::WUPMC15_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC16_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC16_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC16_1,
}
impl From<WUPMC16_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC16_A) -> Self {
    match variant {
      WUPMC16_A::WUPMC16_0 => false,
      WUPMC16_A::WUPMC16_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC16`"]
pub type WUPMC16_R = crate::R<bool, WUPMC16_A>;
impl WUPMC16_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC16_A {
    match self.bits {
      false => WUPMC16_A::WUPMC16_0,
      true => WUPMC16_A::WUPMC16_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC16_0`"]
  #[inline(always)]
  pub fn is_wupmc16_0(&self) -> bool {
    *self == WUPMC16_A::WUPMC16_0
  }
  #[doc = "Checks if the value of the field is `WUPMC16_1`"]
  #[inline(always)]
  pub fn is_wupmc16_1(&self) -> bool {
    *self == WUPMC16_A::WUPMC16_1
  }
}
#[doc = "Write proxy for field `WUPMC16`"]
pub struct WUPMC16_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC16_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC16_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc16_0(self) -> &'a mut W {
    self.variant(WUPMC16_A::WUPMC16_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc16_1(self) -> &'a mut W {
    self.variant(WUPMC16_A::WUPMC16_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC17_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC17_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC17_1,
}
impl From<WUPMC17_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC17_A) -> Self {
    match variant {
      WUPMC17_A::WUPMC17_0 => false,
      WUPMC17_A::WUPMC17_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC17`"]
pub type WUPMC17_R = crate::R<bool, WUPMC17_A>;
impl WUPMC17_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC17_A {
    match self.bits {
      false => WUPMC17_A::WUPMC17_0,
      true => WUPMC17_A::WUPMC17_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC17_0`"]
  #[inline(always)]
  pub fn is_wupmc17_0(&self) -> bool {
    *self == WUPMC17_A::WUPMC17_0
  }
  #[doc = "Checks if the value of the field is `WUPMC17_1`"]
  #[inline(always)]
  pub fn is_wupmc17_1(&self) -> bool {
    *self == WUPMC17_A::WUPMC17_1
  }
}
#[doc = "Write proxy for field `WUPMC17`"]
pub struct WUPMC17_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC17_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC17_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc17_0(self) -> &'a mut W {
    self.variant(WUPMC17_A::WUPMC17_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc17_1(self) -> &'a mut W {
    self.variant(WUPMC17_A::WUPMC17_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC18_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC18_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC18_1,
}
impl From<WUPMC18_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC18_A) -> Self {
    match variant {
      WUPMC18_A::WUPMC18_0 => false,
      WUPMC18_A::WUPMC18_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC18`"]
pub type WUPMC18_R = crate::R<bool, WUPMC18_A>;
impl WUPMC18_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC18_A {
    match self.bits {
      false => WUPMC18_A::WUPMC18_0,
      true => WUPMC18_A::WUPMC18_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC18_0`"]
  #[inline(always)]
  pub fn is_wupmc18_0(&self) -> bool {
    *self == WUPMC18_A::WUPMC18_0
  }
  #[doc = "Checks if the value of the field is `WUPMC18_1`"]
  #[inline(always)]
  pub fn is_wupmc18_1(&self) -> bool {
    *self == WUPMC18_A::WUPMC18_1
  }
}
#[doc = "Write proxy for field `WUPMC18`"]
pub struct WUPMC18_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC18_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC18_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc18_0(self) -> &'a mut W {
    self.variant(WUPMC18_A::WUPMC18_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc18_1(self) -> &'a mut W {
    self.variant(WUPMC18_A::WUPMC18_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC19_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC19_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC19_1,
}
impl From<WUPMC19_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC19_A) -> Self {
    match variant {
      WUPMC19_A::WUPMC19_0 => false,
      WUPMC19_A::WUPMC19_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC19`"]
pub type WUPMC19_R = crate::R<bool, WUPMC19_A>;
impl WUPMC19_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC19_A {
    match self.bits {
      false => WUPMC19_A::WUPMC19_0,
      true => WUPMC19_A::WUPMC19_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC19_0`"]
  #[inline(always)]
  pub fn is_wupmc19_0(&self) -> bool {
    *self == WUPMC19_A::WUPMC19_0
  }
  #[doc = "Checks if the value of the field is `WUPMC19_1`"]
  #[inline(always)]
  pub fn is_wupmc19_1(&self) -> bool {
    *self == WUPMC19_A::WUPMC19_1
  }
}
#[doc = "Write proxy for field `WUPMC19`"]
pub struct WUPMC19_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC19_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC19_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc19_0(self) -> &'a mut W {
    self.variant(WUPMC19_A::WUPMC19_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc19_1(self) -> &'a mut W {
    self.variant(WUPMC19_A::WUPMC19_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
    self.w
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC20_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC20_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC20_1,
}
impl From<WUPMC20_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC20_A) -> Self {
    match variant {
      WUPMC20_A::WUPMC20_0 => false,
      WUPMC20_A::WUPMC20_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC20`"]
pub type WUPMC20_R = crate::R<bool, WUPMC20_A>;
impl WUPMC20_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC20_A {
    match self.bits {
      false => WUPMC20_A::WUPMC20_0,
      true => WUPMC20_A::WUPMC20_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC20_0`"]
  #[inline(always)]
  pub fn is_wupmc20_0(&self) -> bool {
    *self == WUPMC20_A::WUPMC20_0
  }
  #[doc = "Checks if the value of the field is `WUPMC20_1`"]
  #[inline(always)]
  pub fn is_wupmc20_1(&self) -> bool {
    *self == WUPMC20_A::WUPMC20_1
  }
}
#[doc = "Write proxy for field `WUPMC20`"]
pub struct WUPMC20_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC20_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC20_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc20_0(self) -> &'a mut W {
    self.variant(WUPMC20_A::WUPMC20_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc20_1(self) -> &'a mut W {
    self.variant(WUPMC20_A::WUPMC20_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
    self.w
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC21_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC21_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC21_1,
}
impl From<WUPMC21_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC21_A) -> Self {
    match variant {
      WUPMC21_A::WUPMC21_0 => false,
      WUPMC21_A::WUPMC21_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC21`"]
pub type WUPMC21_R = crate::R<bool, WUPMC21_A>;
impl WUPMC21_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC21_A {
    match self.bits {
      false => WUPMC21_A::WUPMC21_0,
      true => WUPMC21_A::WUPMC21_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC21_0`"]
  #[inline(always)]
  pub fn is_wupmc21_0(&self) -> bool {
    *self == WUPMC21_A::WUPMC21_0
  }
  #[doc = "Checks if the value of the field is `WUPMC21_1`"]
  #[inline(always)]
  pub fn is_wupmc21_1(&self) -> bool {
    *self == WUPMC21_A::WUPMC21_1
  }
}
#[doc = "Write proxy for field `WUPMC21`"]
pub struct WUPMC21_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC21_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC21_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc21_0(self) -> &'a mut W {
    self.variant(WUPMC21_A::WUPMC21_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc21_1(self) -> &'a mut W {
    self.variant(WUPMC21_A::WUPMC21_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
    self.w
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC22_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC22_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC22_1,
}
impl From<WUPMC22_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC22_A) -> Self {
    match variant {
      WUPMC22_A::WUPMC22_0 => false,
      WUPMC22_A::WUPMC22_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC22`"]
pub type WUPMC22_R = crate::R<bool, WUPMC22_A>;
impl WUPMC22_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC22_A {
    match self.bits {
      false => WUPMC22_A::WUPMC22_0,
      true => WUPMC22_A::WUPMC22_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC22_0`"]
  #[inline(always)]
  pub fn is_wupmc22_0(&self) -> bool {
    *self == WUPMC22_A::WUPMC22_0
  }
  #[doc = "Checks if the value of the field is `WUPMC22_1`"]
  #[inline(always)]
  pub fn is_wupmc22_1(&self) -> bool {
    *self == WUPMC22_A::WUPMC22_1
  }
}
#[doc = "Write proxy for field `WUPMC22`"]
pub struct WUPMC22_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC22_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC22_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc22_0(self) -> &'a mut W {
    self.variant(WUPMC22_A::WUPMC22_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc22_1(self) -> &'a mut W {
    self.variant(WUPMC22_A::WUPMC22_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
    self.w
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC23_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC23_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC23_1,
}
impl From<WUPMC23_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC23_A) -> Self {
    match variant {
      WUPMC23_A::WUPMC23_0 => false,
      WUPMC23_A::WUPMC23_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC23`"]
pub type WUPMC23_R = crate::R<bool, WUPMC23_A>;
impl WUPMC23_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC23_A {
    match self.bits {
      false => WUPMC23_A::WUPMC23_0,
      true => WUPMC23_A::WUPMC23_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC23_0`"]
  #[inline(always)]
  pub fn is_wupmc23_0(&self) -> bool {
    *self == WUPMC23_A::WUPMC23_0
  }
  #[doc = "Checks if the value of the field is `WUPMC23_1`"]
  #[inline(always)]
  pub fn is_wupmc23_1(&self) -> bool {
    *self == WUPMC23_A::WUPMC23_1
  }
}
#[doc = "Write proxy for field `WUPMC23`"]
pub struct WUPMC23_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC23_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC23_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc23_0(self) -> &'a mut W {
    self.variant(WUPMC23_A::WUPMC23_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc23_1(self) -> &'a mut W {
    self.variant(WUPMC23_A::WUPMC23_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC24_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC24_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC24_1,
}
impl From<WUPMC24_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC24_A) -> Self {
    match variant {
      WUPMC24_A::WUPMC24_0 => false,
      WUPMC24_A::WUPMC24_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC24`"]
pub type WUPMC24_R = crate::R<bool, WUPMC24_A>;
impl WUPMC24_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC24_A {
    match self.bits {
      false => WUPMC24_A::WUPMC24_0,
      true => WUPMC24_A::WUPMC24_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC24_0`"]
  #[inline(always)]
  pub fn is_wupmc24_0(&self) -> bool {
    *self == WUPMC24_A::WUPMC24_0
  }
  #[doc = "Checks if the value of the field is `WUPMC24_1`"]
  #[inline(always)]
  pub fn is_wupmc24_1(&self) -> bool {
    *self == WUPMC24_A::WUPMC24_1
  }
}
#[doc = "Write proxy for field `WUPMC24`"]
pub struct WUPMC24_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC24_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC24_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc24_0(self) -> &'a mut W {
    self.variant(WUPMC24_A::WUPMC24_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc24_1(self) -> &'a mut W {
    self.variant(WUPMC24_A::WUPMC24_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC25_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC25_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC25_1,
}
impl From<WUPMC25_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC25_A) -> Self {
    match variant {
      WUPMC25_A::WUPMC25_0 => false,
      WUPMC25_A::WUPMC25_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC25`"]
pub type WUPMC25_R = crate::R<bool, WUPMC25_A>;
impl WUPMC25_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC25_A {
    match self.bits {
      false => WUPMC25_A::WUPMC25_0,
      true => WUPMC25_A::WUPMC25_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC25_0`"]
  #[inline(always)]
  pub fn is_wupmc25_0(&self) -> bool {
    *self == WUPMC25_A::WUPMC25_0
  }
  #[doc = "Checks if the value of the field is `WUPMC25_1`"]
  #[inline(always)]
  pub fn is_wupmc25_1(&self) -> bool {
    *self == WUPMC25_A::WUPMC25_1
  }
}
#[doc = "Write proxy for field `WUPMC25`"]
pub struct WUPMC25_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC25_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC25_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc25_0(self) -> &'a mut W {
    self.variant(WUPMC25_A::WUPMC25_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc25_1(self) -> &'a mut W {
    self.variant(WUPMC25_A::WUPMC25_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC26_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC26_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC26_1,
}
impl From<WUPMC26_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC26_A) -> Self {
    match variant {
      WUPMC26_A::WUPMC26_0 => false,
      WUPMC26_A::WUPMC26_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC26`"]
pub type WUPMC26_R = crate::R<bool, WUPMC26_A>;
impl WUPMC26_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC26_A {
    match self.bits {
      false => WUPMC26_A::WUPMC26_0,
      true => WUPMC26_A::WUPMC26_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC26_0`"]
  #[inline(always)]
  pub fn is_wupmc26_0(&self) -> bool {
    *self == WUPMC26_A::WUPMC26_0
  }
  #[doc = "Checks if the value of the field is `WUPMC26_1`"]
  #[inline(always)]
  pub fn is_wupmc26_1(&self) -> bool {
    *self == WUPMC26_A::WUPMC26_1
  }
}
#[doc = "Write proxy for field `WUPMC26`"]
pub struct WUPMC26_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC26_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC26_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc26_0(self) -> &'a mut W {
    self.variant(WUPMC26_A::WUPMC26_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc26_1(self) -> &'a mut W {
    self.variant(WUPMC26_A::WUPMC26_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED27_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  RESERVED27_0,
  #[doc = "1: External input pin detection active during all power modes"]
  RESERVED27_1,
}
impl From<RESERVED27_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED27_A) -> Self {
    match variant {
      RESERVED27_A::RESERVED27_0 => false,
      RESERVED27_A::RESERVED27_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved27`"]
pub type RESERVED27_R = crate::R<bool, RESERVED27_A>;
impl RESERVED27_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED27_A {
    match self.bits {
      false => RESERVED27_A::RESERVED27_0,
      true => RESERVED27_A::RESERVED27_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED27_0`"]
  #[inline(always)]
  pub fn is_reserved27_0(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_0
  }
  #[doc = "Checks if the value of the field is `RESERVED27_1`"]
  #[inline(always)]
  pub fn is_reserved27_1(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_1
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED28_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  RESERVED28_0,
  #[doc = "1: External input pin detection active during all power modes"]
  RESERVED28_1,
}
impl From<RESERVED28_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED28_A) -> Self {
    match variant {
      RESERVED28_A::RESERVED28_0 => false,
      RESERVED28_A::RESERVED28_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved28`"]
pub type RESERVED28_R = crate::R<bool, RESERVED28_A>;
impl RESERVED28_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED28_A {
    match self.bits {
      false => RESERVED28_A::RESERVED28_0,
      true => RESERVED28_A::RESERVED28_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED28_0`"]
  #[inline(always)]
  pub fn is_reserved28_0(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_0
  }
  #[doc = "Checks if the value of the field is `RESERVED28_1`"]
  #[inline(always)]
  pub fn is_reserved28_1(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_1
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC29_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC29_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC29_1,
}
impl From<WUPMC29_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC29_A) -> Self {
    match variant {
      WUPMC29_A::WUPMC29_0 => false,
      WUPMC29_A::WUPMC29_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC29`"]
pub type WUPMC29_R = crate::R<bool, WUPMC29_A>;
impl WUPMC29_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC29_A {
    match self.bits {
      false => WUPMC29_A::WUPMC29_0,
      true => WUPMC29_A::WUPMC29_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC29_0`"]
  #[inline(always)]
  pub fn is_wupmc29_0(&self) -> bool {
    *self == WUPMC29_A::WUPMC29_0
  }
  #[doc = "Checks if the value of the field is `WUPMC29_1`"]
  #[inline(always)]
  pub fn is_wupmc29_1(&self) -> bool {
    *self == WUPMC29_A::WUPMC29_1
  }
}
#[doc = "Write proxy for field `WUPMC29`"]
pub struct WUPMC29_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC29_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC29_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc29_0(self) -> &'a mut W {
    self.variant(WUPMC29_A::WUPMC29_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc29_1(self) -> &'a mut W {
    self.variant(WUPMC29_A::WUPMC29_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC30_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC30_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC30_1,
}
impl From<WUPMC30_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC30_A) -> Self {
    match variant {
      WUPMC30_A::WUPMC30_0 => false,
      WUPMC30_A::WUPMC30_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC30`"]
pub type WUPMC30_R = crate::R<bool, WUPMC30_A>;
impl WUPMC30_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC30_A {
    match self.bits {
      false => WUPMC30_A::WUPMC30_0,
      true => WUPMC30_A::WUPMC30_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC30_0`"]
  #[inline(always)]
  pub fn is_wupmc30_0(&self) -> bool {
    *self == WUPMC30_A::WUPMC30_0
  }
  #[doc = "Checks if the value of the field is `WUPMC30_1`"]
  #[inline(always)]
  pub fn is_wupmc30_1(&self) -> bool {
    *self == WUPMC30_A::WUPMC30_1
  }
}
#[doc = "Write proxy for field `WUPMC30`"]
pub struct WUPMC30_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC30_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC30_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc30_0(self) -> &'a mut W {
    self.variant(WUPMC30_A::WUPMC30_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc30_1(self) -> &'a mut W {
    self.variant(WUPMC30_A::WUPMC30_1)
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
#[doc = "Wakeup pin mode for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPMC31_A {
  #[doc = "0: External input pin detection active only during LLS/VLLS mode"]
  WUPMC31_0,
  #[doc = "1: External input pin detection active during all power modes"]
  WUPMC31_1,
}
impl From<WUPMC31_A> for bool {
  #[inline(always)]
  fn from(variant: WUPMC31_A) -> Self {
    match variant {
      WUPMC31_A::WUPMC31_0 => false,
      WUPMC31_A::WUPMC31_1 => true,
    }
  }
}
#[doc = "Reader of field `WUPMC31`"]
pub type WUPMC31_R = crate::R<bool, WUPMC31_A>;
impl WUPMC31_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPMC31_A {
    match self.bits {
      false => WUPMC31_A::WUPMC31_0,
      true => WUPMC31_A::WUPMC31_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUPMC31_0`"]
  #[inline(always)]
  pub fn is_wupmc31_0(&self) -> bool {
    *self == WUPMC31_A::WUPMC31_0
  }
  #[doc = "Checks if the value of the field is `WUPMC31_1`"]
  #[inline(always)]
  pub fn is_wupmc31_1(&self) -> bool {
    *self == WUPMC31_A::WUPMC31_1
  }
}
#[doc = "Write proxy for field `WUPMC31`"]
pub struct WUPMC31_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPMC31_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPMC31_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn wupmc31_0(self) -> &'a mut W {
    self.variant(WUPMC31_A::WUPMC31_0)
  }
  #[doc = "External input pin detection active during all power modes"]
  #[inline(always)]
  pub fn wupmc31_1(self) -> &'a mut W {
    self.variant(WUPMC31_A::WUPMC31_1)
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
  #[doc = "Bit 0 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc0(&self) -> WUPMC0_R {
    WUPMC0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc1(&self) -> WUPMC1_R {
    WUPMC1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc2(&self) -> WUPMC2_R {
    WUPMC2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc3(&self) -> WUPMC3_R {
    WUPMC3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc4(&self) -> WUPMC4_R {
    WUPMC4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc5(&self) -> WUPMC5_R {
    WUPMC5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc6(&self) -> WUPMC6_R {
    WUPMC6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc7(&self) -> WUPMC7_R {
    WUPMC7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc8(&self) -> WUPMC8_R {
    WUPMC8_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc9(&self) -> WUPMC9_R {
    WUPMC9_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc10(&self) -> WUPMC10_R {
    WUPMC10_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc11(&self) -> WUPMC11_R {
    WUPMC11_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc12(&self) -> WUPMC12_R {
    WUPMC12_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc13(&self) -> WUPMC13_R {
    WUPMC13_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc14(&self) -> WUPMC14_R {
    WUPMC14_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc15(&self) -> WUPMC15_R {
    WUPMC15_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc16(&self) -> WUPMC16_R {
    WUPMC16_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc17(&self) -> WUPMC17_R {
    WUPMC17_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc18(&self) -> WUPMC18_R {
    WUPMC18_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc19(&self) -> WUPMC19_R {
    WUPMC19_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 20 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc20(&self) -> WUPMC20_R {
    WUPMC20_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc21(&self) -> WUPMC21_R {
    WUPMC21_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 22 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc22(&self) -> WUPMC22_R {
    WUPMC22_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc23(&self) -> WUPMC23_R {
    WUPMC23_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc24(&self) -> WUPMC24_R {
    WUPMC24_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc25(&self) -> WUPMC25_R {
    WUPMC25_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc26(&self) -> WUPMC26_R {
    WUPMC26_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved27(&self) -> RESERVED27_R {
    RESERVED27_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved28(&self) -> RESERVED28_R {
    RESERVED28_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc29(&self) -> WUPMC29_R {
    WUPMC29_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc30(&self) -> WUPMC30_R {
    WUPMC30_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc31(&self) -> WUPMC31_R {
    WUPMC31_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc0(&mut self) -> WUPMC0_W {
    WUPMC0_W { w: self }
  }
  #[doc = "Bit 1 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc1(&mut self) -> WUPMC1_W {
    WUPMC1_W { w: self }
  }
  #[doc = "Bit 2 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc2(&mut self) -> WUPMC2_W {
    WUPMC2_W { w: self }
  }
  #[doc = "Bit 3 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc3(&mut self) -> WUPMC3_W {
    WUPMC3_W { w: self }
  }
  #[doc = "Bit 4 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc4(&mut self) -> WUPMC4_W {
    WUPMC4_W { w: self }
  }
  #[doc = "Bit 5 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc5(&mut self) -> WUPMC5_W {
    WUPMC5_W { w: self }
  }
  #[doc = "Bit 6 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc6(&mut self) -> WUPMC6_W {
    WUPMC6_W { w: self }
  }
  #[doc = "Bit 7 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc7(&mut self) -> WUPMC7_W {
    WUPMC7_W { w: self }
  }
  #[doc = "Bit 8 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc8(&mut self) -> WUPMC8_W {
    WUPMC8_W { w: self }
  }
  #[doc = "Bit 9 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc9(&mut self) -> WUPMC9_W {
    WUPMC9_W { w: self }
  }
  #[doc = "Bit 10 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc10(&mut self) -> WUPMC10_W {
    WUPMC10_W { w: self }
  }
  #[doc = "Bit 11 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc11(&mut self) -> WUPMC11_W {
    WUPMC11_W { w: self }
  }
  #[doc = "Bit 12 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc12(&mut self) -> WUPMC12_W {
    WUPMC12_W { w: self }
  }
  #[doc = "Bit 13 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc13(&mut self) -> WUPMC13_W {
    WUPMC13_W { w: self }
  }
  #[doc = "Bit 14 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc14(&mut self) -> WUPMC14_W {
    WUPMC14_W { w: self }
  }
  #[doc = "Bit 15 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc15(&mut self) -> WUPMC15_W {
    WUPMC15_W { w: self }
  }
  #[doc = "Bit 16 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc16(&mut self) -> WUPMC16_W {
    WUPMC16_W { w: self }
  }
  #[doc = "Bit 17 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc17(&mut self) -> WUPMC17_W {
    WUPMC17_W { w: self }
  }
  #[doc = "Bit 18 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc18(&mut self) -> WUPMC18_W {
    WUPMC18_W { w: self }
  }
  #[doc = "Bit 19 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc19(&mut self) -> WUPMC19_W {
    WUPMC19_W { w: self }
  }
  #[doc = "Bit 20 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc20(&mut self) -> WUPMC20_W {
    WUPMC20_W { w: self }
  }
  #[doc = "Bit 21 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc21(&mut self) -> WUPMC21_W {
    WUPMC21_W { w: self }
  }
  #[doc = "Bit 22 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc22(&mut self) -> WUPMC22_W {
    WUPMC22_W { w: self }
  }
  #[doc = "Bit 23 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc23(&mut self) -> WUPMC23_W {
    WUPMC23_W { w: self }
  }
  #[doc = "Bit 24 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc24(&mut self) -> WUPMC24_W {
    WUPMC24_W { w: self }
  }
  #[doc = "Bit 25 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc25(&mut self) -> WUPMC25_W {
    WUPMC25_W { w: self }
  }
  #[doc = "Bit 26 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc26(&mut self) -> WUPMC26_W {
    WUPMC26_W { w: self }
  }
  #[doc = "Bit 29 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc29(&mut self) -> WUPMC29_W {
    WUPMC29_W { w: self }
  }
  #[doc = "Bit 30 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc30(&mut self) -> WUPMC30_W {
    WUPMC30_W { w: self }
  }
  #[doc = "Bit 31 - Wakeup pin mode for LLWU_Pn"]
  #[inline(always)]
  pub fn wupmc31(&mut self) -> WUPMC31_W {
    WUPMC31_W { w: self }
  }
}
