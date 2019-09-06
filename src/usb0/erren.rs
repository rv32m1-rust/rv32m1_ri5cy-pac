#[doc = "Reader of register ERREN"]
pub type R = crate::R<u8, super::ERREN>;
#[doc = "Writer for register ERREN"]
pub type W = crate::W<u8, super::ERREN>;
#[doc = "Register ERREN `reset()`'s with value 0"]
impl crate::ResetValue for super::ERREN {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "PIDERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDERREN_A {
  #[doc = "0: Disables the PIDERR interrupt."]
  PIDERREN_0,
  #[doc = "1: Enters the PIDERR interrupt."]
  PIDERREN_1,
}
impl From<PIDERREN_A> for bool {
  #[inline(always)]
  fn from(variant: PIDERREN_A) -> Self {
    match variant {
      PIDERREN_A::PIDERREN_0 => false,
      PIDERREN_A::PIDERREN_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDERREN`"]
pub type PIDERREN_R = crate::R<bool, PIDERREN_A>;
impl PIDERREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDERREN_A {
    match self.bits {
      false => PIDERREN_A::PIDERREN_0,
      true => PIDERREN_A::PIDERREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDERREN_0`"]
  #[inline(always)]
  pub fn is_piderren_0(&self) -> bool {
    *self == PIDERREN_A::PIDERREN_0
  }
  #[doc = "Checks if the value of the field is `PIDERREN_1`"]
  #[inline(always)]
  pub fn is_piderren_1(&self) -> bool {
    *self == PIDERREN_A::PIDERREN_1
  }
}
#[doc = "Write proxy for field `PIDERREN`"]
pub struct PIDERREN_W<'a> {
  w: &'a mut W,
}
impl<'a> PIDERREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PIDERREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the PIDERR interrupt."]
  #[inline(always)]
  pub fn piderren_0(self) -> &'a mut W {
    self.variant(PIDERREN_A::PIDERREN_0)
  }
  #[doc = "Enters the PIDERR interrupt."]
  #[inline(always)]
  pub fn piderren_1(self) -> &'a mut W {
    self.variant(PIDERREN_A::PIDERREN_1)
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
#[doc = "CRC5/EOF Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC5EOFEN_A {
  #[doc = "0: Disables the CRC5/EOF interrupt."]
  CRC5EOFEN_0,
  #[doc = "1: Enables the CRC5/EOF interrupt."]
  CRC5EOFEN_1,
}
impl From<CRC5EOFEN_A> for bool {
  #[inline(always)]
  fn from(variant: CRC5EOFEN_A) -> Self {
    match variant {
      CRC5EOFEN_A::CRC5EOFEN_0 => false,
      CRC5EOFEN_A::CRC5EOFEN_1 => true,
    }
  }
}
#[doc = "Reader of field `CRC5EOFEN`"]
pub type CRC5EOFEN_R = crate::R<bool, CRC5EOFEN_A>;
impl CRC5EOFEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CRC5EOFEN_A {
    match self.bits {
      false => CRC5EOFEN_A::CRC5EOFEN_0,
      true => CRC5EOFEN_A::CRC5EOFEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CRC5EOFEN_0`"]
  #[inline(always)]
  pub fn is_crc5eofen_0(&self) -> bool {
    *self == CRC5EOFEN_A::CRC5EOFEN_0
  }
  #[doc = "Checks if the value of the field is `CRC5EOFEN_1`"]
  #[inline(always)]
  pub fn is_crc5eofen_1(&self) -> bool {
    *self == CRC5EOFEN_A::CRC5EOFEN_1
  }
}
#[doc = "Write proxy for field `CRC5EOFEN`"]
pub struct CRC5EOFEN_W<'a> {
  w: &'a mut W,
}
impl<'a> CRC5EOFEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CRC5EOFEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the CRC5/EOF interrupt."]
  #[inline(always)]
  pub fn crc5eofen_0(self) -> &'a mut W {
    self.variant(CRC5EOFEN_A::CRC5EOFEN_0)
  }
  #[doc = "Enables the CRC5/EOF interrupt."]
  #[inline(always)]
  pub fn crc5eofen_1(self) -> &'a mut W {
    self.variant(CRC5EOFEN_A::CRC5EOFEN_1)
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
#[doc = "CRC16 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC16EN_A {
  #[doc = "0: Disables the CRC16 interrupt."]
  CRC16EN_0,
  #[doc = "1: Enables the CRC16 interrupt."]
  CRC16EN_1,
}
impl From<CRC16EN_A> for bool {
  #[inline(always)]
  fn from(variant: CRC16EN_A) -> Self {
    match variant {
      CRC16EN_A::CRC16EN_0 => false,
      CRC16EN_A::CRC16EN_1 => true,
    }
  }
}
#[doc = "Reader of field `CRC16EN`"]
pub type CRC16EN_R = crate::R<bool, CRC16EN_A>;
impl CRC16EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CRC16EN_A {
    match self.bits {
      false => CRC16EN_A::CRC16EN_0,
      true => CRC16EN_A::CRC16EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CRC16EN_0`"]
  #[inline(always)]
  pub fn is_crc16en_0(&self) -> bool {
    *self == CRC16EN_A::CRC16EN_0
  }
  #[doc = "Checks if the value of the field is `CRC16EN_1`"]
  #[inline(always)]
  pub fn is_crc16en_1(&self) -> bool {
    *self == CRC16EN_A::CRC16EN_1
  }
}
#[doc = "Write proxy for field `CRC16EN`"]
pub struct CRC16EN_W<'a> {
  w: &'a mut W,
}
impl<'a> CRC16EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CRC16EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the CRC16 interrupt."]
  #[inline(always)]
  pub fn crc16en_0(self) -> &'a mut W {
    self.variant(CRC16EN_A::CRC16EN_0)
  }
  #[doc = "Enables the CRC16 interrupt."]
  #[inline(always)]
  pub fn crc16en_1(self) -> &'a mut W {
    self.variant(CRC16EN_A::CRC16EN_1)
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
#[doc = "DFN8 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFN8EN_A {
  #[doc = "0: Disables the DFN8 interrupt."]
  DFN8EN_0,
  #[doc = "1: Enables the DFN8 interrupt."]
  DFN8EN_1,
}
impl From<DFN8EN_A> for bool {
  #[inline(always)]
  fn from(variant: DFN8EN_A) -> Self {
    match variant {
      DFN8EN_A::DFN8EN_0 => false,
      DFN8EN_A::DFN8EN_1 => true,
    }
  }
}
#[doc = "Reader of field `DFN8EN`"]
pub type DFN8EN_R = crate::R<bool, DFN8EN_A>;
impl DFN8EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DFN8EN_A {
    match self.bits {
      false => DFN8EN_A::DFN8EN_0,
      true => DFN8EN_A::DFN8EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DFN8EN_0`"]
  #[inline(always)]
  pub fn is_dfn8en_0(&self) -> bool {
    *self == DFN8EN_A::DFN8EN_0
  }
  #[doc = "Checks if the value of the field is `DFN8EN_1`"]
  #[inline(always)]
  pub fn is_dfn8en_1(&self) -> bool {
    *self == DFN8EN_A::DFN8EN_1
  }
}
#[doc = "Write proxy for field `DFN8EN`"]
pub struct DFN8EN_W<'a> {
  w: &'a mut W,
}
impl<'a> DFN8EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DFN8EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the DFN8 interrupt."]
  #[inline(always)]
  pub fn dfn8en_0(self) -> &'a mut W {
    self.variant(DFN8EN_A::DFN8EN_0)
  }
  #[doc = "Enables the DFN8 interrupt."]
  #[inline(always)]
  pub fn dfn8en_1(self) -> &'a mut W {
    self.variant(DFN8EN_A::DFN8EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
    self.w
  }
}
#[doc = "BTOERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTOERREN_A {
  #[doc = "0: Disables the BTOERR interrupt."]
  BTOERREN_0,
  #[doc = "1: Enables the BTOERR interrupt."]
  BTOERREN_1,
}
impl From<BTOERREN_A> for bool {
  #[inline(always)]
  fn from(variant: BTOERREN_A) -> Self {
    match variant {
      BTOERREN_A::BTOERREN_0 => false,
      BTOERREN_A::BTOERREN_1 => true,
    }
  }
}
#[doc = "Reader of field `BTOERREN`"]
pub type BTOERREN_R = crate::R<bool, BTOERREN_A>;
impl BTOERREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BTOERREN_A {
    match self.bits {
      false => BTOERREN_A::BTOERREN_0,
      true => BTOERREN_A::BTOERREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `BTOERREN_0`"]
  #[inline(always)]
  pub fn is_btoerren_0(&self) -> bool {
    *self == BTOERREN_A::BTOERREN_0
  }
  #[doc = "Checks if the value of the field is `BTOERREN_1`"]
  #[inline(always)]
  pub fn is_btoerren_1(&self) -> bool {
    *self == BTOERREN_A::BTOERREN_1
  }
}
#[doc = "Write proxy for field `BTOERREN`"]
pub struct BTOERREN_W<'a> {
  w: &'a mut W,
}
impl<'a> BTOERREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BTOERREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the BTOERR interrupt."]
  #[inline(always)]
  pub fn btoerren_0(self) -> &'a mut W {
    self.variant(BTOERREN_A::BTOERREN_0)
  }
  #[doc = "Enables the BTOERR interrupt."]
  #[inline(always)]
  pub fn btoerren_1(self) -> &'a mut W {
    self.variant(BTOERREN_A::BTOERREN_1)
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
#[doc = "DMAERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAERREN_A {
  #[doc = "0: Disables the DMAERR interrupt."]
  DMAERREN_0,
  #[doc = "1: Enables the DMAERR interrupt."]
  DMAERREN_1,
}
impl From<DMAERREN_A> for bool {
  #[inline(always)]
  fn from(variant: DMAERREN_A) -> Self {
    match variant {
      DMAERREN_A::DMAERREN_0 => false,
      DMAERREN_A::DMAERREN_1 => true,
    }
  }
}
#[doc = "Reader of field `DMAERREN`"]
pub type DMAERREN_R = crate::R<bool, DMAERREN_A>;
impl DMAERREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DMAERREN_A {
    match self.bits {
      false => DMAERREN_A::DMAERREN_0,
      true => DMAERREN_A::DMAERREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DMAERREN_0`"]
  #[inline(always)]
  pub fn is_dmaerren_0(&self) -> bool {
    *self == DMAERREN_A::DMAERREN_0
  }
  #[doc = "Checks if the value of the field is `DMAERREN_1`"]
  #[inline(always)]
  pub fn is_dmaerren_1(&self) -> bool {
    *self == DMAERREN_A::DMAERREN_1
  }
}
#[doc = "Write proxy for field `DMAERREN`"]
pub struct DMAERREN_W<'a> {
  w: &'a mut W,
}
impl<'a> DMAERREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DMAERREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the DMAERR interrupt."]
  #[inline(always)]
  pub fn dmaerren_0(self) -> &'a mut W {
    self.variant(DMAERREN_A::DMAERREN_0)
  }
  #[doc = "Enables the DMAERR interrupt."]
  #[inline(always)]
  pub fn dmaerren_1(self) -> &'a mut W {
    self.variant(DMAERREN_A::DMAERREN_1)
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
#[doc = "OWNERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWNERREN_A {
  #[doc = "0: Disables the OWNERR interrupt."]
  OWNERREN_0,
  #[doc = "1: Enables the OWNERR interrupt."]
  OWNERREN_1,
}
impl From<OWNERREN_A> for bool {
  #[inline(always)]
  fn from(variant: OWNERREN_A) -> Self {
    match variant {
      OWNERREN_A::OWNERREN_0 => false,
      OWNERREN_A::OWNERREN_1 => true,
    }
  }
}
#[doc = "Reader of field `OWNERREN`"]
pub type OWNERREN_R = crate::R<bool, OWNERREN_A>;
impl OWNERREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OWNERREN_A {
    match self.bits {
      false => OWNERREN_A::OWNERREN_0,
      true => OWNERREN_A::OWNERREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `OWNERREN_0`"]
  #[inline(always)]
  pub fn is_ownerren_0(&self) -> bool {
    *self == OWNERREN_A::OWNERREN_0
  }
  #[doc = "Checks if the value of the field is `OWNERREN_1`"]
  #[inline(always)]
  pub fn is_ownerren_1(&self) -> bool {
    *self == OWNERREN_A::OWNERREN_1
  }
}
#[doc = "Write proxy for field `OWNERREN`"]
pub struct OWNERREN_W<'a> {
  w: &'a mut W,
}
impl<'a> OWNERREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OWNERREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the OWNERR interrupt."]
  #[inline(always)]
  pub fn ownerren_0(self) -> &'a mut W {
    self.variant(OWNERREN_A::OWNERREN_0)
  }
  #[doc = "Enables the OWNERR interrupt."]
  #[inline(always)]
  pub fn ownerren_1(self) -> &'a mut W {
    self.variant(OWNERREN_A::OWNERREN_1)
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
#[doc = "BTSERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTSERREN_A {
  #[doc = "0: Disables the BTSERR interrupt."]
  BTSERREN_0,
  #[doc = "1: Enables the BTSERR interrupt."]
  BTSERREN_1,
}
impl From<BTSERREN_A> for bool {
  #[inline(always)]
  fn from(variant: BTSERREN_A) -> Self {
    match variant {
      BTSERREN_A::BTSERREN_0 => false,
      BTSERREN_A::BTSERREN_1 => true,
    }
  }
}
#[doc = "Reader of field `BTSERREN`"]
pub type BTSERREN_R = crate::R<bool, BTSERREN_A>;
impl BTSERREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BTSERREN_A {
    match self.bits {
      false => BTSERREN_A::BTSERREN_0,
      true => BTSERREN_A::BTSERREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `BTSERREN_0`"]
  #[inline(always)]
  pub fn is_btserren_0(&self) -> bool {
    *self == BTSERREN_A::BTSERREN_0
  }
  #[doc = "Checks if the value of the field is `BTSERREN_1`"]
  #[inline(always)]
  pub fn is_btserren_1(&self) -> bool {
    *self == BTSERREN_A::BTSERREN_1
  }
}
#[doc = "Write proxy for field `BTSERREN`"]
pub struct BTSERREN_W<'a> {
  w: &'a mut W,
}
impl<'a> BTSERREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BTSERREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the BTSERR interrupt."]
  #[inline(always)]
  pub fn btserren_0(self) -> &'a mut W {
    self.variant(BTSERREN_A::BTSERREN_0)
  }
  #[doc = "Enables the BTSERR interrupt."]
  #[inline(always)]
  pub fn btserren_1(self) -> &'a mut W {
    self.variant(BTSERREN_A::BTSERREN_1)
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
  #[doc = "Bit 0 - PIDERR Interrupt Enable"]
  #[inline(always)]
  pub fn piderren(&self) -> PIDERREN_R {
    PIDERREN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
  #[inline(always)]
  pub fn crc5eofen(&self) -> CRC5EOFEN_R {
    CRC5EOFEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - CRC16 Interrupt Enable"]
  #[inline(always)]
  pub fn crc16en(&self) -> CRC16EN_R {
    CRC16EN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - DFN8 Interrupt Enable"]
  #[inline(always)]
  pub fn dfn8en(&self) -> DFN8EN_R {
    DFN8EN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - BTOERR Interrupt Enable"]
  #[inline(always)]
  pub fn btoerren(&self) -> BTOERREN_R {
    BTOERREN_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - DMAERR Interrupt Enable"]
  #[inline(always)]
  pub fn dmaerren(&self) -> DMAERREN_R {
    DMAERREN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - OWNERR Interrupt Enable"]
  #[inline(always)]
  pub fn ownerren(&self) -> OWNERREN_R {
    OWNERREN_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - BTSERR Interrupt Enable"]
  #[inline(always)]
  pub fn btserren(&self) -> BTSERREN_R {
    BTSERREN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - PIDERR Interrupt Enable"]
  #[inline(always)]
  pub fn piderren(&mut self) -> PIDERREN_W {
    PIDERREN_W { w: self }
  }
  #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
  #[inline(always)]
  pub fn crc5eofen(&mut self) -> CRC5EOFEN_W {
    CRC5EOFEN_W { w: self }
  }
  #[doc = "Bit 2 - CRC16 Interrupt Enable"]
  #[inline(always)]
  pub fn crc16en(&mut self) -> CRC16EN_W {
    CRC16EN_W { w: self }
  }
  #[doc = "Bit 3 - DFN8 Interrupt Enable"]
  #[inline(always)]
  pub fn dfn8en(&mut self) -> DFN8EN_W {
    DFN8EN_W { w: self }
  }
  #[doc = "Bit 4 - BTOERR Interrupt Enable"]
  #[inline(always)]
  pub fn btoerren(&mut self) -> BTOERREN_W {
    BTOERREN_W { w: self }
  }
  #[doc = "Bit 5 - DMAERR Interrupt Enable"]
  #[inline(always)]
  pub fn dmaerren(&mut self) -> DMAERREN_W {
    DMAERREN_W { w: self }
  }
  #[doc = "Bit 6 - OWNERR Interrupt Enable"]
  #[inline(always)]
  pub fn ownerren(&mut self) -> OWNERREN_W {
    OWNERREN_W { w: self }
  }
  #[doc = "Bit 7 - BTSERR Interrupt Enable"]
  #[inline(always)]
  pub fn btserren(&mut self) -> BTSERREN_W {
    BTSERREN_W { w: self }
  }
}
