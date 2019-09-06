#[doc = "Reader of register PDAC_W1_0_1"]
pub type R = crate::R<u32, super::PDAC_W1_0_1>;
#[doc = "Writer for register PDAC_W1_0_1"]
pub type W = crate::W<u32, super::PDAC_W1_0_1>;
#[doc = "Register PDAC_W1_0_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDAC_W1_0_1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Exclusive Access Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EAL_A {
  #[doc = "0: Lock disabled"]
  EAL_0,
  #[doc = "1: Lock disabled until next reset"]
  EAL_1,
  #[doc = "2: Lock enabled, lock state = available"]
  EAL_2,
  #[doc = "3: Lock enabled, lock state = not available"]
  EAL_3,
}
impl From<EAL_A> for u8 {
  #[inline(always)]
  fn from(variant: EAL_A) -> Self {
    match variant {
      EAL_A::EAL_0 => 0,
      EAL_A::EAL_1 => 1,
      EAL_A::EAL_2 => 2,
      EAL_A::EAL_3 => 3,
    }
  }
}
#[doc = "Reader of field `EAL`"]
pub type EAL_R = crate::R<u8, EAL_A>;
impl EAL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EAL_A {
    match self.bits {
      0 => EAL_A::EAL_0,
      1 => EAL_A::EAL_1,
      2 => EAL_A::EAL_2,
      3 => EAL_A::EAL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `EAL_0`"]
  #[inline(always)]
  pub fn is_eal_0(&self) -> bool {
    *self == EAL_A::EAL_0
  }
  #[doc = "Checks if the value of the field is `EAL_1`"]
  #[inline(always)]
  pub fn is_eal_1(&self) -> bool {
    *self == EAL_A::EAL_1
  }
  #[doc = "Checks if the value of the field is `EAL_2`"]
  #[inline(always)]
  pub fn is_eal_2(&self) -> bool {
    *self == EAL_A::EAL_2
  }
  #[doc = "Checks if the value of the field is `EAL_3`"]
  #[inline(always)]
  pub fn is_eal_3(&self) -> bool {
    *self == EAL_A::EAL_3
  }
}
#[doc = "Write proxy for field `EAL`"]
pub struct EAL_W<'a> {
  w: &'a mut W,
}
impl<'a> EAL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EAL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Lock disabled"]
  #[inline(always)]
  pub fn eal_0(self) -> &'a mut W {
    self.variant(EAL_A::EAL_0)
  }
  #[doc = "Lock disabled until next reset"]
  #[inline(always)]
  pub fn eal_1(self) -> &'a mut W {
    self.variant(EAL_A::EAL_1)
  }
  #[doc = "Lock enabled, lock state = available"]
  #[inline(always)]
  pub fn eal_2(self) -> &'a mut W {
    self.variant(EAL_A::EAL_2)
  }
  #[doc = "Lock enabled, lock state = not available"]
  #[inline(always)]
  pub fn eal_3(self) -> &'a mut W {
    self.variant(EAL_A::EAL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
#[doc = "Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK2_A {
  #[doc = "0: Entire PDACs can be written."]
  LK2_0,
  #[doc = "1: Entire PDACs can be written."]
  LK2_1,
  #[doc = "2: Domain x can only update the DxACP field and the LK2 field; no other PDACs fields can be written."]
  LK2_2,
  #[doc = "3: PDACs is locked (read-only) until the next reset."]
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
  #[doc = "Entire PDACs can be written."]
  #[inline(always)]
  pub fn lk2_0(self) -> &'a mut W {
    self.variant(LK2_A::LK2_0)
  }
  #[doc = "Entire PDACs can be written."]
  #[inline(always)]
  pub fn lk2_1(self) -> &'a mut W {
    self.variant(LK2_A::LK2_1)
  }
  #[doc = "Domain x can only update the DxACP field and the LK2 field; no other PDACs fields can be written."]
  #[inline(always)]
  pub fn lk2_2(self) -> &'a mut W {
    self.variant(LK2_A::LK2_2)
  }
  #[doc = "PDACs is locked (read-only) until the next reset."]
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
  #[doc = "0: The PDACs assignment is invalid."]
  VLD_0,
  #[doc = "1: The PDACs assignment is valid."]
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
  #[doc = "The PDACs assignment is invalid."]
  #[inline(always)]
  pub fn vld_0(self) -> &'a mut W {
    self.variant(VLD_A::VLD_0)
  }
  #[doc = "The PDACs assignment is valid."]
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
  #[doc = "Bits 24:25 - Exclusive Access Lock"]
  #[inline(always)]
  pub fn eal(&self) -> EAL_R {
    EAL_R::new(((self.bits >> 24) & 0x03) as u8)
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
  #[doc = "Bits 24:25 - Exclusive Access Lock"]
  #[inline(always)]
  pub fn eal(&mut self) -> EAL_W {
    EAL_W { w: self }
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
