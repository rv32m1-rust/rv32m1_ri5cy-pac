#[doc = "Reader of register TRIG"]
pub type R = crate::R<u32, super::TRIG>;
#[doc = "Writer for register TRIG"]
pub type W = crate::W<u32, super::TRIG>;
#[doc = "Register TRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Channel 0 Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG0_A {
  #[doc = "0: No effect."]
  TRIG0_0,
  #[doc = "1: Configures trigger input 0 to be used by channel 0."]
  TRIG0_1,
}
impl From<TRIG0_A> for bool {
  #[inline(always)]
  fn from(variant: TRIG0_A) -> Self {
    match variant {
      TRIG0_A::TRIG0_0 => false,
      TRIG0_A::TRIG0_1 => true,
    }
  }
}
#[doc = "Reader of field `TRIG0`"]
pub type TRIG0_R = crate::R<bool, TRIG0_A>;
impl TRIG0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIG0_A {
    match self.bits {
      false => TRIG0_A::TRIG0_0,
      true => TRIG0_A::TRIG0_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRIG0_0`"]
  #[inline(always)]
  pub fn is_trig0_0(&self) -> bool {
    *self == TRIG0_A::TRIG0_0
  }
  #[doc = "Checks if the value of the field is `TRIG0_1`"]
  #[inline(always)]
  pub fn is_trig0_1(&self) -> bool {
    *self == TRIG0_A::TRIG0_1
  }
}
#[doc = "Write proxy for field `TRIG0`"]
pub struct TRIG0_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIG0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIG0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn trig0_0(self) -> &'a mut W {
    self.variant(TRIG0_A::TRIG0_0)
  }
  #[doc = "Configures trigger input 0 to be used by channel 0."]
  #[inline(always)]
  pub fn trig0_1(self) -> &'a mut W {
    self.variant(TRIG0_A::TRIG0_1)
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
#[doc = "Channel 1 Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG1_A {
  #[doc = "0: No effect."]
  TRIG1_0,
  #[doc = "1: Configures trigger input 1 to be used by channel 1."]
  TRIG1_1,
}
impl From<TRIG1_A> for bool {
  #[inline(always)]
  fn from(variant: TRIG1_A) -> Self {
    match variant {
      TRIG1_A::TRIG1_0 => false,
      TRIG1_A::TRIG1_1 => true,
    }
  }
}
#[doc = "Reader of field `TRIG1`"]
pub type TRIG1_R = crate::R<bool, TRIG1_A>;
impl TRIG1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIG1_A {
    match self.bits {
      false => TRIG1_A::TRIG1_0,
      true => TRIG1_A::TRIG1_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRIG1_0`"]
  #[inline(always)]
  pub fn is_trig1_0(&self) -> bool {
    *self == TRIG1_A::TRIG1_0
  }
  #[doc = "Checks if the value of the field is `TRIG1_1`"]
  #[inline(always)]
  pub fn is_trig1_1(&self) -> bool {
    *self == TRIG1_A::TRIG1_1
  }
}
#[doc = "Write proxy for field `TRIG1`"]
pub struct TRIG1_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIG1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIG1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn trig1_0(self) -> &'a mut W {
    self.variant(TRIG1_A::TRIG1_0)
  }
  #[doc = "Configures trigger input 1 to be used by channel 1."]
  #[inline(always)]
  pub fn trig1_1(self) -> &'a mut W {
    self.variant(TRIG1_A::TRIG1_1)
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
#[doc = "Channel 2 Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG2_A {
  #[doc = "0: No effect."]
  TRIG2_0,
  #[doc = "1: Configures trigger input 0 to be used by channel 2."]
  TRIG2_1,
}
impl From<TRIG2_A> for bool {
  #[inline(always)]
  fn from(variant: TRIG2_A) -> Self {
    match variant {
      TRIG2_A::TRIG2_0 => false,
      TRIG2_A::TRIG2_1 => true,
    }
  }
}
#[doc = "Reader of field `TRIG2`"]
pub type TRIG2_R = crate::R<bool, TRIG2_A>;
impl TRIG2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIG2_A {
    match self.bits {
      false => TRIG2_A::TRIG2_0,
      true => TRIG2_A::TRIG2_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRIG2_0`"]
  #[inline(always)]
  pub fn is_trig2_0(&self) -> bool {
    *self == TRIG2_A::TRIG2_0
  }
  #[doc = "Checks if the value of the field is `TRIG2_1`"]
  #[inline(always)]
  pub fn is_trig2_1(&self) -> bool {
    *self == TRIG2_A::TRIG2_1
  }
}
#[doc = "Write proxy for field `TRIG2`"]
pub struct TRIG2_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIG2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIG2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn trig2_0(self) -> &'a mut W {
    self.variant(TRIG2_A::TRIG2_0)
  }
  #[doc = "Configures trigger input 0 to be used by channel 2."]
  #[inline(always)]
  pub fn trig2_1(self) -> &'a mut W {
    self.variant(TRIG2_A::TRIG2_1)
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
#[doc = "Channel 3 Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG3_A {
  #[doc = "0: No effect."]
  TRIG3_0,
  #[doc = "1: Configures trigger input 1 to be used by channel 3."]
  TRIG3_1,
}
impl From<TRIG3_A> for bool {
  #[inline(always)]
  fn from(variant: TRIG3_A) -> Self {
    match variant {
      TRIG3_A::TRIG3_0 => false,
      TRIG3_A::TRIG3_1 => true,
    }
  }
}
#[doc = "Reader of field `TRIG3`"]
pub type TRIG3_R = crate::R<bool, TRIG3_A>;
impl TRIG3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIG3_A {
    match self.bits {
      false => TRIG3_A::TRIG3_0,
      true => TRIG3_A::TRIG3_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRIG3_0`"]
  #[inline(always)]
  pub fn is_trig3_0(&self) -> bool {
    *self == TRIG3_A::TRIG3_0
  }
  #[doc = "Checks if the value of the field is `TRIG3_1`"]
  #[inline(always)]
  pub fn is_trig3_1(&self) -> bool {
    *self == TRIG3_A::TRIG3_1
  }
}
#[doc = "Write proxy for field `TRIG3`"]
pub struct TRIG3_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIG3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIG3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn trig3_0(self) -> &'a mut W {
    self.variant(TRIG3_A::TRIG3_0)
  }
  #[doc = "Configures trigger input 1 to be used by channel 3."]
  #[inline(always)]
  pub fn trig3_1(self) -> &'a mut W {
    self.variant(TRIG3_A::TRIG3_1)
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
#[doc = "Channel 4 Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG4_A {
  #[doc = "0: No effect."]
  TRIG4_0,
  #[doc = "1: Configures trigger input 0 to be used by channel 4."]
  TRIG4_1,
}
impl From<TRIG4_A> for bool {
  #[inline(always)]
  fn from(variant: TRIG4_A) -> Self {
    match variant {
      TRIG4_A::TRIG4_0 => false,
      TRIG4_A::TRIG4_1 => true,
    }
  }
}
#[doc = "Reader of field `TRIG4`"]
pub type TRIG4_R = crate::R<bool, TRIG4_A>;
impl TRIG4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIG4_A {
    match self.bits {
      false => TRIG4_A::TRIG4_0,
      true => TRIG4_A::TRIG4_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRIG4_0`"]
  #[inline(always)]
  pub fn is_trig4_0(&self) -> bool {
    *self == TRIG4_A::TRIG4_0
  }
  #[doc = "Checks if the value of the field is `TRIG4_1`"]
  #[inline(always)]
  pub fn is_trig4_1(&self) -> bool {
    *self == TRIG4_A::TRIG4_1
  }
}
#[doc = "Write proxy for field `TRIG4`"]
pub struct TRIG4_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIG4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIG4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn trig4_0(self) -> &'a mut W {
    self.variant(TRIG4_A::TRIG4_0)
  }
  #[doc = "Configures trigger input 0 to be used by channel 4."]
  #[inline(always)]
  pub fn trig4_1(self) -> &'a mut W {
    self.variant(TRIG4_A::TRIG4_1)
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
#[doc = "Channel 5 Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG5_A {
  #[doc = "0: No effect."]
  TRIG5_0,
  #[doc = "1: Configures trigger input 1 to be used by channel 5."]
  TRIG5_1,
}
impl From<TRIG5_A> for bool {
  #[inline(always)]
  fn from(variant: TRIG5_A) -> Self {
    match variant {
      TRIG5_A::TRIG5_0 => false,
      TRIG5_A::TRIG5_1 => true,
    }
  }
}
#[doc = "Reader of field `TRIG5`"]
pub type TRIG5_R = crate::R<bool, TRIG5_A>;
impl TRIG5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIG5_A {
    match self.bits {
      false => TRIG5_A::TRIG5_0,
      true => TRIG5_A::TRIG5_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRIG5_0`"]
  #[inline(always)]
  pub fn is_trig5_0(&self) -> bool {
    *self == TRIG5_A::TRIG5_0
  }
  #[doc = "Checks if the value of the field is `TRIG5_1`"]
  #[inline(always)]
  pub fn is_trig5_1(&self) -> bool {
    *self == TRIG5_A::TRIG5_1
  }
}
#[doc = "Write proxy for field `TRIG5`"]
pub struct TRIG5_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIG5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIG5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn trig5_0(self) -> &'a mut W {
    self.variant(TRIG5_A::TRIG5_0)
  }
  #[doc = "Configures trigger input 1 to be used by channel 5."]
  #[inline(always)]
  pub fn trig5_1(self) -> &'a mut W {
    self.variant(TRIG5_A::TRIG5_1)
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
impl R {
  #[doc = "Bit 0 - Channel 0 Trigger"]
  #[inline(always)]
  pub fn trig0(&self) -> TRIG0_R {
    TRIG0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Channel 1 Trigger"]
  #[inline(always)]
  pub fn trig1(&self) -> TRIG1_R {
    TRIG1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Channel 2 Trigger"]
  #[inline(always)]
  pub fn trig2(&self) -> TRIG2_R {
    TRIG2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Channel 3 Trigger"]
  #[inline(always)]
  pub fn trig3(&self) -> TRIG3_R {
    TRIG3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Channel 4 Trigger"]
  #[inline(always)]
  pub fn trig4(&self) -> TRIG4_R {
    TRIG4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Channel 5 Trigger"]
  #[inline(always)]
  pub fn trig5(&self) -> TRIG5_R {
    TRIG5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Channel 0 Trigger"]
  #[inline(always)]
  pub fn trig0(&mut self) -> TRIG0_W {
    TRIG0_W { w: self }
  }
  #[doc = "Bit 1 - Channel 1 Trigger"]
  #[inline(always)]
  pub fn trig1(&mut self) -> TRIG1_W {
    TRIG1_W { w: self }
  }
  #[doc = "Bit 2 - Channel 2 Trigger"]
  #[inline(always)]
  pub fn trig2(&mut self) -> TRIG2_W {
    TRIG2_W { w: self }
  }
  #[doc = "Bit 3 - Channel 3 Trigger"]
  #[inline(always)]
  pub fn trig3(&mut self) -> TRIG3_W {
    TRIG3_W { w: self }
  }
  #[doc = "Bit 4 - Channel 4 Trigger"]
  #[inline(always)]
  pub fn trig4(&mut self) -> TRIG4_W {
    TRIG4_W { w: self }
  }
  #[doc = "Bit 5 - Channel 5 Trigger"]
  #[inline(always)]
  pub fn trig5(&mut self) -> TRIG5_W {
    TRIG5_W { w: self }
  }
}
