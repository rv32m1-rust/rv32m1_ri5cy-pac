#[doc = "Reader of register RCR"]
pub type R = crate::R<u32, super::RCR>;
#[doc = "Writer for register RCR"]
pub type W = crate::W<u32, super::RCR>;
#[doc = "Register RCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_A {
  #[doc = "0: No effect"]
  SWRST_0,
  #[doc = "1: Software reset"]
  SWRST_1,
}
impl From<SWRST_A> for bool {
  #[inline(always)]
  fn from(variant: SWRST_A) -> Self {
    match variant {
      SWRST_A::SWRST_0 => false,
      SWRST_A::SWRST_1 => true,
    }
  }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, SWRST_A>;
impl SWRST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWRST_A {
    match self.bits {
      false => SWRST_A::SWRST_0,
      true => SWRST_A::SWRST_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWRST_0`"]
  #[inline(always)]
  pub fn is_swrst_0(&self) -> bool {
    *self == SWRST_A::SWRST_0
  }
  #[doc = "Checks if the value of the field is `SWRST_1`"]
  #[inline(always)]
  pub fn is_swrst_1(&self) -> bool {
    *self == SWRST_A::SWRST_1
  }
}
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
  w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWRST_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect"]
  #[inline(always)]
  pub fn swrst_0(self) -> &'a mut W {
    self.variant(SWRST_A::SWRST_0)
  }
  #[doc = "Software reset"]
  #[inline(always)]
  pub fn swrst_1(self) -> &'a mut W {
    self.variant(SWRST_A::SWRST_1)
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
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFORST_A {
  #[doc = "0: No effect"]
  FIFORST_0,
  #[doc = "1: FIFO reset"]
  FIFORST_1,
}
impl From<FIFORST_A> for bool {
  #[inline(always)]
  fn from(variant: FIFORST_A) -> Self {
    match variant {
      FIFORST_A::FIFORST_0 => false,
      FIFORST_A::FIFORST_1 => true,
    }
  }
}
#[doc = "Reader of field `FIFORST`"]
pub type FIFORST_R = crate::R<bool, FIFORST_A>;
impl FIFORST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIFORST_A {
    match self.bits {
      false => FIFORST_A::FIFORST_0,
      true => FIFORST_A::FIFORST_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIFORST_0`"]
  #[inline(always)]
  pub fn is_fiforst_0(&self) -> bool {
    *self == FIFORST_A::FIFORST_0
  }
  #[doc = "Checks if the value of the field is `FIFORST_1`"]
  #[inline(always)]
  pub fn is_fiforst_1(&self) -> bool {
    *self == FIFORST_A::FIFORST_1
  }
}
#[doc = "Write proxy for field `FIFORST`"]
pub struct FIFORST_W<'a> {
  w: &'a mut W,
}
impl<'a> FIFORST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIFORST_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect"]
  #[inline(always)]
  pub fn fiforst_0(self) -> &'a mut W {
    self.variant(FIFORST_A::FIFORST_0)
  }
  #[doc = "FIFO reset"]
  #[inline(always)]
  pub fn fiforst_1(self) -> &'a mut W {
    self.variant(FIFORST_A::FIFORST_1)
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
  #[doc = "Bit 0 - Software Reset"]
  #[inline(always)]
  pub fn swrst(&self) -> SWRST_R {
    SWRST_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - FIFO Reset"]
  #[inline(always)]
  pub fn fiforst(&self) -> FIFORST_R {
    FIFORST_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Software Reset"]
  #[inline(always)]
  pub fn swrst(&mut self) -> SWRST_W {
    SWRST_W { w: self }
  }
  #[doc = "Bit 1 - FIFO Reset"]
  #[inline(always)]
  pub fn fiforst(&mut self) -> FIFORST_W {
    FIFORST_W { w: self }
  }
}
