#[doc = "Reader of register MRGD_W4_0_5"]
pub type R = crate::R<u32, super::MRGD_W4_0_5>;
#[doc = "Writer for register MRGD_W4_0_5"]
pub type W = crate::W<u32, super::MRGD_W4_0_5>;
#[doc = "Register MRGD_W4_0_5 `reset()`'s with value 0"]
impl crate::ResetValue for super::MRGD_W4_0_5 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `ACCSET1`"]
pub type ACCSET1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ACCSET1`"]
pub struct ACCSET1_W<'a> {
  w: &'a mut W,
}
impl<'a> ACCSET1_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
    self.w
  }
}
#[doc = "Lock ACCSET1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKAS1_A {
  #[doc = "0: Writes to ACCSET1 affect lesser modes"]
  LKAS1_0,
  #[doc = "1: ACCSET1 cannot be modified"]
  LKAS1_1,
}
impl From<LKAS1_A> for bool {
  #[inline(always)]
  fn from(variant: LKAS1_A) -> Self {
    match variant {
      LKAS1_A::LKAS1_0 => false,
      LKAS1_A::LKAS1_1 => true,
    }
  }
}
#[doc = "Reader of field `LKAS1`"]
pub type LKAS1_R = crate::R<bool, LKAS1_A>;
impl LKAS1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LKAS1_A {
    match self.bits {
      false => LKAS1_A::LKAS1_0,
      true => LKAS1_A::LKAS1_1,
    }
  }
  #[doc = "Checks if the value of the field is `LKAS1_0`"]
  #[inline(always)]
  pub fn is_lkas1_0(&self) -> bool {
    *self == LKAS1_A::LKAS1_0
  }
  #[doc = "Checks if the value of the field is `LKAS1_1`"]
  #[inline(always)]
  pub fn is_lkas1_1(&self) -> bool {
    *self == LKAS1_A::LKAS1_1
  }
}
#[doc = "Write proxy for field `LKAS1`"]
pub struct LKAS1_W<'a> {
  w: &'a mut W,
}
impl<'a> LKAS1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LKAS1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to ACCSET1 affect lesser modes"]
  #[inline(always)]
  pub fn lkas1_0(self) -> &'a mut W {
    self.variant(LKAS1_A::LKAS1_0)
  }
  #[doc = "ACCSET1 cannot be modified"]
  #[inline(always)]
  pub fn lkas1_1(self) -> &'a mut W {
    self.variant(LKAS1_A::LKAS1_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "Reader of field `ACCSET2`"]
pub type ACCSET2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ACCSET2`"]
pub struct ACCSET2_W<'a> {
  w: &'a mut W,
}
impl<'a> ACCSET2_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
    self.w
  }
}
#[doc = "Lock ACCSET2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKAS2_A {
  #[doc = "0: Writes to ACCSET2 affect lesser modes"]
  LKAS2_0,
  #[doc = "1: ACCSET2 cannot be modified"]
  LKAS2_1,
}
impl From<LKAS2_A> for bool {
  #[inline(always)]
  fn from(variant: LKAS2_A) -> Self {
    match variant {
      LKAS2_A::LKAS2_0 => false,
      LKAS2_A::LKAS2_1 => true,
    }
  }
}
#[doc = "Reader of field `LKAS2`"]
pub type LKAS2_R = crate::R<bool, LKAS2_A>;
impl LKAS2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LKAS2_A {
    match self.bits {
      false => LKAS2_A::LKAS2_0,
      true => LKAS2_A::LKAS2_1,
    }
  }
  #[doc = "Checks if the value of the field is `LKAS2_0`"]
  #[inline(always)]
  pub fn is_lkas2_0(&self) -> bool {
    *self == LKAS2_A::LKAS2_0
  }
  #[doc = "Checks if the value of the field is `LKAS2_1`"]
  #[inline(always)]
  pub fn is_lkas2_1(&self) -> bool {
    *self == LKAS2_A::LKAS2_1
  }
}
#[doc = "Write proxy for field `LKAS2`"]
pub struct LKAS2_W<'a> {
  w: &'a mut W,
}
impl<'a> LKAS2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LKAS2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to ACCSET2 affect lesser modes"]
  #[inline(always)]
  pub fn lkas2_0(self) -> &'a mut W {
    self.variant(LKAS2_A::LKAS2_0)
  }
  #[doc = "ACCSET2 cannot be modified"]
  #[inline(always)]
  pub fn lkas2_1(self) -> &'a mut W {
    self.variant(LKAS2_A::LKAS2_1)
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
#[doc = "Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK2_A {
  #[doc = "0: no description available"]
  LK2_0,
  #[doc = "1: no description available"]
  LK2_1,
  #[doc = "2: no description available"]
  LK2_2,
  #[doc = "3: no description available"]
  LK2_3,
}
impl From<LK2_A> for u8 {
  #[inline(always)]
  fn from(variant: LK2_A) -> Self {
    match variant {
      LK2_A::LK2_0 => 0,
      LK2_A::LK2_1 => 1,
      LK2_A::LK2_2 => 2,
      LK2_A::LK2_3 => 3,
    }
  }
}
#[doc = "Reader of field `LK2`"]
pub type LK2_R = crate::R<u8, LK2_A>;
impl LK2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK2_A {
    match self.bits {
      0 => LK2_A::LK2_0,
      1 => LK2_A::LK2_1,
      2 => LK2_A::LK2_2,
      3 => LK2_A::LK2_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `LK2_0`"]
  #[inline(always)]
  pub fn is_lk2_0(&self) -> bool {
    *self == LK2_A::LK2_0
  }
  #[doc = "Checks if the value of the field is `LK2_1`"]
  #[inline(always)]
  pub fn is_lk2_1(&self) -> bool {
    *self == LK2_A::LK2_1
  }
  #[doc = "Checks if the value of the field is `LK2_2`"]
  #[inline(always)]
  pub fn is_lk2_2(&self) -> bool {
    *self == LK2_A::LK2_2
  }
  #[doc = "Checks if the value of the field is `LK2_3`"]
  #[inline(always)]
  pub fn is_lk2_3(&self) -> bool {
    *self == LK2_A::LK2_3
  }
}
#[doc = "Write proxy for field `LK2`"]
pub struct LK2_W<'a> {
  w: &'a mut W,
}
impl<'a> LK2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn lk2_0(self) -> &'a mut W {
    self.variant(LK2_A::LK2_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn lk2_1(self) -> &'a mut W {
    self.variant(LK2_A::LK2_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn lk2_2(self) -> &'a mut W {
    self.variant(LK2_A::LK2_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn lk2_3(self) -> &'a mut W {
    self.variant(LK2_A::LK2_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
    self.w
  }
}
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLD_A {
  #[doc = "0: no description available"]
  VLD_0,
  #[doc = "1: no description available"]
  VLD_1,
}
impl From<VLD_A> for bool {
  #[inline(always)]
  fn from(variant: VLD_A) -> Self {
    match variant {
      VLD_A::VLD_0 => false,
      VLD_A::VLD_1 => true,
    }
  }
}
#[doc = "Reader of field `VLD`"]
pub type VLD_R = crate::R<bool, VLD_A>;
impl VLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VLD_A {
    match self.bits {
      false => VLD_A::VLD_0,
      true => VLD_A::VLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `VLD_0`"]
  #[inline(always)]
  pub fn is_vld_0(&self) -> bool {
    *self == VLD_A::VLD_0
  }
  #[doc = "Checks if the value of the field is `VLD_1`"]
  #[inline(always)]
  pub fn is_vld_1(&self) -> bool {
    *self == VLD_A::VLD_1
  }
}
#[doc = "Write proxy for field `VLD`"]
pub struct VLD_W<'a> {
  w: &'a mut W,
}
impl<'a> VLD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VLD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vld_0(self) -> &'a mut W {
    self.variant(VLD_A::VLD_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vld_1(self) -> &'a mut W {
    self.variant(VLD_A::VLD_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:11 - SET 1 of Programmable access flags."]
  #[inline(always)]
  pub fn accset1(&self) -> ACCSET1_R {
    ACCSET1_R::new((self.bits & 0x0fff) as u16)
  }
  #[doc = "Bit 12 - Lock ACCSET1"]
  #[inline(always)]
  pub fn lkas1(&self) -> LKAS1_R {
    LKAS1_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bits 16:27 - SET 2 of Programmable access flags."]
  #[inline(always)]
  pub fn accset2(&self) -> ACCSET2_R {
    ACCSET2_R::new(((self.bits >> 16) & 0x0fff) as u16)
  }
  #[doc = "Bit 28 - Lock ACCSET2"]
  #[inline(always)]
  pub fn lkas2(&self) -> LKAS2_R {
    LKAS2_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bits 29:30 - Lock"]
  #[inline(always)]
  pub fn lk2(&self) -> LK2_R {
    LK2_R::new(((self.bits >> 29) & 0x03) as u8)
  }
  #[doc = "Bit 31 - Valid"]
  #[inline(always)]
  pub fn vld(&self) -> VLD_R {
    VLD_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:11 - SET 1 of Programmable access flags."]
  #[inline(always)]
  pub fn accset1(&mut self) -> ACCSET1_W {
    ACCSET1_W { w: self }
  }
  #[doc = "Bit 12 - Lock ACCSET1"]
  #[inline(always)]
  pub fn lkas1(&mut self) -> LKAS1_W {
    LKAS1_W { w: self }
  }
  #[doc = "Bits 16:27 - SET 2 of Programmable access flags."]
  #[inline(always)]
  pub fn accset2(&mut self) -> ACCSET2_W {
    ACCSET2_W { w: self }
  }
  #[doc = "Bit 28 - Lock ACCSET2"]
  #[inline(always)]
  pub fn lkas2(&mut self) -> LKAS2_W {
    LKAS2_W { w: self }
  }
  #[doc = "Bits 29:30 - Lock"]
  #[inline(always)]
  pub fn lk2(&mut self) -> LK2_W {
    LK2_W { w: self }
  }
  #[doc = "Bit 31 - Valid"]
  #[inline(always)]
  pub fn vld(&mut self) -> VLD_W {
    VLD_W { w: self }
  }
}
