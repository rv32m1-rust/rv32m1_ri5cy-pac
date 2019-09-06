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
}
