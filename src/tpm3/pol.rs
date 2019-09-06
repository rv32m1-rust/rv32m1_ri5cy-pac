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
}
