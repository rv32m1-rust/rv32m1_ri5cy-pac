#[doc = "Reader of register CORELPCNFG"]
pub type R = crate::R<u32, super::CORELPCNFG>;
#[doc = "Writer for register CORELPCNFG"]
pub type W = crate::W<u32, super::CORELPCNFG>;
#[doc = "Register CORELPCNFG `reset()`'s with value 0x0007_4000"]
impl crate::ResetValue for super::CORELPCNFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0007_4000
  }
}
#[doc = "LPSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSEL_A {
  #[doc = "0: Core LDO enters low power state in VLP/Stop modes."]
  LPSEL_0,
  #[doc = "1: Core LDO remains in high power state in VLP/Stop modes. If LPSEL = 1 in a low power mode then BGEN must also be set to 1."]
  LPSEL_1,
}
impl From<LPSEL_A> for bool {
  #[inline(always)]
  fn from(variant: LPSEL_A) -> Self {
    match variant {
      LPSEL_A::LPSEL_0 => false,
      LPSEL_A::LPSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `LPSEL`"]
pub type LPSEL_R = crate::R<bool, LPSEL_A>;
impl LPSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPSEL_A {
    match self.bits {
      false => LPSEL_A::LPSEL_0,
      true => LPSEL_A::LPSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPSEL_0`"]
  #[inline(always)]
  pub fn is_lpsel_0(&self) -> bool {
    *self == LPSEL_A::LPSEL_0
  }
  #[doc = "Checks if the value of the field is `LPSEL_1`"]
  #[inline(always)]
  pub fn is_lpsel_1(&self) -> bool {
    *self == LPSEL_A::LPSEL_1
  }
}
#[doc = "Write proxy for field `LPSEL`"]
pub struct LPSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> LPSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPSEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Core LDO enters low power state in VLP/Stop modes."]
  #[inline(always)]
  pub fn lpsel_0(self) -> &'a mut W {
    self.variant(LPSEL_A::LPSEL_0)
  }
  #[doc = "Core LDO remains in high power state in VLP/Stop modes. If LPSEL = 1 in a low power mode then BGEN must also be set to 1."]
  #[inline(always)]
  pub fn lpsel_1(self) -> &'a mut W {
    self.variant(LPSEL_A::LPSEL_1)
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
#[doc = "Bandgap Enable In Low Power Mode Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGEN_A {
  #[doc = "0: Bandgap is disabled in STOP/VLP/LLS and VLLS modes."]
  BGEN_0,
  #[doc = "1: Bandgap remains enabled in STOP/VLP/LLS and VLLS modes."]
  BGEN_1,
}
impl From<BGEN_A> for bool {
  #[inline(always)]
  fn from(variant: BGEN_A) -> Self {
    match variant {
      BGEN_A::BGEN_0 => false,
      BGEN_A::BGEN_1 => true,
    }
  }
}
#[doc = "Reader of field `BGEN`"]
pub type BGEN_R = crate::R<bool, BGEN_A>;
impl BGEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BGEN_A {
    match self.bits {
      false => BGEN_A::BGEN_0,
      true => BGEN_A::BGEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `BGEN_0`"]
  #[inline(always)]
  pub fn is_bgen_0(&self) -> bool {
    *self == BGEN_A::BGEN_0
  }
  #[doc = "Checks if the value of the field is `BGEN_1`"]
  #[inline(always)]
  pub fn is_bgen_1(&self) -> bool {
    *self == BGEN_A::BGEN_1
  }
}
#[doc = "Write proxy for field `BGEN`"]
pub struct BGEN_W<'a> {
  w: &'a mut W,
}
impl<'a> BGEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BGEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Bandgap is disabled in STOP/VLP/LLS and VLLS modes."]
  #[inline(always)]
  pub fn bgen_0(self) -> &'a mut W {
    self.variant(BGEN_A::BGEN_0)
  }
  #[doc = "Bandgap remains enabled in STOP/VLP/LLS and VLLS modes."]
  #[inline(always)]
  pub fn bgen_1(self) -> &'a mut W {
    self.variant(BGEN_A::BGEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Bandgap Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBEN_A {
  #[doc = "0: Bandgap buffer not enabled"]
  BGBEN_0,
  #[doc = "1: Bandgap buffer enabled BGEN must be set when this bit is also set."]
  BGBEN_1,
}
impl From<BGBEN_A> for bool {
  #[inline(always)]
  fn from(variant: BGBEN_A) -> Self {
    match variant {
      BGBEN_A::BGBEN_0 => false,
      BGBEN_A::BGBEN_1 => true,
    }
  }
}
#[doc = "Reader of field `BGBEN`"]
pub type BGBEN_R = crate::R<bool, BGBEN_A>;
impl BGBEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BGBEN_A {
    match self.bits {
      false => BGBEN_A::BGBEN_0,
      true => BGBEN_A::BGBEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `BGBEN_0`"]
  #[inline(always)]
  pub fn is_bgben_0(&self) -> bool {
    *self == BGBEN_A::BGBEN_0
  }
  #[doc = "Checks if the value of the field is `BGBEN_1`"]
  #[inline(always)]
  pub fn is_bgben_1(&self) -> bool {
    *self == BGBEN_A::BGBEN_1
  }
}
#[doc = "Write proxy for field `BGBEN`"]
pub struct BGBEN_W<'a> {
  w: &'a mut W,
}
impl<'a> BGBEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BGBEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Bandgap buffer not enabled"]
  #[inline(always)]
  pub fn bgben_0(self) -> &'a mut W {
    self.variant(BGBEN_A::BGBEN_0)
  }
  #[doc = "Bandgap buffer enabled BGEN must be set when this bit is also set."]
  #[inline(always)]
  pub fn bgben_1(self) -> &'a mut W {
    self.variant(BGBEN_A::BGBEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "Bandgap Buffer Drive Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBDS_A {
  #[doc = "0: Low Drive"]
  BGBDS_0,
  #[doc = "1: High Drive"]
  BGBDS_1,
}
impl From<BGBDS_A> for bool {
  #[inline(always)]
  fn from(variant: BGBDS_A) -> Self {
    match variant {
      BGBDS_A::BGBDS_0 => false,
      BGBDS_A::BGBDS_1 => true,
    }
  }
}
#[doc = "Reader of field `BGBDS`"]
pub type BGBDS_R = crate::R<bool, BGBDS_A>;
impl BGBDS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BGBDS_A {
    match self.bits {
      false => BGBDS_A::BGBDS_0,
      true => BGBDS_A::BGBDS_1,
    }
  }
  #[doc = "Checks if the value of the field is `BGBDS_0`"]
  #[inline(always)]
  pub fn is_bgbds_0(&self) -> bool {
    *self == BGBDS_A::BGBDS_0
  }
  #[doc = "Checks if the value of the field is `BGBDS_1`"]
  #[inline(always)]
  pub fn is_bgbds_1(&self) -> bool {
    *self == BGBDS_A::BGBDS_1
  }
}
#[doc = "Write proxy for field `BGBDS`"]
pub struct BGBDS_W<'a> {
  w: &'a mut W,
}
impl<'a> BGBDS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BGBDS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Low Drive"]
  #[inline(always)]
  pub fn bgbds_0(self) -> &'a mut W {
    self.variant(BGBDS_A::BGBDS_0)
  }
  #[doc = "High Drive"]
  #[inline(always)]
  pub fn bgbds_1(self) -> &'a mut W {
    self.variant(BGBDS_A::BGBDS_1)
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
#[doc = "LPO Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOEN_A {
  #[doc = "0: LPO is disabled in VLLS modes."]
  LPOEN_0,
  #[doc = "1: LPO remains enabled in VLLS modes."]
  LPOEN_1,
}
impl From<LPOEN_A> for bool {
  #[inline(always)]
  fn from(variant: LPOEN_A) -> Self {
    match variant {
      LPOEN_A::LPOEN_0 => false,
      LPOEN_A::LPOEN_1 => true,
    }
  }
}
#[doc = "Reader of field `LPOEN`"]
pub type LPOEN_R = crate::R<bool, LPOEN_A>;
impl LPOEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPOEN_A {
    match self.bits {
      false => LPOEN_A::LPOEN_0,
      true => LPOEN_A::LPOEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPOEN_0`"]
  #[inline(always)]
  pub fn is_lpoen_0(&self) -> bool {
    *self == LPOEN_A::LPOEN_0
  }
  #[doc = "Checks if the value of the field is `LPOEN_1`"]
  #[inline(always)]
  pub fn is_lpoen_1(&self) -> bool {
    *self == LPOEN_A::LPOEN_1
  }
}
#[doc = "Write proxy for field `LPOEN`"]
pub struct LPOEN_W<'a> {
  w: &'a mut W,
}
impl<'a> LPOEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPOEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPO is disabled in VLLS modes."]
  #[inline(always)]
  pub fn lpoen_0(self) -> &'a mut W {
    self.variant(LPOEN_A::LPOEN_0)
  }
  #[doc = "LPO remains enabled in VLLS modes."]
  #[inline(always)]
  pub fn lpoen_1(self) -> &'a mut W {
    self.variant(LPOEN_A::LPOEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
#[doc = "POR Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POREN_A {
  #[doc = "0: POR brownout is disabled in VLLS0/1 mode."]
  POREN_0,
  #[doc = "1: POR brownout remains enabled in VLLS0/1 mode."]
  POREN_1,
}
impl From<POREN_A> for bool {
  #[inline(always)]
  fn from(variant: POREN_A) -> Self {
    match variant {
      POREN_A::POREN_0 => false,
      POREN_A::POREN_1 => true,
    }
  }
}
#[doc = "Reader of field `POREN`"]
pub type POREN_R = crate::R<bool, POREN_A>;
impl POREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POREN_A {
    match self.bits {
      false => POREN_A::POREN_0,
      true => POREN_A::POREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `POREN_0`"]
  #[inline(always)]
  pub fn is_poren_0(&self) -> bool {
    *self == POREN_A::POREN_0
  }
  #[doc = "Checks if the value of the field is `POREN_1`"]
  #[inline(always)]
  pub fn is_poren_1(&self) -> bool {
    *self == POREN_A::POREN_1
  }
}
#[doc = "Write proxy for field `POREN`"]
pub struct POREN_W<'a> {
  w: &'a mut W,
}
impl<'a> POREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "POR brownout is disabled in VLLS0/1 mode."]
  #[inline(always)]
  pub fn poren_0(self) -> &'a mut W {
    self.variant(POREN_A::POREN_0)
  }
  #[doc = "POR brownout remains enabled in VLLS0/1 mode."]
  #[inline(always)]
  pub fn poren_1(self) -> &'a mut W {
    self.variant(POREN_A::POREN_1)
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
#[doc = "LVD Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDEN_A {
  #[doc = "0: LVD/HVD is disabled in low power modes."]
  LVDEN_0,
  #[doc = "1: LVD/HVD remains enabled in low power modes. BGEN must be set when this bit is also set."]
  LVDEN_1,
}
impl From<LVDEN_A> for bool {
  #[inline(always)]
  fn from(variant: LVDEN_A) -> Self {
    match variant {
      LVDEN_A::LVDEN_0 => false,
      LVDEN_A::LVDEN_1 => true,
    }
  }
}
#[doc = "Reader of field `LVDEN`"]
pub type LVDEN_R = crate::R<bool, LVDEN_A>;
impl LVDEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LVDEN_A {
    match self.bits {
      false => LVDEN_A::LVDEN_0,
      true => LVDEN_A::LVDEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LVDEN_0`"]
  #[inline(always)]
  pub fn is_lvden_0(&self) -> bool {
    *self == LVDEN_A::LVDEN_0
  }
  #[doc = "Checks if the value of the field is `LVDEN_1`"]
  #[inline(always)]
  pub fn is_lvden_1(&self) -> bool {
    *self == LVDEN_A::LVDEN_1
  }
}
#[doc = "Write proxy for field `LVDEN`"]
pub struct LVDEN_W<'a> {
  w: &'a mut W,
}
impl<'a> LVDEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LVDEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LVD/HVD is disabled in low power modes."]
  #[inline(always)]
  pub fn lvden_0(self) -> &'a mut W {
    self.variant(LVDEN_A::LVDEN_0)
  }
  #[doc = "LVD/HVD remains enabled in low power modes. BGEN must be set when this bit is also set."]
  #[inline(always)]
  pub fn lvden_1(self) -> &'a mut W {
    self.variant(LVDEN_A::LVDEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
#[doc = "LPHIDRIVE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPHIDRIVE_A {
  #[doc = "0: High Drive disabled."]
  LPHIDRIVE_0,
  #[doc = "1: High Drive enabled."]
  LPHIDRIVE_1,
}
impl From<LPHIDRIVE_A> for bool {
  #[inline(always)]
  fn from(variant: LPHIDRIVE_A) -> Self {
    match variant {
      LPHIDRIVE_A::LPHIDRIVE_0 => false,
      LPHIDRIVE_A::LPHIDRIVE_1 => true,
    }
  }
}
#[doc = "Reader of field `LPHIDRIVE`"]
pub type LPHIDRIVE_R = crate::R<bool, LPHIDRIVE_A>;
impl LPHIDRIVE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPHIDRIVE_A {
    match self.bits {
      false => LPHIDRIVE_A::LPHIDRIVE_0,
      true => LPHIDRIVE_A::LPHIDRIVE_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPHIDRIVE_0`"]
  #[inline(always)]
  pub fn is_lphidrive_0(&self) -> bool {
    *self == LPHIDRIVE_A::LPHIDRIVE_0
  }
  #[doc = "Checks if the value of the field is `LPHIDRIVE_1`"]
  #[inline(always)]
  pub fn is_lphidrive_1(&self) -> bool {
    *self == LPHIDRIVE_A::LPHIDRIVE_1
  }
}
#[doc = "Write proxy for field `LPHIDRIVE`"]
pub struct LPHIDRIVE_W<'a> {
  w: &'a mut W,
}
impl<'a> LPHIDRIVE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPHIDRIVE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "High Drive disabled."]
  #[inline(always)]
  pub fn lphidrive_0(self) -> &'a mut W {
    self.variant(LPHIDRIVE_A::LPHIDRIVE_0)
  }
  #[doc = "High Drive enabled."]
  #[inline(always)]
  pub fn lphidrive_1(self) -> &'a mut W {
    self.variant(LPHIDRIVE_A::LPHIDRIVE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
    self.w
  }
}
#[doc = "All Reference Enable. This bit only has an affect in VLLS0/1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLREFEN_A {
  #[doc = "0: All references are disabled in VLLS."]
  ALLREFEN_0,
  #[doc = "1: All references are enabled in VLLS0/1."]
  ALLREFEN_1,
}
impl From<ALLREFEN_A> for bool {
  #[inline(always)]
  fn from(variant: ALLREFEN_A) -> Self {
    match variant {
      ALLREFEN_A::ALLREFEN_0 => false,
      ALLREFEN_A::ALLREFEN_1 => true,
    }
  }
}
#[doc = "Reader of field `ALLREFEN`"]
pub type ALLREFEN_R = crate::R<bool, ALLREFEN_A>;
impl ALLREFEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ALLREFEN_A {
    match self.bits {
      false => ALLREFEN_A::ALLREFEN_0,
      true => ALLREFEN_A::ALLREFEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `ALLREFEN_0`"]
  #[inline(always)]
  pub fn is_allrefen_0(&self) -> bool {
    *self == ALLREFEN_A::ALLREFEN_0
  }
  #[doc = "Checks if the value of the field is `ALLREFEN_1`"]
  #[inline(always)]
  pub fn is_allrefen_1(&self) -> bool {
    *self == ALLREFEN_A::ALLREFEN_1
  }
}
#[doc = "Write proxy for field `ALLREFEN`"]
pub struct ALLREFEN_W<'a> {
  w: &'a mut W,
}
impl<'a> ALLREFEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ALLREFEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "All references are disabled in VLLS."]
  #[inline(always)]
  pub fn allrefen_0(self) -> &'a mut W {
    self.variant(ALLREFEN_A::ALLREFEN_0)
  }
  #[doc = "All references are enabled in VLLS0/1."]
  #[inline(always)]
  pub fn allrefen_1(self) -> &'a mut W {
    self.variant(ALLREFEN_A::ALLREFEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
#[doc = "VDDIOVDDMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIOVDDMEN_A {
  #[doc = "0: VDDIO voltage monitor disabled in lp modes."]
  VDDIOVDDMEN_0,
  #[doc = "1: VDDIO voltage monitor enabled in lp modes. Note: voltage monitor is always disabled in VLLS0/1 modes."]
  VDDIOVDDMEN_1,
}
impl From<VDDIOVDDMEN_A> for bool {
  #[inline(always)]
  fn from(variant: VDDIOVDDMEN_A) -> Self {
    match variant {
      VDDIOVDDMEN_A::VDDIOVDDMEN_0 => false,
      VDDIOVDDMEN_A::VDDIOVDDMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `VDDIOVDDMEN`"]
pub type VDDIOVDDMEN_R = crate::R<bool, VDDIOVDDMEN_A>;
impl VDDIOVDDMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDDIOVDDMEN_A {
    match self.bits {
      false => VDDIOVDDMEN_A::VDDIOVDDMEN_0,
      true => VDDIOVDDMEN_A::VDDIOVDDMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDDIOVDDMEN_0`"]
  #[inline(always)]
  pub fn is_vddiovddmen_0(&self) -> bool {
    *self == VDDIOVDDMEN_A::VDDIOVDDMEN_0
  }
  #[doc = "Checks if the value of the field is `VDDIOVDDMEN_1`"]
  #[inline(always)]
  pub fn is_vddiovddmen_1(&self) -> bool {
    *self == VDDIOVDDMEN_A::VDDIOVDDMEN_1
  }
}
#[doc = "Write proxy for field `VDDIOVDDMEN`"]
pub struct VDDIOVDDMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> VDDIOVDDMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDDIOVDDMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VDDIO voltage monitor disabled in lp modes."]
  #[inline(always)]
  pub fn vddiovddmen_0(self) -> &'a mut W {
    self.variant(VDDIOVDDMEN_A::VDDIOVDDMEN_0)
  }
  #[doc = "VDDIO voltage monitor enabled in lp modes. Note: voltage monitor is always disabled in VLLS0/1 modes."]
  #[inline(always)]
  pub fn vddiovddmen_1(self) -> &'a mut W {
    self.variant(VDDIOVDDMEN_A::VDDIOVDDMEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "USBVDDMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVDDMEN_A {
  #[doc = "0: USB voltage monitor disabled in lp modes."]
  USBVDDMEN_0,
  #[doc = "1: USB voltage monitor enabled in lp modes. Note: voltage monitor is always disabled in VLLS0/1 modes."]
  USBVDDMEN_1,
}
impl From<USBVDDMEN_A> for bool {
  #[inline(always)]
  fn from(variant: USBVDDMEN_A) -> Self {
    match variant {
      USBVDDMEN_A::USBVDDMEN_0 => false,
      USBVDDMEN_A::USBVDDMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `USBVDDMEN`"]
pub type USBVDDMEN_R = crate::R<bool, USBVDDMEN_A>;
impl USBVDDMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USBVDDMEN_A {
    match self.bits {
      false => USBVDDMEN_A::USBVDDMEN_0,
      true => USBVDDMEN_A::USBVDDMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `USBVDDMEN_0`"]
  #[inline(always)]
  pub fn is_usbvddmen_0(&self) -> bool {
    *self == USBVDDMEN_A::USBVDDMEN_0
  }
  #[doc = "Checks if the value of the field is `USBVDDMEN_1`"]
  #[inline(always)]
  pub fn is_usbvddmen_1(&self) -> bool {
    *self == USBVDDMEN_A::USBVDDMEN_1
  }
}
#[doc = "Write proxy for field `USBVDDMEN`"]
pub struct USBVDDMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> USBVDDMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USBVDDMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB voltage monitor disabled in lp modes."]
  #[inline(always)]
  pub fn usbvddmen_0(self) -> &'a mut W {
    self.variant(USBVDDMEN_A::USBVDDMEN_0)
  }
  #[doc = "USB voltage monitor enabled in lp modes. Note: voltage monitor is always disabled in VLLS0/1 modes."]
  #[inline(always)]
  pub fn usbvddmen_1(self) -> &'a mut W {
    self.variant(USBVDDMEN_A::USBVDDMEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
#[doc = "RTCVDDMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCVDDMEN_A {
  #[doc = "0: RTC voltage monitor disabled in lp modes."]
  RTCVDDMEN_0,
  #[doc = "1: RTC voltage monitor enabled in lp modes. Note: voltage monitor is always disabled in VLLS0/1 modes."]
  RTCVDDMEN_1,
}
impl From<RTCVDDMEN_A> for bool {
  #[inline(always)]
  fn from(variant: RTCVDDMEN_A) -> Self {
    match variant {
      RTCVDDMEN_A::RTCVDDMEN_0 => false,
      RTCVDDMEN_A::RTCVDDMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `RTCVDDMEN`"]
pub type RTCVDDMEN_R = crate::R<bool, RTCVDDMEN_A>;
impl RTCVDDMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RTCVDDMEN_A {
    match self.bits {
      false => RTCVDDMEN_A::RTCVDDMEN_0,
      true => RTCVDDMEN_A::RTCVDDMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RTCVDDMEN_0`"]
  #[inline(always)]
  pub fn is_rtcvddmen_0(&self) -> bool {
    *self == RTCVDDMEN_A::RTCVDDMEN_0
  }
  #[doc = "Checks if the value of the field is `RTCVDDMEN_1`"]
  #[inline(always)]
  pub fn is_rtcvddmen_1(&self) -> bool {
    *self == RTCVDDMEN_A::RTCVDDMEN_1
  }
}
#[doc = "Write proxy for field `RTCVDDMEN`"]
pub struct RTCVDDMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> RTCVDDMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RTCVDDMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC voltage monitor disabled in lp modes."]
  #[inline(always)]
  pub fn rtcvddmen_0(self) -> &'a mut W {
    self.variant(RTCVDDMEN_A::RTCVDDMEN_0)
  }
  #[doc = "RTC voltage monitor enabled in lp modes. Note: voltage monitor is always disabled in VLLS0/1 modes."]
  #[inline(always)]
  pub fn rtcvddmen_1(self) -> &'a mut W {
    self.variant(RTCVDDMEN_A::RTCVDDMEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
impl R {
  #[doc = "Bit 1 - LPSEL"]
  #[inline(always)]
  pub fn lpsel(&self) -> LPSEL_R {
    LPSEL_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Bandgap Enable In Low Power Mode Operation"]
  #[inline(always)]
  pub fn bgen(&self) -> BGEN_R {
    BGEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Bandgap Buffer Enable"]
  #[inline(always)]
  pub fn bgben(&self) -> BGBEN_R {
    BGBEN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Bandgap Buffer Drive Select"]
  #[inline(always)]
  pub fn bgbds(&self) -> BGBDS_R {
    BGBDS_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 7 - LPO Enabled"]
  #[inline(always)]
  pub fn lpoen(&self) -> LPOEN_R {
    LPOEN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - POR Enabled"]
  #[inline(always)]
  pub fn poren(&self) -> POREN_R {
    POREN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - LVD Enabled"]
  #[inline(always)]
  pub fn lvden(&self) -> LVDEN_R {
    LVDEN_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 14 - LPHIDRIVE"]
  #[inline(always)]
  pub fn lphidrive(&self) -> LPHIDRIVE_R {
    LPHIDRIVE_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - All Reference Enable. This bit only has an affect in VLLS0/1."]
  #[inline(always)]
  pub fn allrefen(&self) -> ALLREFEN_R {
    ALLREFEN_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - VDDIOVDDMEN"]
  #[inline(always)]
  pub fn vddiovddmen(&self) -> VDDIOVDDMEN_R {
    VDDIOVDDMEN_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - USBVDDMEN"]
  #[inline(always)]
  pub fn usbvddmen(&self) -> USBVDDMEN_R {
    USBVDDMEN_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - RTCVDDMEN"]
  #[inline(always)]
  pub fn rtcvddmen(&self) -> RTCVDDMEN_R {
    RTCVDDMEN_R::new(((self.bits >> 18) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 1 - LPSEL"]
  #[inline(always)]
  pub fn lpsel(&mut self) -> LPSEL_W {
    LPSEL_W { w: self }
  }
  #[doc = "Bit 2 - Bandgap Enable In Low Power Mode Operation"]
  #[inline(always)]
  pub fn bgen(&mut self) -> BGEN_W {
    BGEN_W { w: self }
  }
  #[doc = "Bit 3 - Bandgap Buffer Enable"]
  #[inline(always)]
  pub fn bgben(&mut self) -> BGBEN_W {
    BGBEN_W { w: self }
  }
  #[doc = "Bit 4 - Bandgap Buffer Drive Select"]
  #[inline(always)]
  pub fn bgbds(&mut self) -> BGBDS_W {
    BGBDS_W { w: self }
  }
  #[doc = "Bit 7 - LPO Enabled"]
  #[inline(always)]
  pub fn lpoen(&mut self) -> LPOEN_W {
    LPOEN_W { w: self }
  }
  #[doc = "Bit 8 - POR Enabled"]
  #[inline(always)]
  pub fn poren(&mut self) -> POREN_W {
    POREN_W { w: self }
  }
  #[doc = "Bit 9 - LVD Enabled"]
  #[inline(always)]
  pub fn lvden(&mut self) -> LVDEN_W {
    LVDEN_W { w: self }
  }
  #[doc = "Bit 14 - LPHIDRIVE"]
  #[inline(always)]
  pub fn lphidrive(&mut self) -> LPHIDRIVE_W {
    LPHIDRIVE_W { w: self }
  }
  #[doc = "Bit 15 - All Reference Enable. This bit only has an affect in VLLS0/1."]
  #[inline(always)]
  pub fn allrefen(&mut self) -> ALLREFEN_W {
    ALLREFEN_W { w: self }
  }
  #[doc = "Bit 16 - VDDIOVDDMEN"]
  #[inline(always)]
  pub fn vddiovddmen(&mut self) -> VDDIOVDDMEN_W {
    VDDIOVDDMEN_W { w: self }
  }
  #[doc = "Bit 17 - USBVDDMEN"]
  #[inline(always)]
  pub fn usbvddmen(&mut self) -> USBVDDMEN_W {
    USBVDDMEN_W { w: self }
  }
  #[doc = "Bit 18 - RTCVDDMEN"]
  #[inline(always)]
  pub fn rtcvddmen(&mut self) -> RTCVDDMEN_W {
    RTCVDDMEN_W { w: self }
  }
}
