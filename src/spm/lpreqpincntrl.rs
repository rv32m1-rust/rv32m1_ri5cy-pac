#[doc = "Reader of register LPREQPINCNTRL"]
pub type R = crate::R<u32, super::LPREQPINCNTRL>;
#[doc = "Writer for register LPREQPINCNTRL"]
pub type W = crate::W<u32, super::LPREQPINCNTRL>;
#[doc = "Register LPREQPINCNTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::LPREQPINCNTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Low Power Request Output Enable Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPREQOE_A {
  #[doc = "0: Low Power request output pin not enabled."]
  LPREQOE_0,
  #[doc = "1: Low Power request output pin enabled."]
  LPREQOE_1,
}
impl From<LPREQOE_A> for bool {
  #[inline(always)]
  fn from(variant: LPREQOE_A) -> Self {
    match variant {
      LPREQOE_A::LPREQOE_0 => false,
      LPREQOE_A::LPREQOE_1 => true,
    }
  }
}
#[doc = "Reader of field `LPREQOE`"]
pub type LPREQOE_R = crate::R<bool, LPREQOE_A>;
impl LPREQOE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPREQOE_A {
    match self.bits {
      false => LPREQOE_A::LPREQOE_0,
      true => LPREQOE_A::LPREQOE_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPREQOE_0`"]
  #[inline(always)]
  pub fn is_lpreqoe_0(&self) -> bool {
    *self == LPREQOE_A::LPREQOE_0
  }
  #[doc = "Checks if the value of the field is `LPREQOE_1`"]
  #[inline(always)]
  pub fn is_lpreqoe_1(&self) -> bool {
    *self == LPREQOE_A::LPREQOE_1
  }
}
#[doc = "Write proxy for field `LPREQOE`"]
pub struct LPREQOE_W<'a> {
  w: &'a mut W,
}
impl<'a> LPREQOE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPREQOE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Low Power request output pin not enabled."]
  #[inline(always)]
  pub fn lpreqoe_0(self) -> &'a mut W {
    self.variant(LPREQOE_A::LPREQOE_0)
  }
  #[doc = "Low Power request output pin enabled."]
  #[inline(always)]
  pub fn lpreqoe_1(self) -> &'a mut W {
    self.variant(LPREQOE_A::LPREQOE_1)
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
#[doc = "Low Power Request Output Pin Polarity Control Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITY_A {
  #[doc = "0: High true polarity."]
  POLARITY_0,
  #[doc = "1: Low true polarity."]
  POLARITY_1,
}
impl From<POLARITY_A> for bool {
  #[inline(always)]
  fn from(variant: POLARITY_A) -> Self {
    match variant {
      POLARITY_A::POLARITY_0 => false,
      POLARITY_A::POLARITY_1 => true,
    }
  }
}
#[doc = "Reader of field `POLARITY`"]
pub type POLARITY_R = crate::R<bool, POLARITY_A>;
impl POLARITY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POLARITY_A {
    match self.bits {
      false => POLARITY_A::POLARITY_0,
      true => POLARITY_A::POLARITY_1,
    }
  }
  #[doc = "Checks if the value of the field is `POLARITY_0`"]
  #[inline(always)]
  pub fn is_polarity_0(&self) -> bool {
    *self == POLARITY_A::POLARITY_0
  }
  #[doc = "Checks if the value of the field is `POLARITY_1`"]
  #[inline(always)]
  pub fn is_polarity_1(&self) -> bool {
    *self == POLARITY_A::POLARITY_1
  }
}
#[doc = "Write proxy for field `POLARITY`"]
pub struct POLARITY_W<'a> {
  w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "High true polarity."]
  #[inline(always)]
  pub fn polarity_0(self) -> &'a mut W {
    self.variant(POLARITY_A::POLARITY_0)
  }
  #[doc = "Low true polarity."]
  #[inline(always)]
  pub fn polarity_1(self) -> &'a mut W {
    self.variant(POLARITY_A::POLARITY_1)
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
  #[doc = "Bit 0 - Low Power Request Output Enable Register"]
  #[inline(always)]
  pub fn lpreqoe(&self) -> LPREQOE_R {
    LPREQOE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Low Power Request Output Pin Polarity Control Register"]
  #[inline(always)]
  pub fn polarity(&self) -> POLARITY_R {
    POLARITY_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Low Power Request Output Enable Register"]
  #[inline(always)]
  pub fn lpreqoe(&mut self) -> LPREQOE_W {
    LPREQOE_W { w: self }
  }
  #[doc = "Bit 1 - Low Power Request Output Pin Polarity Control Register"]
  #[inline(always)]
  pub fn polarity(&mut self) -> POLARITY_W {
    POLARITY_W { w: self }
  }
}
