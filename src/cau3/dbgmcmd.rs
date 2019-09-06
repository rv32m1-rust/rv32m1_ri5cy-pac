#[doc = "Reader of register DBGMCMD"]
pub type R = crate::R<u32, super::DBGMCMD>;
#[doc = "Writer for register DBGMCMD"]
pub type W = crate::W<u32, super::DBGMCMD>;
#[doc = "Register DBGMCMD `reset()`'s with value 0x8800_0000"]
impl crate::ResetValue for super::DBGMCMD {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x8800_0000
  }
}
#[doc = "Instruction/Data Memory Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DM_A {
  #[doc = "0: IMEM is selected"]
  DM_0,
  #[doc = "1: DMEM is selected"]
  DM_1,
}
impl From<DM_A> for bool {
  #[inline(always)]
  fn from(variant: DM_A) -> Self {
    match variant {
      DM_A::DM_0 => false,
      DM_A::DM_1 => true,
    }
  }
}
#[doc = "Reader of field `DM`"]
pub type DM_R = crate::R<bool, DM_A>;
impl DM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DM_A {
    match self.bits {
      false => DM_A::DM_0,
      true => DM_A::DM_1,
    }
  }
  #[doc = "Checks if the value of the field is `DM_0`"]
  #[inline(always)]
  pub fn is_dm_0(&self) -> bool {
    *self == DM_A::DM_0
  }
  #[doc = "Checks if the value of the field is `DM_1`"]
  #[inline(always)]
  pub fn is_dm_1(&self) -> bool {
    *self == DM_A::DM_1
  }
}
#[doc = "Write proxy for field `DM`"]
pub struct DM_W<'a> {
  w: &'a mut W,
}
impl<'a> DM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "IMEM is selected"]
  #[inline(always)]
  pub fn dm_0(self) -> &'a mut W {
    self.variant(DM_A::DM_0)
  }
  #[doc = "DMEM is selected"]
  #[inline(always)]
  pub fn dm_1(self) -> &'a mut W {
    self.variant(DM_A::DM_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
    self.w
  }
}
#[doc = "Increment Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IA_A {
  #[doc = "0: Address is not incremented"]
  IA_0,
  #[doc = "1: Address is incremented after the access"]
  IA_1,
}
impl From<IA_A> for bool {
  #[inline(always)]
  fn from(variant: IA_A) -> Self {
    match variant {
      IA_A::IA_0 => false,
      IA_A::IA_1 => true,
    }
  }
}
#[doc = "Reader of field `IA`"]
pub type IA_R = crate::R<bool, IA_A>;
impl IA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IA_A {
    match self.bits {
      false => IA_A::IA_0,
      true => IA_A::IA_1,
    }
  }
  #[doc = "Checks if the value of the field is `IA_0`"]
  #[inline(always)]
  pub fn is_ia_0(&self) -> bool {
    *self == IA_A::IA_0
  }
  #[doc = "Checks if the value of the field is `IA_1`"]
  #[inline(always)]
  pub fn is_ia_1(&self) -> bool {
    *self == IA_A::IA_1
  }
}
#[doc = "Write proxy for field `IA`"]
pub struct IA_W<'a> {
  w: &'a mut W,
}
impl<'a> IA_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IA_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Address is not incremented"]
  #[inline(always)]
  pub fn ia_0(self) -> &'a mut W {
    self.variant(IA_A::IA_0)
  }
  #[doc = "Address is incremented after the access"]
  #[inline(always)]
  pub fn ia_1(self) -> &'a mut W {
    self.variant(IA_A::IA_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
    self.w
  }
}
#[doc = "Reader of field `Rb_1`"]
pub type RB_1_R = crate::R<bool, bool>;
#[doc = "Byte Reversal Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BV_A {
  #[doc = "0: DMEM bytes are not reversed"]
  BV_0,
  #[doc = "1: DMEM bytes are reversed"]
  BV_1,
}
impl From<BV_A> for bool {
  #[inline(always)]
  fn from(variant: BV_A) -> Self {
    match variant {
      BV_A::BV_0 => false,
      BV_A::BV_1 => true,
    }
  }
}
#[doc = "Reader of field `BV`"]
pub type BV_R = crate::R<bool, BV_A>;
impl BV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BV_A {
    match self.bits {
      false => BV_A::BV_0,
      true => BV_A::BV_1,
    }
  }
  #[doc = "Checks if the value of the field is `BV_0`"]
  #[inline(always)]
  pub fn is_bv_0(&self) -> bool {
    *self == BV_A::BV_0
  }
  #[doc = "Checks if the value of the field is `BV_1`"]
  #[inline(always)]
  pub fn is_bv_1(&self) -> bool {
    *self == BV_A::BV_1
  }
}
#[doc = "Write proxy for field `BV`"]
pub struct BV_W<'a> {
  w: &'a mut W,
}
impl<'a> BV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BV_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DMEM bytes are not reversed"]
  #[inline(always)]
  pub fn bv_0(self) -> &'a mut W {
    self.variant(BV_A::BV_0)
  }
  #[doc = "DMEM bytes are reversed"]
  #[inline(always)]
  pub fn bv_1(self) -> &'a mut W {
    self.variant(BV_A::BV_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
    self.w
  }
}
#[doc = "Reader of field `R_0`"]
pub type R_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `R_1`"]
pub type R_1_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 24 - Instruction/Data Memory Selection"]
  #[inline(always)]
  pub fn dm(&self) -> DM_R {
    DM_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Increment Address"]
  #[inline(always)]
  pub fn ia(&self) -> IA_R {
    IA_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - Read always as 1"]
  #[inline(always)]
  pub fn rb_1(&self) -> RB_1_R {
    RB_1_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Byte Reversal Control"]
  #[inline(always)]
  pub fn bv(&self) -> BV_R {
    BV_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Read always as 0"]
  #[inline(always)]
  pub fn r_0(&self) -> R_0_R {
    R_0_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Read always as 1"]
  #[inline(always)]
  pub fn r_1(&self) -> R_1_R {
    R_1_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 24 - Instruction/Data Memory Selection"]
  #[inline(always)]
  pub fn dm(&mut self) -> DM_W {
    DM_W { w: self }
  }
  #[doc = "Bit 26 - Increment Address"]
  #[inline(always)]
  pub fn ia(&mut self) -> IA_W {
    IA_W { w: self }
  }
  #[doc = "Bit 28 - Byte Reversal Control"]
  #[inline(always)]
  pub fn bv(&mut self) -> BV_W {
    BV_W { w: self }
  }
}
