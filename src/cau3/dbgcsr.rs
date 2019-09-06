#[doc = "Reader of register DBGCSR"]
pub type R = crate::R<u32, super::DBGCSR>;
#[doc = "Writer for register DBGCSR"]
pub type W = crate::W<u32, super::DBGCSR>;
#[doc = "Register DBGCSR `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::DBGCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x4000_0000
  }
}
#[doc = "Debug Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDBG_A {
  #[doc = "0: debug is enabled"]
  DDBG_0,
  #[doc = "1: debug is disabled"]
  DDBG_1,
}
impl From<DDBG_A> for bool {
  #[inline(always)]
  fn from(variant: DDBG_A) -> Self {
    match variant {
      DDBG_A::DDBG_0 => false,
      DDBG_A::DDBG_1 => true,
    }
  }
}
#[doc = "Reader of field `DDBG`"]
pub type DDBG_R = crate::R<bool, DDBG_A>;
impl DDBG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DDBG_A {
    match self.bits {
      false => DDBG_A::DDBG_0,
      true => DDBG_A::DDBG_1,
    }
  }
  #[doc = "Checks if the value of the field is `DDBG_0`"]
  #[inline(always)]
  pub fn is_ddbg_0(&self) -> bool {
    *self == DDBG_A::DDBG_0
  }
  #[doc = "Checks if the value of the field is `DDBG_1`"]
  #[inline(always)]
  pub fn is_ddbg_1(&self) -> bool {
    *self == DDBG_A::DDBG_1
  }
}
#[doc = "Write proxy for field `DDBG`"]
pub struct DDBG_W<'a> {
  w: &'a mut W,
}
impl<'a> DDBG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DDBG_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "debug is enabled"]
  #[inline(always)]
  pub fn ddbg_0(self) -> &'a mut W {
    self.variant(DDBG_A::DDBG_0)
  }
  #[doc = "debug is disabled"]
  #[inline(always)]
  pub fn ddbg_1(self) -> &'a mut W {
    self.variant(DDBG_A::DDBG_1)
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
#[doc = "Disable Debug Memory Commands\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDBGMC_A {
  #[doc = "0: IPS access to IMEM and DMEM are enabled"]
  DDBGMC_0,
  #[doc = "1: IPS access to IMEM and DMEM are disabled"]
  DDBGMC_1,
}
impl From<DDBGMC_A> for bool {
  #[inline(always)]
  fn from(variant: DDBGMC_A) -> Self {
    match variant {
      DDBGMC_A::DDBGMC_0 => false,
      DDBGMC_A::DDBGMC_1 => true,
    }
  }
}
#[doc = "Reader of field `DDBGMC`"]
pub type DDBGMC_R = crate::R<bool, DDBGMC_A>;
impl DDBGMC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DDBGMC_A {
    match self.bits {
      false => DDBGMC_A::DDBGMC_0,
      true => DDBGMC_A::DDBGMC_1,
    }
  }
  #[doc = "Checks if the value of the field is `DDBGMC_0`"]
  #[inline(always)]
  pub fn is_ddbgmc_0(&self) -> bool {
    *self == DDBGMC_A::DDBGMC_0
  }
  #[doc = "Checks if the value of the field is `DDBGMC_1`"]
  #[inline(always)]
  pub fn is_ddbgmc_1(&self) -> bool {
    *self == DDBGMC_A::DDBGMC_1
  }
}
#[doc = "Write proxy for field `DDBGMC`"]
pub struct DDBGMC_W<'a> {
  w: &'a mut W,
}
impl<'a> DDBGMC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DDBGMC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "IPS access to IMEM and DMEM are enabled"]
  #[inline(always)]
  pub fn ddbgmc_0(self) -> &'a mut W {
    self.variant(DDBGMC_A::DDBGMC_0)
  }
  #[doc = "IPS access to IMEM and DMEM are disabled"]
  #[inline(always)]
  pub fn ddbgmc_1(self) -> &'a mut W {
    self.variant(DDBGMC_A::DDBGMC_1)
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
#[doc = "PC Breakpoint Register Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBREN_A {
  #[doc = "0: PC breakpoint register (DBGPBR) is disabled"]
  PBREN_0,
  #[doc = "1: PC breakpoint register (DBGPBR) is enabled"]
  PBREN_1,
}
impl From<PBREN_A> for bool {
  #[inline(always)]
  fn from(variant: PBREN_A) -> Self {
    match variant {
      PBREN_A::PBREN_0 => false,
      PBREN_A::PBREN_1 => true,
    }
  }
}
#[doc = "Reader of field `PBREN`"]
pub type PBREN_R = crate::R<bool, PBREN_A>;
impl PBREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PBREN_A {
    match self.bits {
      false => PBREN_A::PBREN_0,
      true => PBREN_A::PBREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `PBREN_0`"]
  #[inline(always)]
  pub fn is_pbren_0(&self) -> bool {
    *self == PBREN_A::PBREN_0
  }
  #[doc = "Checks if the value of the field is `PBREN_1`"]
  #[inline(always)]
  pub fn is_pbren_1(&self) -> bool {
    *self == PBREN_A::PBREN_1
  }
}
#[doc = "Write proxy for field `PBREN`"]
pub struct PBREN_W<'a> {
  w: &'a mut W,
}
impl<'a> PBREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PBREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "PC breakpoint register (DBGPBR) is disabled"]
  #[inline(always)]
  pub fn pbren_0(self) -> &'a mut W {
    self.variant(PBREN_A::PBREN_0)
  }
  #[doc = "PC breakpoint register (DBGPBR) is enabled"]
  #[inline(always)]
  pub fn pbren_1(self) -> &'a mut W {
    self.variant(PBREN_A::PBREN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "Single Instruction Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIM_A {
  #[doc = "0: Single instruction mode is disabled"]
  SIM_0,
  #[doc = "1: Single instruction mode is enabled"]
  SIM_1,
}
impl From<SIM_A> for bool {
  #[inline(always)]
  fn from(variant: SIM_A) -> Self {
    match variant {
      SIM_A::SIM_0 => false,
      SIM_A::SIM_1 => true,
    }
  }
}
#[doc = "Reader of field `SIM`"]
pub type SIM_R = crate::R<bool, SIM_A>;
impl SIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIM_A {
    match self.bits {
      false => SIM_A::SIM_0,
      true => SIM_A::SIM_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIM_0`"]
  #[inline(always)]
  pub fn is_sim_0(&self) -> bool {
    *self == SIM_A::SIM_0
  }
  #[doc = "Checks if the value of the field is `SIM_1`"]
  #[inline(always)]
  pub fn is_sim_1(&self) -> bool {
    *self == SIM_A::SIM_1
  }
}
#[doc = "Write proxy for field `SIM`"]
pub struct SIM_W<'a> {
  w: &'a mut W,
}
impl<'a> SIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Single instruction mode is disabled"]
  #[inline(always)]
  pub fn sim_0(self) -> &'a mut W {
    self.variant(SIM_A::SIM_0)
  }
  #[doc = "Single instruction mode is enabled"]
  #[inline(always)]
  pub fn sim_1(self) -> &'a mut W {
    self.variant(SIM_A::SIM_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "Force Debug Halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRCH_A {
  #[doc = "0: Halt state not forced"]
  FRCH_0,
  #[doc = "1: Force halt state"]
  FRCH_1,
}
impl From<FRCH_A> for bool {
  #[inline(always)]
  fn from(variant: FRCH_A) -> Self {
    match variant {
      FRCH_A::FRCH_0 => false,
      FRCH_A::FRCH_1 => true,
    }
  }
}
#[doc = "Reader of field `FRCH`"]
pub type FRCH_R = crate::R<bool, FRCH_A>;
impl FRCH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FRCH_A {
    match self.bits {
      false => FRCH_A::FRCH_0,
      true => FRCH_A::FRCH_1,
    }
  }
  #[doc = "Checks if the value of the field is `FRCH_0`"]
  #[inline(always)]
  pub fn is_frch_0(&self) -> bool {
    *self == FRCH_A::FRCH_0
  }
  #[doc = "Checks if the value of the field is `FRCH_1`"]
  #[inline(always)]
  pub fn is_frch_1(&self) -> bool {
    *self == FRCH_A::FRCH_1
  }
}
#[doc = "Write proxy for field `FRCH`"]
pub struct FRCH_W<'a> {
  w: &'a mut W,
}
impl<'a> FRCH_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FRCH_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Halt state not forced"]
  #[inline(always)]
  pub fn frch_0(self) -> &'a mut W {
    self.variant(FRCH_A::FRCH_0)
  }
  #[doc = "Force halt state"]
  #[inline(always)]
  pub fn frch_1(self) -> &'a mut W {
    self.variant(FRCH_A::FRCH_1)
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
#[doc = "Debug Go\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGGO_AW {
  #[doc = "0: No action"]
  DBGGO_0,
  #[doc = "1: Resume program execution"]
  DBGGO_1,
}
impl From<DBGGO_AW> for bool {
  #[inline(always)]
  fn from(variant: DBGGO_AW) -> Self {
    match variant {
      DBGGO_AW::DBGGO_0 => false,
      DBGGO_AW::DBGGO_1 => true,
    }
  }
}
#[doc = "Write proxy for field `DBGGO`"]
pub struct DBGGO_W<'a> {
  w: &'a mut W,
}
impl<'a> DBGGO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DBGGO_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No action"]
  #[inline(always)]
  pub fn dbggo_0(self) -> &'a mut W {
    self.variant(DBGGO_AW::DBGGO_0)
  }
  #[doc = "Resume program execution"]
  #[inline(always)]
  pub fn dbggo_1(self) -> &'a mut W {
    self.variant(DBGGO_AW::DBGGO_1)
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
#[doc = "CryptoCore is Halted due to Hardware Breakpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCBHF_A {
  #[doc = "0: CryptoCore is not halted due to a hardware breakpoint"]
  PCBHF_0,
  #[doc = "1: CryptoCore is halted due to a hardware breakpoint"]
  PCBHF_1,
}
impl From<PCBHF_A> for bool {
  #[inline(always)]
  fn from(variant: PCBHF_A) -> Self {
    match variant {
      PCBHF_A::PCBHF_0 => false,
      PCBHF_A::PCBHF_1 => true,
    }
  }
}
#[doc = "Reader of field `PCBHF`"]
pub type PCBHF_R = crate::R<bool, PCBHF_A>;
impl PCBHF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PCBHF_A {
    match self.bits {
      false => PCBHF_A::PCBHF_0,
      true => PCBHF_A::PCBHF_1,
    }
  }
  #[doc = "Checks if the value of the field is `PCBHF_0`"]
  #[inline(always)]
  pub fn is_pcbhf_0(&self) -> bool {
    *self == PCBHF_A::PCBHF_0
  }
  #[doc = "Checks if the value of the field is `PCBHF_1`"]
  #[inline(always)]
  pub fn is_pcbhf_1(&self) -> bool {
    *self == PCBHF_A::PCBHF_1
  }
}
#[doc = "CryptoCore is Halted due to Single Instruction Step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIMHF_A {
  #[doc = "0: CryptoCore is not in a single step halt"]
  SIMHF_0,
  #[doc = "1: CryptoCore is in a single step halt"]
  SIMHF_1,
}
impl From<SIMHF_A> for bool {
  #[inline(always)]
  fn from(variant: SIMHF_A) -> Self {
    match variant {
      SIMHF_A::SIMHF_0 => false,
      SIMHF_A::SIMHF_1 => true,
    }
  }
}
#[doc = "Reader of field `SIMHF`"]
pub type SIMHF_R = crate::R<bool, SIMHF_A>;
impl SIMHF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIMHF_A {
    match self.bits {
      false => SIMHF_A::SIMHF_0,
      true => SIMHF_A::SIMHF_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIMHF_0`"]
  #[inline(always)]
  pub fn is_simhf_0(&self) -> bool {
    *self == SIMHF_A::SIMHF_0
  }
  #[doc = "Checks if the value of the field is `SIMHF_1`"]
  #[inline(always)]
  pub fn is_simhf_1(&self) -> bool {
    *self == SIMHF_A::SIMHF_1
  }
}
#[doc = "CryptoCore is Halted due to HALT Instruction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HLTIF_A {
  #[doc = "0: CryptoCore is not in software breakpoint"]
  HLTIF_0,
  #[doc = "1: CryptoCore is in software breakpoint"]
  HLTIF_1,
}
impl From<HLTIF_A> for bool {
  #[inline(always)]
  fn from(variant: HLTIF_A) -> Self {
    match variant {
      HLTIF_A::HLTIF_0 => false,
      HLTIF_A::HLTIF_1 => true,
    }
  }
}
#[doc = "Reader of field `HLTIF`"]
pub type HLTIF_R = crate::R<bool, HLTIF_A>;
impl HLTIF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HLTIF_A {
    match self.bits {
      false => HLTIF_A::HLTIF_0,
      true => HLTIF_A::HLTIF_1,
    }
  }
  #[doc = "Checks if the value of the field is `HLTIF_0`"]
  #[inline(always)]
  pub fn is_hltif_0(&self) -> bool {
    *self == HLTIF_A::HLTIF_0
  }
  #[doc = "Checks if the value of the field is `HLTIF_1`"]
  #[inline(always)]
  pub fn is_hltif_1(&self) -> bool {
    *self == HLTIF_A::HLTIF_1
  }
}
#[doc = "CryptoCore is Stopped Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTPF_A {
  #[doc = "0: CryptoCore is not stopped"]
  CSTPF_0,
  #[doc = "1: CryptoCore is stopped"]
  CSTPF_1,
}
impl From<CSTPF_A> for bool {
  #[inline(always)]
  fn from(variant: CSTPF_A) -> Self {
    match variant {
      CSTPF_A::CSTPF_0 => false,
      CSTPF_A::CSTPF_1 => true,
    }
  }
}
#[doc = "Reader of field `CSTPF`"]
pub type CSTPF_R = crate::R<bool, CSTPF_A>;
impl CSTPF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CSTPF_A {
    match self.bits {
      false => CSTPF_A::CSTPF_0,
      true => CSTPF_A::CSTPF_1,
    }
  }
  #[doc = "Checks if the value of the field is `CSTPF_0`"]
  #[inline(always)]
  pub fn is_cstpf_0(&self) -> bool {
    *self == CSTPF_A::CSTPF_0
  }
  #[doc = "Checks if the value of the field is `CSTPF_1`"]
  #[inline(always)]
  pub fn is_cstpf_1(&self) -> bool {
    *self == CSTPF_A::CSTPF_1
  }
}
#[doc = "CryptoCore is Halted Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLTF_A {
  #[doc = "0: CryptoCore is not halted"]
  CHLTF_0,
  #[doc = "1: CryptoCore is halted"]
  CHLTF_1,
}
impl From<CHLTF_A> for bool {
  #[inline(always)]
  fn from(variant: CHLTF_A) -> Self {
    match variant {
      CHLTF_A::CHLTF_0 => false,
      CHLTF_A::CHLTF_1 => true,
    }
  }
}
#[doc = "Reader of field `CHLTF`"]
pub type CHLTF_R = crate::R<bool, CHLTF_A>;
impl CHLTF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CHLTF_A {
    match self.bits {
      false => CHLTF_A::CHLTF_0,
      true => CHLTF_A::CHLTF_1,
    }
  }
  #[doc = "Checks if the value of the field is `CHLTF_0`"]
  #[inline(always)]
  pub fn is_chltf_0(&self) -> bool {
    *self == CHLTF_A::CHLTF_0
  }
  #[doc = "Checks if the value of the field is `CHLTF_1`"]
  #[inline(always)]
  pub fn is_chltf_1(&self) -> bool {
    *self == CHLTF_A::CHLTF_1
  }
}
impl R {
  #[doc = "Bit 0 - Debug Disable"]
  #[inline(always)]
  pub fn ddbg(&self) -> DDBG_R {
    DDBG_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Disable Debug Memory Commands"]
  #[inline(always)]
  pub fn ddbgmc(&self) -> DDBGMC_R {
    DDBGMC_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 4 - PC Breakpoint Register Enable"]
  #[inline(always)]
  pub fn pbren(&self) -> PBREN_R {
    PBREN_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Single Instruction Mode"]
  #[inline(always)]
  pub fn sim(&self) -> SIM_R {
    SIM_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Force Debug Halt"]
  #[inline(always)]
  pub fn frch(&self) -> FRCH_R {
    FRCH_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 16 - CryptoCore is Halted due to Hardware Breakpoint"]
  #[inline(always)]
  pub fn pcbhf(&self) -> PCBHF_R {
    PCBHF_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - CryptoCore is Halted due to Single Instruction Step"]
  #[inline(always)]
  pub fn simhf(&self) -> SIMHF_R {
    SIMHF_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - CryptoCore is Halted due to HALT Instruction"]
  #[inline(always)]
  pub fn hltif(&self) -> HLTIF_R {
    HLTIF_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 30 - CryptoCore is Stopped Status Flag"]
  #[inline(always)]
  pub fn cstpf(&self) -> CSTPF_R {
    CSTPF_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - CryptoCore is Halted Status Flag"]
  #[inline(always)]
  pub fn chltf(&self) -> CHLTF_R {
    CHLTF_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Debug Disable"]
  #[inline(always)]
  pub fn ddbg(&mut self) -> DDBG_W {
    DDBG_W { w: self }
  }
  #[doc = "Bit 1 - Disable Debug Memory Commands"]
  #[inline(always)]
  pub fn ddbgmc(&mut self) -> DDBGMC_W {
    DDBGMC_W { w: self }
  }
  #[doc = "Bit 4 - PC Breakpoint Register Enable"]
  #[inline(always)]
  pub fn pbren(&mut self) -> PBREN_W {
    PBREN_W { w: self }
  }
  #[doc = "Bit 5 - Single Instruction Mode"]
  #[inline(always)]
  pub fn sim(&mut self) -> SIM_W {
    SIM_W { w: self }
  }
  #[doc = "Bit 8 - Force Debug Halt"]
  #[inline(always)]
  pub fn frch(&mut self) -> FRCH_W {
    FRCH_W { w: self }
  }
  #[doc = "Bit 12 - Debug Go"]
  #[inline(always)]
  pub fn dbggo(&mut self) -> DBGGO_W {
    DBGGO_W { w: self }
  }
}
