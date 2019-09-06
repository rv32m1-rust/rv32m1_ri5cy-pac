#[doc = "Reader of register COMBINE"]
pub type R = crate::R<u32, super::COMBINE>;
#[doc = "Writer for register COMBINE"]
pub type W = crate::W<u32, super::COMBINE>;
#[doc = "Register COMBINE `reset()`'s with value 0"]
impl crate::ResetValue for super::COMBINE {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Combine Channels 0 and 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE0_A {
  #[doc = "0: Channels 0 and 1 are independent."]
  COMBINE0_0,
  #[doc = "1: Channels 0 and 1 are combined."]
  COMBINE0_1,
}
impl From<COMBINE0_A> for bool {
  #[inline(always)]
  fn from(variant: COMBINE0_A) -> Self {
    match variant {
      COMBINE0_A::COMBINE0_0 => false,
      COMBINE0_A::COMBINE0_1 => true,
    }
  }
}
#[doc = "Reader of field `COMBINE0`"]
pub type COMBINE0_R = crate::R<bool, COMBINE0_A>;
impl COMBINE0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COMBINE0_A {
    match self.bits {
      false => COMBINE0_A::COMBINE0_0,
      true => COMBINE0_A::COMBINE0_1,
    }
  }
  #[doc = "Checks if the value of the field is `COMBINE0_0`"]
  #[inline(always)]
  pub fn is_combine0_0(&self) -> bool {
    *self == COMBINE0_A::COMBINE0_0
  }
  #[doc = "Checks if the value of the field is `COMBINE0_1`"]
  #[inline(always)]
  pub fn is_combine0_1(&self) -> bool {
    *self == COMBINE0_A::COMBINE0_1
  }
}
#[doc = "Write proxy for field `COMBINE0`"]
pub struct COMBINE0_W<'a> {
  w: &'a mut W,
}
impl<'a> COMBINE0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COMBINE0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Channels 0 and 1 are independent."]
  #[inline(always)]
  pub fn combine0_0(self) -> &'a mut W {
    self.variant(COMBINE0_A::COMBINE0_0)
  }
  #[doc = "Channels 0 and 1 are combined."]
  #[inline(always)]
  pub fn combine0_1(self) -> &'a mut W {
    self.variant(COMBINE0_A::COMBINE0_1)
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
#[doc = "Combine Channel 0 and 1 Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP0_A {
  #[doc = "0: Even channel is used for input capture and 1st compare."]
  COMSWAP0_0,
  #[doc = "1: Odd channel is used for input capture and 1st compare."]
  COMSWAP0_1,
}
impl From<COMSWAP0_A> for bool {
  #[inline(always)]
  fn from(variant: COMSWAP0_A) -> Self {
    match variant {
      COMSWAP0_A::COMSWAP0_0 => false,
      COMSWAP0_A::COMSWAP0_1 => true,
    }
  }
}
#[doc = "Reader of field `COMSWAP0`"]
pub type COMSWAP0_R = crate::R<bool, COMSWAP0_A>;
impl COMSWAP0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COMSWAP0_A {
    match self.bits {
      false => COMSWAP0_A::COMSWAP0_0,
      true => COMSWAP0_A::COMSWAP0_1,
    }
  }
  #[doc = "Checks if the value of the field is `COMSWAP0_0`"]
  #[inline(always)]
  pub fn is_comswap0_0(&self) -> bool {
    *self == COMSWAP0_A::COMSWAP0_0
  }
  #[doc = "Checks if the value of the field is `COMSWAP0_1`"]
  #[inline(always)]
  pub fn is_comswap0_1(&self) -> bool {
    *self == COMSWAP0_A::COMSWAP0_1
  }
}
#[doc = "Write proxy for field `COMSWAP0`"]
pub struct COMSWAP0_W<'a> {
  w: &'a mut W,
}
impl<'a> COMSWAP0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COMSWAP0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Even channel is used for input capture and 1st compare."]
  #[inline(always)]
  pub fn comswap0_0(self) -> &'a mut W {
    self.variant(COMSWAP0_A::COMSWAP0_0)
  }
  #[doc = "Odd channel is used for input capture and 1st compare."]
  #[inline(always)]
  pub fn comswap0_1(self) -> &'a mut W {
    self.variant(COMSWAP0_A::COMSWAP0_1)
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
#[doc = "Combine Channels 2 and 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE1_A {
  #[doc = "0: Channels 2 and 3 are independent."]
  COMBINE1_0,
  #[doc = "1: Channels 2 and 3 are combined."]
  COMBINE1_1,
}
impl From<COMBINE1_A> for bool {
  #[inline(always)]
  fn from(variant: COMBINE1_A) -> Self {
    match variant {
      COMBINE1_A::COMBINE1_0 => false,
      COMBINE1_A::COMBINE1_1 => true,
    }
  }
}
#[doc = "Reader of field `COMBINE1`"]
pub type COMBINE1_R = crate::R<bool, COMBINE1_A>;
impl COMBINE1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COMBINE1_A {
    match self.bits {
      false => COMBINE1_A::COMBINE1_0,
      true => COMBINE1_A::COMBINE1_1,
    }
  }
  #[doc = "Checks if the value of the field is `COMBINE1_0`"]
  #[inline(always)]
  pub fn is_combine1_0(&self) -> bool {
    *self == COMBINE1_A::COMBINE1_0
  }
  #[doc = "Checks if the value of the field is `COMBINE1_1`"]
  #[inline(always)]
  pub fn is_combine1_1(&self) -> bool {
    *self == COMBINE1_A::COMBINE1_1
  }
}
#[doc = "Write proxy for field `COMBINE1`"]
pub struct COMBINE1_W<'a> {
  w: &'a mut W,
}
impl<'a> COMBINE1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COMBINE1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Channels 2 and 3 are independent."]
  #[inline(always)]
  pub fn combine1_0(self) -> &'a mut W {
    self.variant(COMBINE1_A::COMBINE1_0)
  }
  #[doc = "Channels 2 and 3 are combined."]
  #[inline(always)]
  pub fn combine1_1(self) -> &'a mut W {
    self.variant(COMBINE1_A::COMBINE1_1)
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
#[doc = "Combine Channels 2 and 3 Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP1_A {
  #[doc = "0: Even channel is used for input capture and 1st compare."]
  COMSWAP1_0,
  #[doc = "1: Odd channel is used for input capture and 1st compare."]
  COMSWAP1_1,
}
impl From<COMSWAP1_A> for bool {
  #[inline(always)]
  fn from(variant: COMSWAP1_A) -> Self {
    match variant {
      COMSWAP1_A::COMSWAP1_0 => false,
      COMSWAP1_A::COMSWAP1_1 => true,
    }
  }
}
#[doc = "Reader of field `COMSWAP1`"]
pub type COMSWAP1_R = crate::R<bool, COMSWAP1_A>;
impl COMSWAP1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COMSWAP1_A {
    match self.bits {
      false => COMSWAP1_A::COMSWAP1_0,
      true => COMSWAP1_A::COMSWAP1_1,
    }
  }
  #[doc = "Checks if the value of the field is `COMSWAP1_0`"]
  #[inline(always)]
  pub fn is_comswap1_0(&self) -> bool {
    *self == COMSWAP1_A::COMSWAP1_0
  }
  #[doc = "Checks if the value of the field is `COMSWAP1_1`"]
  #[inline(always)]
  pub fn is_comswap1_1(&self) -> bool {
    *self == COMSWAP1_A::COMSWAP1_1
  }
}
#[doc = "Write proxy for field `COMSWAP1`"]
pub struct COMSWAP1_W<'a> {
  w: &'a mut W,
}
impl<'a> COMSWAP1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COMSWAP1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Even channel is used for input capture and 1st compare."]
  #[inline(always)]
  pub fn comswap1_0(self) -> &'a mut W {
    self.variant(COMSWAP1_A::COMSWAP1_0)
  }
  #[doc = "Odd channel is used for input capture and 1st compare."]
  #[inline(always)]
  pub fn comswap1_1(self) -> &'a mut W {
    self.variant(COMSWAP1_A::COMSWAP1_1)
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
#[doc = "Combine Channels 4 and 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE2_A {
  #[doc = "0: Channels 4 and 5 are independent."]
  COMBINE2_0,
  #[doc = "1: Channels 4 and 5 are combined."]
  COMBINE2_1,
}
impl From<COMBINE2_A> for bool {
  #[inline(always)]
  fn from(variant: COMBINE2_A) -> Self {
    match variant {
      COMBINE2_A::COMBINE2_0 => false,
      COMBINE2_A::COMBINE2_1 => true,
    }
  }
}
#[doc = "Reader of field `COMBINE2`"]
pub type COMBINE2_R = crate::R<bool, COMBINE2_A>;
impl COMBINE2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COMBINE2_A {
    match self.bits {
      false => COMBINE2_A::COMBINE2_0,
      true => COMBINE2_A::COMBINE2_1,
    }
  }
  #[doc = "Checks if the value of the field is `COMBINE2_0`"]
  #[inline(always)]
  pub fn is_combine2_0(&self) -> bool {
    *self == COMBINE2_A::COMBINE2_0
  }
  #[doc = "Checks if the value of the field is `COMBINE2_1`"]
  #[inline(always)]
  pub fn is_combine2_1(&self) -> bool {
    *self == COMBINE2_A::COMBINE2_1
  }
}
#[doc = "Write proxy for field `COMBINE2`"]
pub struct COMBINE2_W<'a> {
  w: &'a mut W,
}
impl<'a> COMBINE2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COMBINE2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Channels 4 and 5 are independent."]
  #[inline(always)]
  pub fn combine2_0(self) -> &'a mut W {
    self.variant(COMBINE2_A::COMBINE2_0)
  }
  #[doc = "Channels 4 and 5 are combined."]
  #[inline(always)]
  pub fn combine2_1(self) -> &'a mut W {
    self.variant(COMBINE2_A::COMBINE2_1)
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
#[doc = "Combine Channels 4 and 5 Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP2_A {
  #[doc = "0: Even channel is used for input capture and 1st compare."]
  COMSWAP2_0,
  #[doc = "1: Odd channel is used for input capture and 1st compare."]
  COMSWAP2_1,
}
impl From<COMSWAP2_A> for bool {
  #[inline(always)]
  fn from(variant: COMSWAP2_A) -> Self {
    match variant {
      COMSWAP2_A::COMSWAP2_0 => false,
      COMSWAP2_A::COMSWAP2_1 => true,
    }
  }
}
#[doc = "Reader of field `COMSWAP2`"]
pub type COMSWAP2_R = crate::R<bool, COMSWAP2_A>;
impl COMSWAP2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COMSWAP2_A {
    match self.bits {
      false => COMSWAP2_A::COMSWAP2_0,
      true => COMSWAP2_A::COMSWAP2_1,
    }
  }
  #[doc = "Checks if the value of the field is `COMSWAP2_0`"]
  #[inline(always)]
  pub fn is_comswap2_0(&self) -> bool {
    *self == COMSWAP2_A::COMSWAP2_0
  }
  #[doc = "Checks if the value of the field is `COMSWAP2_1`"]
  #[inline(always)]
  pub fn is_comswap2_1(&self) -> bool {
    *self == COMSWAP2_A::COMSWAP2_1
  }
}
#[doc = "Write proxy for field `COMSWAP2`"]
pub struct COMSWAP2_W<'a> {
  w: &'a mut W,
}
impl<'a> COMSWAP2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COMSWAP2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Even channel is used for input capture and 1st compare."]
  #[inline(always)]
  pub fn comswap2_0(self) -> &'a mut W {
    self.variant(COMSWAP2_A::COMSWAP2_0)
  }
  #[doc = "Odd channel is used for input capture and 1st compare."]
  #[inline(always)]
  pub fn comswap2_1(self) -> &'a mut W {
    self.variant(COMSWAP2_A::COMSWAP2_1)
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
impl R {
  #[doc = "Bit 0 - Combine Channels 0 and 1"]
  #[inline(always)]
  pub fn combine0(&self) -> COMBINE0_R {
    COMBINE0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Combine Channel 0 and 1 Swap"]
  #[inline(always)]
  pub fn comswap0(&self) -> COMSWAP0_R {
    COMSWAP0_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Combine Channels 2 and 3"]
  #[inline(always)]
  pub fn combine1(&self) -> COMBINE1_R {
    COMBINE1_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Combine Channels 2 and 3 Swap"]
  #[inline(always)]
  pub fn comswap1(&self) -> COMSWAP1_R {
    COMSWAP1_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Combine Channels 4 and 5"]
  #[inline(always)]
  pub fn combine2(&self) -> COMBINE2_R {
    COMBINE2_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Combine Channels 4 and 5 Swap"]
  #[inline(always)]
  pub fn comswap2(&self) -> COMSWAP2_R {
    COMSWAP2_R::new(((self.bits >> 17) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Combine Channels 0 and 1"]
  #[inline(always)]
  pub fn combine0(&mut self) -> COMBINE0_W {
    COMBINE0_W { w: self }
  }
  #[doc = "Bit 1 - Combine Channel 0 and 1 Swap"]
  #[inline(always)]
  pub fn comswap0(&mut self) -> COMSWAP0_W {
    COMSWAP0_W { w: self }
  }
  #[doc = "Bit 8 - Combine Channels 2 and 3"]
  #[inline(always)]
  pub fn combine1(&mut self) -> COMBINE1_W {
    COMBINE1_W { w: self }
  }
  #[doc = "Bit 9 - Combine Channels 2 and 3 Swap"]
  #[inline(always)]
  pub fn comswap1(&mut self) -> COMSWAP1_W {
    COMSWAP1_W { w: self }
  }
  #[doc = "Bit 16 - Combine Channels 4 and 5"]
  #[inline(always)]
  pub fn combine2(&mut self) -> COMBINE2_W {
    COMBINE2_W { w: self }
  }
  #[doc = "Bit 17 - Combine Channels 4 and 5 Swap"]
  #[inline(always)]
  pub fn comswap2(&mut self) -> COMSWAP2_W {
    COMSWAP2_W { w: self }
  }
}
