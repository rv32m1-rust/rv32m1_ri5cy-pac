#[doc = "Reader of register FSTAT"]
pub type R = crate::R<u8, super::FSTAT>;
#[doc = "Writer for register FSTAT"]
pub type W = crate::W<u8, super::FSTAT>;
#[doc = "Register FSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FSTAT {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `MGSTAT0`"]
pub type MGSTAT0_R = crate::R<bool, bool>;
#[doc = "Flash Protection Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPVIOL_A {
  #[doc = "0: No protection violation detected"]
  FPVIOL_0,
  #[doc = "1: Protection violation detected"]
  FPVIOL_1,
}
impl From<FPVIOL_A> for bool {
  #[inline(always)]
  fn from(variant: FPVIOL_A) -> Self {
    match variant {
      FPVIOL_A::FPVIOL_0 => false,
      FPVIOL_A::FPVIOL_1 => true,
    }
  }
}
#[doc = "Reader of field `FPVIOL`"]
pub type FPVIOL_R = crate::R<bool, FPVIOL_A>;
impl FPVIOL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FPVIOL_A {
    match self.bits {
      false => FPVIOL_A::FPVIOL_0,
      true => FPVIOL_A::FPVIOL_1,
    }
  }
  #[doc = "Checks if the value of the field is `FPVIOL_0`"]
  #[inline(always)]
  pub fn is_fpviol_0(&self) -> bool {
    *self == FPVIOL_A::FPVIOL_0
  }
  #[doc = "Checks if the value of the field is `FPVIOL_1`"]
  #[inline(always)]
  pub fn is_fpviol_1(&self) -> bool {
    *self == FPVIOL_A::FPVIOL_1
  }
}
#[doc = "Write proxy for field `FPVIOL`"]
pub struct FPVIOL_W<'a> {
  w: &'a mut W,
}
impl<'a> FPVIOL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FPVIOL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No protection violation detected"]
  #[inline(always)]
  pub fn fpviol_0(self) -> &'a mut W {
    self.variant(FPVIOL_A::FPVIOL_0)
  }
  #[doc = "Protection violation detected"]
  #[inline(always)]
  pub fn fpviol_1(self) -> &'a mut W {
    self.variant(FPVIOL_A::FPVIOL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
    self.w
  }
}
#[doc = "Flash Access Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCERR_A {
  #[doc = "0: No access error detected"]
  ACCERR_0,
  #[doc = "1: Access error detected"]
  ACCERR_1,
}
impl From<ACCERR_A> for bool {
  #[inline(always)]
  fn from(variant: ACCERR_A) -> Self {
    match variant {
      ACCERR_A::ACCERR_0 => false,
      ACCERR_A::ACCERR_1 => true,
    }
  }
}
#[doc = "Reader of field `ACCERR`"]
pub type ACCERR_R = crate::R<bool, ACCERR_A>;
impl ACCERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ACCERR_A {
    match self.bits {
      false => ACCERR_A::ACCERR_0,
      true => ACCERR_A::ACCERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `ACCERR_0`"]
  #[inline(always)]
  pub fn is_accerr_0(&self) -> bool {
    *self == ACCERR_A::ACCERR_0
  }
  #[doc = "Checks if the value of the field is `ACCERR_1`"]
  #[inline(always)]
  pub fn is_accerr_1(&self) -> bool {
    *self == ACCERR_A::ACCERR_1
  }
}
#[doc = "Write proxy for field `ACCERR`"]
pub struct ACCERR_W<'a> {
  w: &'a mut W,
}
impl<'a> ACCERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ACCERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No access error detected"]
  #[inline(always)]
  pub fn accerr_0(self) -> &'a mut W {
    self.variant(ACCERR_A::ACCERR_0)
  }
  #[doc = "Access error detected"]
  #[inline(always)]
  pub fn accerr_1(self) -> &'a mut W {
    self.variant(ACCERR_A::ACCERR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
    self.w
  }
}
#[doc = "Flash Read Collision Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLERR_A {
  #[doc = "0: No collision error detected"]
  RDCOLERR_0,
  #[doc = "1: Collision error detected"]
  RDCOLERR_1,
}
impl From<RDCOLERR_A> for bool {
  #[inline(always)]
  fn from(variant: RDCOLERR_A) -> Self {
    match variant {
      RDCOLERR_A::RDCOLERR_0 => false,
      RDCOLERR_A::RDCOLERR_1 => true,
    }
  }
}
#[doc = "Reader of field `RDCOLERR`"]
pub type RDCOLERR_R = crate::R<bool, RDCOLERR_A>;
impl RDCOLERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDCOLERR_A {
    match self.bits {
      false => RDCOLERR_A::RDCOLERR_0,
      true => RDCOLERR_A::RDCOLERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDCOLERR_0`"]
  #[inline(always)]
  pub fn is_rdcolerr_0(&self) -> bool {
    *self == RDCOLERR_A::RDCOLERR_0
  }
  #[doc = "Checks if the value of the field is `RDCOLERR_1`"]
  #[inline(always)]
  pub fn is_rdcolerr_1(&self) -> bool {
    *self == RDCOLERR_A::RDCOLERR_1
  }
}
#[doc = "Write proxy for field `RDCOLERR`"]
pub struct RDCOLERR_W<'a> {
  w: &'a mut W,
}
impl<'a> RDCOLERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RDCOLERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No collision error detected"]
  #[inline(always)]
  pub fn rdcolerr_0(self) -> &'a mut W {
    self.variant(RDCOLERR_A::RDCOLERR_0)
  }
  #[doc = "Collision error detected"]
  #[inline(always)]
  pub fn rdcolerr_1(self) -> &'a mut W {
    self.variant(RDCOLERR_A::RDCOLERR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
#[doc = "Command Complete Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIF_A {
  #[doc = "0: no description available"]
  CCIF_0,
  #[doc = "1: no description available"]
  CCIF_1,
}
impl From<CCIF_A> for bool {
  #[inline(always)]
  fn from(variant: CCIF_A) -> Self {
    match variant {
      CCIF_A::CCIF_0 => false,
      CCIF_A::CCIF_1 => true,
    }
  }
}
#[doc = "Reader of field `CCIF`"]
pub type CCIF_R = crate::R<bool, CCIF_A>;
impl CCIF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CCIF_A {
    match self.bits {
      false => CCIF_A::CCIF_0,
      true => CCIF_A::CCIF_1,
    }
  }
  #[doc = "Checks if the value of the field is `CCIF_0`"]
  #[inline(always)]
  pub fn is_ccif_0(&self) -> bool {
    *self == CCIF_A::CCIF_0
  }
  #[doc = "Checks if the value of the field is `CCIF_1`"]
  #[inline(always)]
  pub fn is_ccif_1(&self) -> bool {
    *self == CCIF_A::CCIF_1
  }
}
#[doc = "Write proxy for field `CCIF`"]
pub struct CCIF_W<'a> {
  w: &'a mut W,
}
impl<'a> CCIF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CCIF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn ccif_0(self) -> &'a mut W {
    self.variant(CCIF_A::CCIF_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn ccif_1(self) -> &'a mut W {
    self.variant(CCIF_A::CCIF_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Memory Controller Command Completion Status Flag"]
  #[inline(always)]
  pub fn mgstat0(&self) -> MGSTAT0_R {
    MGSTAT0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 4 - Flash Protection Violation Flag"]
  #[inline(always)]
  pub fn fpviol(&self) -> FPVIOL_R {
    FPVIOL_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Flash Access Error Flag"]
  #[inline(always)]
  pub fn accerr(&self) -> ACCERR_R {
    ACCERR_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Flash Read Collision Error Flag"]
  #[inline(always)]
  pub fn rdcolerr(&self) -> RDCOLERR_R {
    RDCOLERR_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Command Complete Interrupt Flag"]
  #[inline(always)]
  pub fn ccif(&self) -> CCIF_R {
    CCIF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - Flash Protection Violation Flag"]
  #[inline(always)]
  pub fn fpviol(&mut self) -> FPVIOL_W {
    FPVIOL_W { w: self }
  }
  #[doc = "Bit 5 - Flash Access Error Flag"]
  #[inline(always)]
  pub fn accerr(&mut self) -> ACCERR_W {
    ACCERR_W { w: self }
  }
  #[doc = "Bit 6 - Flash Read Collision Error Flag"]
  #[inline(always)]
  pub fn rdcolerr(&mut self) -> RDCOLERR_W {
    RDCOLERR_W { w: self }
  }
  #[doc = "Bit 7 - Command Complete Interrupt Flag"]
  #[inline(always)]
  pub fn ccif(&mut self) -> CCIF_W {
    CCIF_W { w: self }
  }
}
