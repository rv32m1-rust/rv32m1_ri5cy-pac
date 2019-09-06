#[doc = "Reader of register FSTDBY"]
pub type R = crate::R<u8, super::FSTDBY>;
#[doc = "Writer for register FSTDBY"]
pub type W = crate::W<u8, super::FSTDBY>;
#[doc = "Register FSTDBY `reset()`'s with value 0"]
impl crate::ResetValue for super::FSTDBY {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Standy Mode for Flash Block 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBY0_A {
  #[doc = "0: Standby mode not enabled for flash block 0"]
  STDBY0_0,
  #[doc = "1: If STDBYDIS is clear, standby mode is enabled for flash block 0 (when SWAP=0/1, flash block 1/0 is the inactive block)"]
  STDBY0_1,
}
impl From<STDBY0_A> for bool {
  #[inline(always)]
  fn from(variant: STDBY0_A) -> Self {
    match variant {
      STDBY0_A::STDBY0_0 => false,
      STDBY0_A::STDBY0_1 => true,
    }
  }
}
#[doc = "Reader of field `STDBY0`"]
pub type STDBY0_R = crate::R<bool, STDBY0_A>;
impl STDBY0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STDBY0_A {
    match self.bits {
      false => STDBY0_A::STDBY0_0,
      true => STDBY0_A::STDBY0_1,
    }
  }
  #[doc = "Checks if the value of the field is `STDBY0_0`"]
  #[inline(always)]
  pub fn is_stdby0_0(&self) -> bool {
    *self == STDBY0_A::STDBY0_0
  }
  #[doc = "Checks if the value of the field is `STDBY0_1`"]
  #[inline(always)]
  pub fn is_stdby0_1(&self) -> bool {
    *self == STDBY0_A::STDBY0_1
  }
}
#[doc = "Write proxy for field `STDBY0`"]
pub struct STDBY0_W<'a> {
  w: &'a mut W,
}
impl<'a> STDBY0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STDBY0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Standby mode not enabled for flash block 0"]
  #[inline(always)]
  pub fn stdby0_0(self) -> &'a mut W {
    self.variant(STDBY0_A::STDBY0_0)
  }
  #[doc = "If STDBYDIS is clear, standby mode is enabled for flash block 0 (when SWAP=0/1, flash block 1/0 is the inactive block)"]
  #[inline(always)]
  pub fn stdby0_1(self) -> &'a mut W {
    self.variant(STDBY0_A::STDBY0_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
    self.w
  }
}
#[doc = "Standy Mode for Flash Block 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBY1_A {
  #[doc = "0: Standby mode not enabled for flash block 1"]
  STDBY1_0,
  #[doc = "1: If STDBYDIS is clear, standby mode is enabled for flash block 1 (when SWAP=0/1, flash block 1/0 is the inactive block)"]
  STDBY1_1,
}
impl From<STDBY1_A> for bool {
  #[inline(always)]
  fn from(variant: STDBY1_A) -> Self {
    match variant {
      STDBY1_A::STDBY1_0 => false,
      STDBY1_A::STDBY1_1 => true,
    }
  }
}
#[doc = "Reader of field `STDBY1`"]
pub type STDBY1_R = crate::R<bool, STDBY1_A>;
impl STDBY1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STDBY1_A {
    match self.bits {
      false => STDBY1_A::STDBY1_0,
      true => STDBY1_A::STDBY1_1,
    }
  }
  #[doc = "Checks if the value of the field is `STDBY1_0`"]
  #[inline(always)]
  pub fn is_stdby1_0(&self) -> bool {
    *self == STDBY1_A::STDBY1_0
  }
  #[doc = "Checks if the value of the field is `STDBY1_1`"]
  #[inline(always)]
  pub fn is_stdby1_1(&self) -> bool {
    *self == STDBY1_A::STDBY1_1
  }
}
#[doc = "Write proxy for field `STDBY1`"]
pub struct STDBY1_W<'a> {
  w: &'a mut W,
}
impl<'a> STDBY1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STDBY1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Standby mode not enabled for flash block 1"]
  #[inline(always)]
  pub fn stdby1_0(self) -> &'a mut W {
    self.variant(STDBY1_A::STDBY1_0)
  }
  #[doc = "If STDBYDIS is clear, standby mode is enabled for flash block 1 (when SWAP=0/1, flash block 1/0 is the inactive block)"]
  #[inline(always)]
  pub fn stdby1_1(self) -> &'a mut W {
    self.variant(STDBY1_A::STDBY1_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
    self.w
  }
}
#[doc = "Standy Mode for Flash Block 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBY2_A {
  #[doc = "0: Standby mode not enabled for flash block 2"]
  STDBY2_0,
  #[doc = "1: If STDBYDIS is clear, standby mode is enabled for flash block 2"]
  STDBY2_1,
}
impl From<STDBY2_A> for bool {
  #[inline(always)]
  fn from(variant: STDBY2_A) -> Self {
    match variant {
      STDBY2_A::STDBY2_0 => false,
      STDBY2_A::STDBY2_1 => true,
    }
  }
}
#[doc = "Reader of field `STDBY2`"]
pub type STDBY2_R = crate::R<bool, STDBY2_A>;
impl STDBY2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STDBY2_A {
    match self.bits {
      false => STDBY2_A::STDBY2_0,
      true => STDBY2_A::STDBY2_1,
    }
  }
  #[doc = "Checks if the value of the field is `STDBY2_0`"]
  #[inline(always)]
  pub fn is_stdby2_0(&self) -> bool {
    *self == STDBY2_A::STDBY2_0
  }
  #[doc = "Checks if the value of the field is `STDBY2_1`"]
  #[inline(always)]
  pub fn is_stdby2_1(&self) -> bool {
    *self == STDBY2_A::STDBY2_1
  }
}
#[doc = "Write proxy for field `STDBY2`"]
pub struct STDBY2_W<'a> {
  w: &'a mut W,
}
impl<'a> STDBY2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STDBY2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Standby mode not enabled for flash block 2"]
  #[inline(always)]
  pub fn stdby2_0(self) -> &'a mut W {
    self.variant(STDBY2_A::STDBY2_0)
  }
  #[doc = "If STDBYDIS is clear, standby mode is enabled for flash block 2"]
  #[inline(always)]
  pub fn stdby2_1(self) -> &'a mut W {
    self.variant(STDBY2_A::STDBY2_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Standy Mode for Flash Block 0"]
  #[inline(always)]
  pub fn stdby0(&self) -> STDBY0_R {
    STDBY0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Standy Mode for Flash Block 1"]
  #[inline(always)]
  pub fn stdby1(&self) -> STDBY1_R {
    STDBY1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Standy Mode for Flash Block 2"]
  #[inline(always)]
  pub fn stdby2(&self) -> STDBY2_R {
    STDBY2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Standy Mode for Flash Block 0"]
  #[inline(always)]
  pub fn stdby0(&mut self) -> STDBY0_W {
    STDBY0_W { w: self }
  }
  #[doc = "Bit 1 - Standy Mode for Flash Block 1"]
  #[inline(always)]
  pub fn stdby1(&mut self) -> STDBY1_W {
    STDBY1_W { w: self }
  }
  #[doc = "Bit 2 - Standy Mode for Flash Block 2"]
  #[inline(always)]
  pub fn stdby2(&mut self) -> STDBY2_W {
    STDBY2_W { w: self }
  }
}
