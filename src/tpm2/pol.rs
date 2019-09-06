#[doc = "Reader of register POL"]
pub type R = crate::R<u32, super::POL>;
#[doc = "Writer for register POL"]
pub type W = crate::W<u32, super::POL>;
#[doc = "Register POL `reset()`'s with value 0"]
impl crate::ResetValue for super::POL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Channel 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL0_A {
  #[doc = "0: The channel polarity is active high."]
  POL0_0,
  #[doc = "1: The channel polarity is active low."]
  POL0_1,
}
impl From<POL0_A> for bool {
  #[inline(always)]
  fn from(variant: POL0_A) -> Self {
    match variant {
      POL0_A::POL0_0 => false,
      POL0_A::POL0_1 => true,
    }
  }
}
#[doc = "Reader of field `POL0`"]
pub type POL0_R = crate::R<bool, POL0_A>;
impl POL0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POL0_A {
    match self.bits {
      false => POL0_A::POL0_0,
      true => POL0_A::POL0_1,
    }
  }
  #[doc = "Checks if the value of the field is `POL0_0`"]
  #[inline(always)]
  pub fn is_pol0_0(&self) -> bool {
    *self == POL0_A::POL0_0
  }
  #[doc = "Checks if the value of the field is `POL0_1`"]
  #[inline(always)]
  pub fn is_pol0_1(&self) -> bool {
    *self == POL0_A::POL0_1
  }
}
#[doc = "Write proxy for field `POL0`"]
pub struct POL0_W<'a> {
  w: &'a mut W,
}
impl<'a> POL0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POL0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel polarity is active high."]
  #[inline(always)]
  pub fn pol0_0(self) -> &'a mut W {
    self.variant(POL0_A::POL0_0)
  }
  #[doc = "The channel polarity is active low."]
  #[inline(always)]
  pub fn pol0_1(self) -> &'a mut W {
    self.variant(POL0_A::POL0_1)
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
#[doc = "Channel 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1_A {
  #[doc = "0: The channel polarity is active high."]
  POL1_0,
  #[doc = "1: The channel polarity is active low."]
  POL1_1,
}
impl From<POL1_A> for bool {
  #[inline(always)]
  fn from(variant: POL1_A) -> Self {
    match variant {
      POL1_A::POL1_0 => false,
      POL1_A::POL1_1 => true,
    }
  }
}
#[doc = "Reader of field `POL1`"]
pub type POL1_R = crate::R<bool, POL1_A>;
impl POL1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POL1_A {
    match self.bits {
      false => POL1_A::POL1_0,
      true => POL1_A::POL1_1,
    }
  }
  #[doc = "Checks if the value of the field is `POL1_0`"]
  #[inline(always)]
  pub fn is_pol1_0(&self) -> bool {
    *self == POL1_A::POL1_0
  }
  #[doc = "Checks if the value of the field is `POL1_1`"]
  #[inline(always)]
  pub fn is_pol1_1(&self) -> bool {
    *self == POL1_A::POL1_1
  }
}
#[doc = "Write proxy for field `POL1`"]
pub struct POL1_W<'a> {
  w: &'a mut W,
}
impl<'a> POL1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POL1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel polarity is active high."]
  #[inline(always)]
  pub fn pol1_0(self) -> &'a mut W {
    self.variant(POL1_A::POL1_0)
  }
  #[doc = "The channel polarity is active low."]
  #[inline(always)]
  pub fn pol1_1(self) -> &'a mut W {
    self.variant(POL1_A::POL1_1)
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
#[doc = "Channel 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2_A {
  #[doc = "0: The channel polarity is active high."]
  POL2_0,
  #[doc = "1: The channel polarity is active low."]
  POL2_1,
}
impl From<POL2_A> for bool {
  #[inline(always)]
  fn from(variant: POL2_A) -> Self {
    match variant {
      POL2_A::POL2_0 => false,
      POL2_A::POL2_1 => true,
    }
  }
}
#[doc = "Reader of field `POL2`"]
pub type POL2_R = crate::R<bool, POL2_A>;
impl POL2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POL2_A {
    match self.bits {
      false => POL2_A::POL2_0,
      true => POL2_A::POL2_1,
    }
  }
  #[doc = "Checks if the value of the field is `POL2_0`"]
  #[inline(always)]
  pub fn is_pol2_0(&self) -> bool {
    *self == POL2_A::POL2_0
  }
  #[doc = "Checks if the value of the field is `POL2_1`"]
  #[inline(always)]
  pub fn is_pol2_1(&self) -> bool {
    *self == POL2_A::POL2_1
  }
}
#[doc = "Write proxy for field `POL2`"]
pub struct POL2_W<'a> {
  w: &'a mut W,
}
impl<'a> POL2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POL2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel polarity is active high."]
  #[inline(always)]
  pub fn pol2_0(self) -> &'a mut W {
    self.variant(POL2_A::POL2_0)
  }
  #[doc = "The channel polarity is active low."]
  #[inline(always)]
  pub fn pol2_1(self) -> &'a mut W {
    self.variant(POL2_A::POL2_1)
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
#[doc = "Channel 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL3_A {
  #[doc = "0: The channel polarity is active high."]
  POL3_0,
  #[doc = "1: The channel polarity is active low."]
  POL3_1,
}
impl From<POL3_A> for bool {
  #[inline(always)]
  fn from(variant: POL3_A) -> Self {
    match variant {
      POL3_A::POL3_0 => false,
      POL3_A::POL3_1 => true,
    }
  }
}
#[doc = "Reader of field `POL3`"]
pub type POL3_R = crate::R<bool, POL3_A>;
impl POL3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POL3_A {
    match self.bits {
      false => POL3_A::POL3_0,
      true => POL3_A::POL3_1,
    }
  }
  #[doc = "Checks if the value of the field is `POL3_0`"]
  #[inline(always)]
  pub fn is_pol3_0(&self) -> bool {
    *self == POL3_A::POL3_0
  }
  #[doc = "Checks if the value of the field is `POL3_1`"]
  #[inline(always)]
  pub fn is_pol3_1(&self) -> bool {
    *self == POL3_A::POL3_1
  }
}
#[doc = "Write proxy for field `POL3`"]
pub struct POL3_W<'a> {
  w: &'a mut W,
}
impl<'a> POL3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POL3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel polarity is active high."]
  #[inline(always)]
  pub fn pol3_0(self) -> &'a mut W {
    self.variant(POL3_A::POL3_0)
  }
  #[doc = "The channel polarity is active low."]
  #[inline(always)]
  pub fn pol3_1(self) -> &'a mut W {
    self.variant(POL3_A::POL3_1)
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
#[doc = "Channel 4 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL4_A {
  #[doc = "0: The channel polarity is active high"]
  POL4_0,
  #[doc = "1: The channel polarity is active low."]
  POL4_1,
}
impl From<POL4_A> for bool {
  #[inline(always)]
  fn from(variant: POL4_A) -> Self {
    match variant {
      POL4_A::POL4_0 => false,
      POL4_A::POL4_1 => true,
    }
  }
}
#[doc = "Reader of field `POL4`"]
pub type POL4_R = crate::R<bool, POL4_A>;
impl POL4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POL4_A {
    match self.bits {
      false => POL4_A::POL4_0,
      true => POL4_A::POL4_1,
    }
  }
  #[doc = "Checks if the value of the field is `POL4_0`"]
  #[inline(always)]
  pub fn is_pol4_0(&self) -> bool {
    *self == POL4_A::POL4_0
  }
  #[doc = "Checks if the value of the field is `POL4_1`"]
  #[inline(always)]
  pub fn is_pol4_1(&self) -> bool {
    *self == POL4_A::POL4_1
  }
}
#[doc = "Write proxy for field `POL4`"]
pub struct POL4_W<'a> {
  w: &'a mut W,
}
impl<'a> POL4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POL4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel polarity is active high"]
  #[inline(always)]
  pub fn pol4_0(self) -> &'a mut W {
    self.variant(POL4_A::POL4_0)
  }
  #[doc = "The channel polarity is active low."]
  #[inline(always)]
  pub fn pol4_1(self) -> &'a mut W {
    self.variant(POL4_A::POL4_1)
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
#[doc = "Channel 5 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL5_A {
  #[doc = "0: The channel polarity is active high."]
  POL5_0,
  #[doc = "1: The channel polarity is active low."]
  POL5_1,
}
impl From<POL5_A> for bool {
  #[inline(always)]
  fn from(variant: POL5_A) -> Self {
    match variant {
      POL5_A::POL5_0 => false,
      POL5_A::POL5_1 => true,
    }
  }
}
#[doc = "Reader of field `POL5`"]
pub type POL5_R = crate::R<bool, POL5_A>;
impl POL5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POL5_A {
    match self.bits {
      false => POL5_A::POL5_0,
      true => POL5_A::POL5_1,
    }
  }
  #[doc = "Checks if the value of the field is `POL5_0`"]
  #[inline(always)]
  pub fn is_pol5_0(&self) -> bool {
    *self == POL5_A::POL5_0
  }
  #[doc = "Checks if the value of the field is `POL5_1`"]
  #[inline(always)]
  pub fn is_pol5_1(&self) -> bool {
    *self == POL5_A::POL5_1
  }
}
#[doc = "Write proxy for field `POL5`"]
pub struct POL5_W<'a> {
  w: &'a mut W,
}
impl<'a> POL5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POL5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel polarity is active high."]
  #[inline(always)]
  pub fn pol5_0(self) -> &'a mut W {
    self.variant(POL5_A::POL5_0)
  }
  #[doc = "The channel polarity is active low."]
  #[inline(always)]
  pub fn pol5_1(self) -> &'a mut W {
    self.variant(POL5_A::POL5_1)
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
  #[doc = "Bit 0 - Channel 0 Polarity"]
  #[inline(always)]
  pub fn pol0(&self) -> POL0_R {
    POL0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Channel 1 Polarity"]
  #[inline(always)]
  pub fn pol1(&self) -> POL1_R {
    POL1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Channel 2 Polarity"]
  #[inline(always)]
  pub fn pol2(&self) -> POL2_R {
    POL2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Channel 3 Polarity"]
  #[inline(always)]
  pub fn pol3(&self) -> POL3_R {
    POL3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Channel 4 Polarity"]
  #[inline(always)]
  pub fn pol4(&self) -> POL4_R {
    POL4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Channel 5 Polarity"]
  #[inline(always)]
  pub fn pol5(&self) -> POL5_R {
    POL5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Channel 0 Polarity"]
  #[inline(always)]
  pub fn pol0(&mut self) -> POL0_W {
    POL0_W { w: self }
  }
  #[doc = "Bit 1 - Channel 1 Polarity"]
  #[inline(always)]
  pub fn pol1(&mut self) -> POL1_W {
    POL1_W { w: self }
  }
  #[doc = "Bit 2 - Channel 2 Polarity"]
  #[inline(always)]
  pub fn pol2(&mut self) -> POL2_W {
    POL2_W { w: self }
  }
  #[doc = "Bit 3 - Channel 3 Polarity"]
  #[inline(always)]
  pub fn pol3(&mut self) -> POL3_W {
    POL3_W { w: self }
  }
  #[doc = "Bit 4 - Channel 4 Polarity"]
  #[inline(always)]
  pub fn pol4(&mut self) -> POL4_W {
    POL4_W { w: self }
  }
  #[doc = "Bit 5 - Channel 5 Polarity"]
  #[inline(always)]
  pub fn pol5(&mut self) -> POL5_W {
    POL5_W { w: self }
  }
}
