#[doc = "Reader of register MDA_W0_34_DFMT1"]
pub type R = crate::R<u32, super::MDA_W0_34_DFMT1>;
#[doc = "Writer for register MDA_W0_34_DFMT1"]
pub type W = crate::W<u32, super::MDA_W0_34_DFMT1>;
#[doc = "Register MDA_W0_34_DFMT1 `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::MDA_W0_34_DFMT1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x2000_0000
  }
}
#[doc = "Reader of field `DID`"]
pub type DID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DID`"]
pub struct DID_W<'a> {
  w: &'a mut W,
}
impl<'a> DID_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
    self.w
  }
}
#[doc = "Privileged attribute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_A {
  #[doc = "0: Force the bus attribute for this master to user."]
  PA_0,
  #[doc = "1: Force the bus attribute for this master to privileged."]
  PA_1,
  #[doc = "2: Use the bus master's privileged/user attribute directly."]
  PA_2,
  #[doc = "3: Use the bus master's privileged/user attribute directly."]
  PA_3,
}
impl From<PA_A> for u8 {
  #[inline(always)]
  fn from(variant: PA_A) -> Self {
    match variant {
      PA_A::PA_0 => 0,
      PA_A::PA_1 => 1,
      PA_A::PA_2 => 2,
      PA_A::PA_3 => 3,
    }
  }
}
#[doc = "Reader of field `PA`"]
pub type PA_R = crate::R<u8, PA_A>;
impl PA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PA_A {
    match self.bits {
      0 => PA_A::PA_0,
      1 => PA_A::PA_1,
      2 => PA_A::PA_2,
      3 => PA_A::PA_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PA_0`"]
  #[inline(always)]
  pub fn is_pa_0(&self) -> bool {
    *self == PA_A::PA_0
  }
  #[doc = "Checks if the value of the field is `PA_1`"]
  #[inline(always)]
  pub fn is_pa_1(&self) -> bool {
    *self == PA_A::PA_1
  }
  #[doc = "Checks if the value of the field is `PA_2`"]
  #[inline(always)]
  pub fn is_pa_2(&self) -> bool {
    *self == PA_A::PA_2
  }
  #[doc = "Checks if the value of the field is `PA_3`"]
  #[inline(always)]
  pub fn is_pa_3(&self) -> bool {
    *self == PA_A::PA_3
  }
}
#[doc = "Write proxy for field `PA`"]
pub struct PA_W<'a> {
  w: &'a mut W,
}
impl<'a> PA_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PA_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Force the bus attribute for this master to user."]
  #[inline(always)]
  pub fn pa_0(self) -> &'a mut W {
    self.variant(PA_A::PA_0)
  }
  #[doc = "Force the bus attribute for this master to privileged."]
  #[inline(always)]
  pub fn pa_1(self) -> &'a mut W {
    self.variant(PA_A::PA_1)
  }
  #[doc = "Use the bus master's privileged/user attribute directly."]
  #[inline(always)]
  pub fn pa_2(self) -> &'a mut W {
    self.variant(PA_A::PA_2)
  }
  #[doc = "Use the bus master's privileged/user attribute directly."]
  #[inline(always)]
  pub fn pa_3(self) -> &'a mut W {
    self.variant(PA_A::PA_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Secure attribute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_A {
  #[doc = "0: Force the bus attribute for this master to secure."]
  SA_0,
  #[doc = "1: Force the bus attribute for this master to nonsecure."]
  SA_1,
  #[doc = "2: Use the bus master's secure/nonsecure attribute directly."]
  SA_2,
  #[doc = "3: Use the bus master's secure/nonsecure attribute directly."]
  SA_3,
}
impl From<SA_A> for u8 {
  #[inline(always)]
  fn from(variant: SA_A) -> Self {
    match variant {
      SA_A::SA_0 => 0,
      SA_A::SA_1 => 1,
      SA_A::SA_2 => 2,
      SA_A::SA_3 => 3,
    }
  }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<u8, SA_A>;
impl SA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SA_A {
    match self.bits {
      0 => SA_A::SA_0,
      1 => SA_A::SA_1,
      2 => SA_A::SA_2,
      3 => SA_A::SA_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SA_0`"]
  #[inline(always)]
  pub fn is_sa_0(&self) -> bool {
    *self == SA_A::SA_0
  }
  #[doc = "Checks if the value of the field is `SA_1`"]
  #[inline(always)]
  pub fn is_sa_1(&self) -> bool {
    *self == SA_A::SA_1
  }
  #[doc = "Checks if the value of the field is `SA_2`"]
  #[inline(always)]
  pub fn is_sa_2(&self) -> bool {
    *self == SA_A::SA_2
  }
  #[doc = "Checks if the value of the field is `SA_3`"]
  #[inline(always)]
  pub fn is_sa_3(&self) -> bool {
    *self == SA_A::SA_3
  }
}
#[doc = "Write proxy for field `SA`"]
pub struct SA_W<'a> {
  w: &'a mut W,
}
impl<'a> SA_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SA_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Force the bus attribute for this master to secure."]
  #[inline(always)]
  pub fn sa_0(self) -> &'a mut W {
    self.variant(SA_A::SA_0)
  }
  #[doc = "Force the bus attribute for this master to nonsecure."]
  #[inline(always)]
  pub fn sa_1(self) -> &'a mut W {
    self.variant(SA_A::SA_1)
  }
  #[doc = "Use the bus master's secure/nonsecure attribute directly."]
  #[inline(always)]
  pub fn sa_2(self) -> &'a mut W {
    self.variant(SA_A::SA_2)
  }
  #[doc = "Use the bus master's secure/nonsecure attribute directly."]
  #[inline(always)]
  pub fn sa_3(self) -> &'a mut W {
    self.variant(SA_A::SA_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "DID Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIDB_A {
  #[doc = "0: Use MDAn\\[3:0\\] as the domain identifier."]
  DIDB_0,
  #[doc = "1: Use the DID input as the domain identifier."]
  DIDB_1,
}
impl From<DIDB_A> for bool {
  #[inline(always)]
  fn from(variant: DIDB_A) -> Self {
    match variant {
      DIDB_A::DIDB_0 => false,
      DIDB_A::DIDB_1 => true,
    }
  }
}
#[doc = "Reader of field `DIDB`"]
pub type DIDB_R = crate::R<bool, DIDB_A>;
impl DIDB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIDB_A {
    match self.bits {
      false => DIDB_A::DIDB_0,
      true => DIDB_A::DIDB_1,
    }
  }
  #[doc = "Checks if the value of the field is `DIDB_0`"]
  #[inline(always)]
  pub fn is_didb_0(&self) -> bool {
    *self == DIDB_A::DIDB_0
  }
  #[doc = "Checks if the value of the field is `DIDB_1`"]
  #[inline(always)]
  pub fn is_didb_1(&self) -> bool {
    *self == DIDB_A::DIDB_1
  }
}
#[doc = "Write proxy for field `DIDB`"]
pub struct DIDB_W<'a> {
  w: &'a mut W,
}
impl<'a> DIDB_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DIDB_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
  #[inline(always)]
  pub fn didb_0(self) -> &'a mut W {
    self.variant(DIDB_A::DIDB_0)
  }
  #[doc = "Use the DID input as the domain identifier."]
  #[inline(always)]
  pub fn didb_1(self) -> &'a mut W {
    self.variant(DIDB_A::DIDB_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
    self.w
  }
}
#[doc = "Domain format\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFMT_A {
  #[doc = "0: no description available"]
  DFMT_0,
  #[doc = "1: no description available"]
  DFMT_1,
}
impl From<DFMT_A> for bool {
  #[inline(always)]
  fn from(variant: DFMT_A) -> Self {
    match variant {
      DFMT_A::DFMT_0 => false,
      DFMT_A::DFMT_1 => true,
    }
  }
}
#[doc = "Reader of field `DFMT`"]
pub type DFMT_R = crate::R<bool, DFMT_A>;
impl DFMT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DFMT_A {
    match self.bits {
      false => DFMT_A::DFMT_0,
      true => DFMT_A::DFMT_1,
    }
  }
  #[doc = "Checks if the value of the field is `DFMT_0`"]
  #[inline(always)]
  pub fn is_dfmt_0(&self) -> bool {
    *self == DFMT_A::DFMT_0
  }
  #[doc = "Checks if the value of the field is `DFMT_1`"]
  #[inline(always)]
  pub fn is_dfmt_1(&self) -> bool {
    *self == DFMT_A::DFMT_1
  }
}
#[doc = "1-bit Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK1_A {
  #[doc = "0: Register can be written by any secure privileged write."]
  LK1_0,
  #[doc = "1: Register is locked (read-only) until the next reset."]
  LK1_1,
}
impl From<LK1_A> for bool {
  #[inline(always)]
  fn from(variant: LK1_A) -> Self {
    match variant {
      LK1_A::LK1_0 => false,
      LK1_A::LK1_1 => true,
    }
  }
}
#[doc = "Reader of field `LK1`"]
pub type LK1_R = crate::R<bool, LK1_A>;
impl LK1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK1_A {
    match self.bits {
      false => LK1_A::LK1_0,
      true => LK1_A::LK1_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK1_0`"]
  #[inline(always)]
  pub fn is_lk1_0(&self) -> bool {
    *self == LK1_A::LK1_0
  }
  #[doc = "Checks if the value of the field is `LK1_1`"]
  #[inline(always)]
  pub fn is_lk1_1(&self) -> bool {
    *self == LK1_A::LK1_1
  }
}
#[doc = "Write proxy for field `LK1`"]
pub struct LK1_W<'a> {
  w: &'a mut W,
}
impl<'a> LK1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Register can be written by any secure privileged write."]
  #[inline(always)]
  pub fn lk1_0(self) -> &'a mut W {
    self.variant(LK1_A::LK1_0)
  }
  #[doc = "Register is locked (read-only) until the next reset."]
  #[inline(always)]
  pub fn lk1_1(self) -> &'a mut W {
    self.variant(LK1_A::LK1_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLD_A {
  #[doc = "0: The Wr domain assignment is invalid."]
  VLD_0,
  #[doc = "1: The Wr domain assignment is valid."]
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
  #[doc = "The Wr domain assignment is invalid."]
  #[inline(always)]
  pub fn vld_0(self) -> &'a mut W {
    self.variant(VLD_A::VLD_0)
  }
  #[doc = "The Wr domain assignment is valid."]
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
  #[doc = "Bits 0:3 - Domain identifier"]
  #[inline(always)]
  pub fn did(&self) -> DID_R {
    DID_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 4:5 - Privileged attribute"]
  #[inline(always)]
  pub fn pa(&self) -> PA_R {
    PA_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Secure attribute"]
  #[inline(always)]
  pub fn sa(&self) -> SA_R {
    SA_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bit 8 - DID Bypass"]
  #[inline(always)]
  pub fn didb(&self) -> DIDB_R {
    DIDB_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Domain format"]
  #[inline(always)]
  pub fn dfmt(&self) -> DFMT_R {
    DFMT_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - 1-bit Lock"]
  #[inline(always)]
  pub fn lk1(&self) -> LK1_R {
    LK1_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Valid"]
  #[inline(always)]
  pub fn vld(&self) -> VLD_R {
    VLD_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:3 - Domain identifier"]
  #[inline(always)]
  pub fn did(&mut self) -> DID_W {
    DID_W { w: self }
  }
  #[doc = "Bits 4:5 - Privileged attribute"]
  #[inline(always)]
  pub fn pa(&mut self) -> PA_W {
    PA_W { w: self }
  }
  #[doc = "Bits 6:7 - Secure attribute"]
  #[inline(always)]
  pub fn sa(&mut self) -> SA_W {
    SA_W { w: self }
  }
  #[doc = "Bit 8 - DID Bypass"]
  #[inline(always)]
  pub fn didb(&mut self) -> DIDB_W {
    DIDB_W { w: self }
  }
  #[doc = "Bit 30 - 1-bit Lock"]
  #[inline(always)]
  pub fn lk1(&mut self) -> LK1_W {
    LK1_W { w: self }
  }
  #[doc = "Bit 31 - Valid"]
  #[inline(always)]
  pub fn vld(&mut self) -> VLD_W {
    VLD_W { w: self }
  }
}
