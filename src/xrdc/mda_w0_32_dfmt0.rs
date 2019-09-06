#[doc = "Reader of register MDA_W0_32_DFMT0"]
pub type R = crate::R<u32, super::MDA_W0_32_DFMT0>;
#[doc = "Writer for register MDA_W0_32_DFMT0"]
pub type W = crate::W<u32, super::MDA_W0_32_DFMT0>;
#[doc = "Register MDA_W0_32_DFMT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDA_W0_32_DFMT0 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
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
#[doc = "DID Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIDS_A {
  #[doc = "0: Use MDAm\\[3:0\\] as the domain identifier."]
  DIDS_0,
  #[doc = "1: Use the input DID as the domain identifier."]
  DIDS_1,
  #[doc = "2: Use MDAm\\[3:2\\] concatenated with the low-order 2 bits of the input DID (DID_in\\[1:0\\]) as the domain identifier."]
  DIDS_2,
}
impl From<DIDS_A> for u8 {
  #[inline(always)]
  fn from(variant: DIDS_A) -> Self {
    match variant {
      DIDS_A::DIDS_0 => 0,
      DIDS_A::DIDS_1 => 1,
      DIDS_A::DIDS_2 => 2,
    }
  }
}
#[doc = "Reader of field `DIDS`"]
pub type DIDS_R = crate::R<u8, DIDS_A>;
impl DIDS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DIDS_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DIDS_A::DIDS_0),
      1 => Val(DIDS_A::DIDS_1),
      2 => Val(DIDS_A::DIDS_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DIDS_0`"]
  #[inline(always)]
  pub fn is_dids_0(&self) -> bool {
    *self == DIDS_A::DIDS_0
  }
  #[doc = "Checks if the value of the field is `DIDS_1`"]
  #[inline(always)]
  pub fn is_dids_1(&self) -> bool {
    *self == DIDS_A::DIDS_1
  }
  #[doc = "Checks if the value of the field is `DIDS_2`"]
  #[inline(always)]
  pub fn is_dids_2(&self) -> bool {
    *self == DIDS_A::DIDS_2
  }
}
#[doc = "Write proxy for field `DIDS`"]
pub struct DIDS_W<'a> {
  w: &'a mut W,
}
impl<'a> DIDS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DIDS_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Use MDAm\\[3:0\\] as the domain identifier."]
  #[inline(always)]
  pub fn dids_0(self) -> &'a mut W {
    self.variant(DIDS_A::DIDS_0)
  }
  #[doc = "Use the input DID as the domain identifier."]
  #[inline(always)]
  pub fn dids_1(self) -> &'a mut W {
    self.variant(DIDS_A::DIDS_1)
  }
  #[doc = "Use MDAm\\[3:2\\] concatenated with the low-order 2 bits of the input DID (DID_in\\[1:0\\]) as the domain identifier."]
  #[inline(always)]
  pub fn dids_2(self) -> &'a mut W {
    self.variant(DIDS_A::DIDS_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Process identifier enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
  #[doc = "0: No process identifier is included in the domain hit evaluation."]
  PE_0,
  #[doc = "1: No process identifier is included in the domain hit evaluation."]
  PE_1,
  #[doc = "2: no description available"]
  PE_2,
  #[doc = "3: no description available"]
  PE_3,
}
impl From<PE_A> for u8 {
  #[inline(always)]
  fn from(variant: PE_A) -> Self {
    match variant {
      PE_A::PE_0 => 0,
      PE_A::PE_1 => 1,
      PE_A::PE_2 => 2,
      PE_A::PE_3 => 3,
    }
  }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<u8, PE_A>;
impl PE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PE_A {
    match self.bits {
      0 => PE_A::PE_0,
      1 => PE_A::PE_1,
      2 => PE_A::PE_2,
      3 => PE_A::PE_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PE_0`"]
  #[inline(always)]
  pub fn is_pe_0(&self) -> bool {
    *self == PE_A::PE_0
  }
  #[doc = "Checks if the value of the field is `PE_1`"]
  #[inline(always)]
  pub fn is_pe_1(&self) -> bool {
    *self == PE_A::PE_1
  }
  #[doc = "Checks if the value of the field is `PE_2`"]
  #[inline(always)]
  pub fn is_pe_2(&self) -> bool {
    *self == PE_A::PE_2
  }
  #[doc = "Checks if the value of the field is `PE_3`"]
  #[inline(always)]
  pub fn is_pe_3(&self) -> bool {
    *self == PE_A::PE_3
  }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
  w: &'a mut W,
}
impl<'a> PE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PE_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "No process identifier is included in the domain hit evaluation."]
  #[inline(always)]
  pub fn pe_0(self) -> &'a mut W {
    self.variant(PE_A::PE_0)
  }
  #[doc = "No process identifier is included in the domain hit evaluation."]
  #[inline(always)]
  pub fn pe_1(self) -> &'a mut W {
    self.variant(PE_A::PE_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn pe_2(self) -> &'a mut W {
    self.variant(PE_A::PE_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn pe_3(self) -> &'a mut W {
    self.variant(PE_A::PE_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Reader of field `PIDM`"]
pub type PIDM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIDM`"]
pub struct PIDM_W<'a> {
  w: &'a mut W,
}
impl<'a> PIDM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
    self.w
  }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
  w: &'a mut W,
}
impl<'a> PID_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
    self.w
  }
}
#[doc = "Domain format\n\nValue on reset: 0"]
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
  #[doc = "Bits 4:5 - DID Select"]
  #[inline(always)]
  pub fn dids(&self) -> DIDS_R {
    DIDS_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Process identifier enable"]
  #[inline(always)]
  pub fn pe(&self) -> PE_R {
    PE_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bits 8:13 - Process Identifier Mask"]
  #[inline(always)]
  pub fn pidm(&self) -> PIDM_R {
    PIDM_R::new(((self.bits >> 8) & 0x3f) as u8)
  }
  #[doc = "Bits 16:21 - Process Identifier"]
  #[inline(always)]
  pub fn pid(&self) -> PID_R {
    PID_R::new(((self.bits >> 16) & 0x3f) as u8)
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
  #[doc = "Bits 4:5 - DID Select"]
  #[inline(always)]
  pub fn dids(&mut self) -> DIDS_W {
    DIDS_W { w: self }
  }
  #[doc = "Bits 6:7 - Process identifier enable"]
  #[inline(always)]
  pub fn pe(&mut self) -> PE_W {
    PE_W { w: self }
  }
  #[doc = "Bits 8:13 - Process Identifier Mask"]
  #[inline(always)]
  pub fn pidm(&mut self) -> PIDM_W {
    PIDM_W { w: self }
  }
  #[doc = "Bits 16:21 - Process Identifier"]
  #[inline(always)]
  pub fn pid(&mut self) -> PID_W {
    PID_W { w: self }
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
