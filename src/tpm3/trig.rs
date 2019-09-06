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
}
