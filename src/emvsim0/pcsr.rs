#[doc = "Reader of register PCSR"]
pub type R = crate::R<u32, super::PCSR>;
#[doc = "Writer for register PCSR"]
pub type W = crate::W<u32, super::PCSR>;
#[doc = "Register PCSR `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::PCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0100_0000
  }
}
#[doc = "Auto Power Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAPD_A {
  #[doc = "0: Auto power down disabled (default)"]
  SAPD_0,
  #[doc = "1: Auto power down enabled"]
  SAPD_1,
}
impl From<SAPD_A> for bool {
  #[inline(always)]
  fn from(variant: SAPD_A) -> Self {
    match variant {
      SAPD_A::SAPD_0 => false,
      SAPD_A::SAPD_1 => true,
    }
  }
}
#[doc = "Reader of field `SAPD`"]
pub type SAPD_R = crate::R<bool, SAPD_A>;
impl SAPD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SAPD_A {
    match self.bits {
      false => SAPD_A::SAPD_0,
      true => SAPD_A::SAPD_1,
    }
  }
  #[doc = "Checks if the value of the field is `SAPD_0`"]
  #[inline(always)]
  pub fn is_sapd_0(&self) -> bool {
    *self == SAPD_A::SAPD_0
  }
  #[doc = "Checks if the value of the field is `SAPD_1`"]
  #[inline(always)]
  pub fn is_sapd_1(&self) -> bool {
    *self == SAPD_A::SAPD_1
  }
}
#[doc = "Write proxy for field `SAPD`"]
pub struct SAPD_W<'a> {
  w: &'a mut W,
}
impl<'a> SAPD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SAPD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Auto power down disabled (default)"]
  #[inline(always)]
  pub fn sapd_0(self) -> &'a mut W {
    self.variant(SAPD_A::SAPD_0)
  }
  #[doc = "Auto power down enabled"]
  #[inline(always)]
  pub fn sapd_1(self) -> &'a mut W {
    self.variant(SAPD_A::SAPD_1)
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
#[doc = "Vcc Enable for Smart Card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCC_EN_A {
  #[doc = "0: Smart Card Voltage disabled (default)"]
  SVCC_EN_0,
  #[doc = "1: Smart Card Voltage enabled"]
  SVCC_EN_1,
}
impl From<SVCC_EN_A> for bool {
  #[inline(always)]
  fn from(variant: SVCC_EN_A) -> Self {
    match variant {
      SVCC_EN_A::SVCC_EN_0 => false,
      SVCC_EN_A::SVCC_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `SVCC_EN`"]
pub type SVCC_EN_R = crate::R<bool, SVCC_EN_A>;
impl SVCC_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SVCC_EN_A {
    match self.bits {
      false => SVCC_EN_A::SVCC_EN_0,
      true => SVCC_EN_A::SVCC_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SVCC_EN_0`"]
  #[inline(always)]
  pub fn is_svcc_en_0(&self) -> bool {
    *self == SVCC_EN_A::SVCC_EN_0
  }
  #[doc = "Checks if the value of the field is `SVCC_EN_1`"]
  #[inline(always)]
  pub fn is_svcc_en_1(&self) -> bool {
    *self == SVCC_EN_A::SVCC_EN_1
  }
}
#[doc = "Write proxy for field `SVCC_EN`"]
pub struct SVCC_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> SVCC_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SVCC_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Smart Card Voltage disabled (default)"]
  #[inline(always)]
  pub fn svcc_en_0(self) -> &'a mut W {
    self.variant(SVCC_EN_A::SVCC_EN_0)
  }
  #[doc = "Smart Card Voltage enabled"]
  #[inline(always)]
  pub fn svcc_en_1(self) -> &'a mut W {
    self.variant(SVCC_EN_A::SVCC_EN_1)
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
#[doc = "VCC Enable Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCCENP_A {
  #[doc = "0: VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
  VCCENP_0,
  #[doc = "1: VCC_EN is active low. Polarity of SVCC_EN is inverted."]
  VCCENP_1,
}
impl From<VCCENP_A> for bool {
  #[inline(always)]
  fn from(variant: VCCENP_A) -> Self {
    match variant {
      VCCENP_A::VCCENP_0 => false,
      VCCENP_A::VCCENP_1 => true,
    }
  }
}
#[doc = "Reader of field `VCCENP`"]
pub type VCCENP_R = crate::R<bool, VCCENP_A>;
impl VCCENP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VCCENP_A {
    match self.bits {
      false => VCCENP_A::VCCENP_0,
      true => VCCENP_A::VCCENP_1,
    }
  }
  #[doc = "Checks if the value of the field is `VCCENP_0`"]
  #[inline(always)]
  pub fn is_vccenp_0(&self) -> bool {
    *self == VCCENP_A::VCCENP_0
  }
  #[doc = "Checks if the value of the field is `VCCENP_1`"]
  #[inline(always)]
  pub fn is_vccenp_1(&self) -> bool {
    *self == VCCENP_A::VCCENP_1
  }
}
#[doc = "Write proxy for field `VCCENP`"]
pub struct VCCENP_W<'a> {
  w: &'a mut W,
}
impl<'a> VCCENP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VCCENP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
  #[inline(always)]
  pub fn vccenp_0(self) -> &'a mut W {
    self.variant(VCCENP_A::VCCENP_0)
  }
  #[doc = "VCC_EN is active low. Polarity of SVCC_EN is inverted."]
  #[inline(always)]
  pub fn vccenp_1(self) -> &'a mut W {
    self.variant(VCCENP_A::VCCENP_1)
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
#[doc = "Reset to Smart Card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_A {
  #[doc = "0: Smart Card Reset is asserted (default)"]
  SRST_0,
  #[doc = "1: Smart Card Reset is de-asserted"]
  SRST_1,
}
impl From<SRST_A> for bool {
  #[inline(always)]
  fn from(variant: SRST_A) -> Self {
    match variant {
      SRST_A::SRST_0 => false,
      SRST_A::SRST_1 => true,
    }
  }
}
#[doc = "Reader of field `SRST`"]
pub type SRST_R = crate::R<bool, SRST_A>;
impl SRST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SRST_A {
    match self.bits {
      false => SRST_A::SRST_0,
      true => SRST_A::SRST_1,
    }
  }
  #[doc = "Checks if the value of the field is `SRST_0`"]
  #[inline(always)]
  pub fn is_srst_0(&self) -> bool {
    *self == SRST_A::SRST_0
  }
  #[doc = "Checks if the value of the field is `SRST_1`"]
  #[inline(always)]
  pub fn is_srst_1(&self) -> bool {
    *self == SRST_A::SRST_1
  }
}
#[doc = "Write proxy for field `SRST`"]
pub struct SRST_W<'a> {
  w: &'a mut W,
}
impl<'a> SRST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SRST_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Smart Card Reset is asserted (default)"]
  #[inline(always)]
  pub fn srst_0(self) -> &'a mut W {
    self.variant(SRST_A::SRST_0)
  }
  #[doc = "Smart Card Reset is de-asserted"]
  #[inline(always)]
  pub fn srst_1(self) -> &'a mut W {
    self.variant(SRST_A::SRST_1)
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
#[doc = "Clock Enable for Smart Card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCEN_A {
  #[doc = "0: Smart Card Clock Disabled"]
  SCEN_0,
  #[doc = "1: Smart Card Clock Enabled"]
  SCEN_1,
}
impl From<SCEN_A> for bool {
  #[inline(always)]
  fn from(variant: SCEN_A) -> Self {
    match variant {
      SCEN_A::SCEN_0 => false,
      SCEN_A::SCEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SCEN`"]
pub type SCEN_R = crate::R<bool, SCEN_A>;
impl SCEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SCEN_A {
    match self.bits {
      false => SCEN_A::SCEN_0,
      true => SCEN_A::SCEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SCEN_0`"]
  #[inline(always)]
  pub fn is_scen_0(&self) -> bool {
    *self == SCEN_A::SCEN_0
  }
  #[doc = "Checks if the value of the field is `SCEN_1`"]
  #[inline(always)]
  pub fn is_scen_1(&self) -> bool {
    *self == SCEN_A::SCEN_1
  }
}
#[doc = "Write proxy for field `SCEN`"]
pub struct SCEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SCEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SCEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Smart Card Clock Disabled"]
  #[inline(always)]
  pub fn scen_0(self) -> &'a mut W {
    self.variant(SCEN_A::SCEN_0)
  }
  #[doc = "Smart Card Clock Enabled"]
  #[inline(always)]
  pub fn scen_1(self) -> &'a mut W {
    self.variant(SCEN_A::SCEN_1)
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
#[doc = "Smart Card Clock Stop Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSP_A {
  #[doc = "0: Clock is logic 0 when stopped by SCEN"]
  SCSP_0,
  #[doc = "1: Clock is logic 1 when stopped by SCEN"]
  SCSP_1,
}
impl From<SCSP_A> for bool {
  #[inline(always)]
  fn from(variant: SCSP_A) -> Self {
    match variant {
      SCSP_A::SCSP_0 => false,
      SCSP_A::SCSP_1 => true,
    }
  }
}
#[doc = "Reader of field `SCSP`"]
pub type SCSP_R = crate::R<bool, SCSP_A>;
impl SCSP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SCSP_A {
    match self.bits {
      false => SCSP_A::SCSP_0,
      true => SCSP_A::SCSP_1,
    }
  }
  #[doc = "Checks if the value of the field is `SCSP_0`"]
  #[inline(always)]
  pub fn is_scsp_0(&self) -> bool {
    *self == SCSP_A::SCSP_0
  }
  #[doc = "Checks if the value of the field is `SCSP_1`"]
  #[inline(always)]
  pub fn is_scsp_1(&self) -> bool {
    *self == SCSP_A::SCSP_1
  }
}
#[doc = "Write proxy for field `SCSP`"]
pub struct SCSP_W<'a> {
  w: &'a mut W,
}
impl<'a> SCSP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SCSP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock is logic 0 when stopped by SCEN"]
  #[inline(always)]
  pub fn scsp_0(self) -> &'a mut W {
    self.variant(SCSP_A::SCSP_0)
  }
  #[doc = "Clock is logic 1 when stopped by SCEN"]
  #[inline(always)]
  pub fn scsp_1(self) -> &'a mut W {
    self.variant(SCSP_A::SCSP_1)
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
#[doc = "Auto Power Down Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPD_A {
  #[doc = "0: No effect (default)"]
  SPD_0,
  #[doc = "1: Start Auto Powerdown or Power Down is in progress"]
  SPD_1,
}
impl From<SPD_A> for bool {
  #[inline(always)]
  fn from(variant: SPD_A) -> Self {
    match variant {
      SPD_A::SPD_0 => false,
      SPD_A::SPD_1 => true,
    }
  }
}
#[doc = "Reader of field `SPD`"]
pub type SPD_R = crate::R<bool, SPD_A>;
impl SPD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SPD_A {
    match self.bits {
      false => SPD_A::SPD_0,
      true => SPD_A::SPD_1,
    }
  }
  #[doc = "Checks if the value of the field is `SPD_0`"]
  #[inline(always)]
  pub fn is_spd_0(&self) -> bool {
    *self == SPD_A::SPD_0
  }
  #[doc = "Checks if the value of the field is `SPD_1`"]
  #[inline(always)]
  pub fn is_spd_1(&self) -> bool {
    *self == SPD_A::SPD_1
  }
}
#[doc = "Write proxy for field `SPD`"]
pub struct SPD_W<'a> {
  w: &'a mut W,
}
impl<'a> SPD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SPD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect (default)"]
  #[inline(always)]
  pub fn spd_0(self) -> &'a mut W {
    self.variant(SPD_A::SPD_0)
  }
  #[doc = "Start Auto Powerdown or Power Down is in progress"]
  #[inline(always)]
  pub fn spd_1(self) -> &'a mut W {
    self.variant(SPD_A::SPD_1)
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
#[doc = "Smart Card Presence Detect Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIM_A {
  #[doc = "0: SIM presence detect interrupt is enabled"]
  SPDIM_0,
  #[doc = "1: SIM presence detect interrupt is masked (default)"]
  SPDIM_1,
}
impl From<SPDIM_A> for bool {
  #[inline(always)]
  fn from(variant: SPDIM_A) -> Self {
    match variant {
      SPDIM_A::SPDIM_0 => false,
      SPDIM_A::SPDIM_1 => true,
    }
  }
}
#[doc = "Reader of field `SPDIM`"]
pub type SPDIM_R = crate::R<bool, SPDIM_A>;
impl SPDIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SPDIM_A {
    match self.bits {
      false => SPDIM_A::SPDIM_0,
      true => SPDIM_A::SPDIM_1,
    }
  }
  #[doc = "Checks if the value of the field is `SPDIM_0`"]
  #[inline(always)]
  pub fn is_spdim_0(&self) -> bool {
    *self == SPDIM_A::SPDIM_0
  }
  #[doc = "Checks if the value of the field is `SPDIM_1`"]
  #[inline(always)]
  pub fn is_spdim_1(&self) -> bool {
    *self == SPDIM_A::SPDIM_1
  }
}
#[doc = "Write proxy for field `SPDIM`"]
pub struct SPDIM_W<'a> {
  w: &'a mut W,
}
impl<'a> SPDIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SPDIM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "SIM presence detect interrupt is enabled"]
  #[inline(always)]
  pub fn spdim_0(self) -> &'a mut W {
    self.variant(SPDIM_A::SPDIM_0)
  }
  #[doc = "SIM presence detect interrupt is masked (default)"]
  #[inline(always)]
  pub fn spdim_1(self) -> &'a mut W {
    self.variant(SPDIM_A::SPDIM_1)
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
#[doc = "Smart Card Presence Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF_A {
  #[doc = "0: No insertion or removal of Smart Card detected on Port (default)"]
  SPDIF_0,
  #[doc = "1: Insertion or removal of Smart Card detected on Port"]
  SPDIF_1,
}
impl From<SPDIF_A> for bool {
  #[inline(always)]
  fn from(variant: SPDIF_A) -> Self {
    match variant {
      SPDIF_A::SPDIF_0 => false,
      SPDIF_A::SPDIF_1 => true,
    }
  }
}
#[doc = "Reader of field `SPDIF`"]
pub type SPDIF_R = crate::R<bool, SPDIF_A>;
impl SPDIF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SPDIF_A {
    match self.bits {
      false => SPDIF_A::SPDIF_0,
      true => SPDIF_A::SPDIF_1,
    }
  }
  #[doc = "Checks if the value of the field is `SPDIF_0`"]
  #[inline(always)]
  pub fn is_spdif_0(&self) -> bool {
    *self == SPDIF_A::SPDIF_0
  }
  #[doc = "Checks if the value of the field is `SPDIF_1`"]
  #[inline(always)]
  pub fn is_spdif_1(&self) -> bool {
    *self == SPDIF_A::SPDIF_1
  }
}
#[doc = "Write proxy for field `SPDIF`"]
pub struct SPDIF_W<'a> {
  w: &'a mut W,
}
impl<'a> SPDIF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SPDIF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No insertion or removal of Smart Card detected on Port (default)"]
  #[inline(always)]
  pub fn spdif_0(self) -> &'a mut W {
    self.variant(SPDIF_A::SPDIF_0)
  }
  #[doc = "Insertion or removal of Smart Card detected on Port"]
  #[inline(always)]
  pub fn spdif_1(self) -> &'a mut W {
    self.variant(SPDIF_A::SPDIF_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
    self.w
  }
}
#[doc = "Smart Card Presence Detect Pin Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDP_A {
  #[doc = "0: SIM Presence Detect pin is logic low"]
  SPDP_0,
  #[doc = "1: SIM Presence Detectpin is logic high"]
  SPDP_1,
}
impl From<SPDP_A> for bool {
  #[inline(always)]
  fn from(variant: SPDP_A) -> Self {
    match variant {
      SPDP_A::SPDP_0 => false,
      SPDP_A::SPDP_1 => true,
    }
  }
}
#[doc = "Reader of field `SPDP`"]
pub type SPDP_R = crate::R<bool, SPDP_A>;
impl SPDP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SPDP_A {
    match self.bits {
      false => SPDP_A::SPDP_0,
      true => SPDP_A::SPDP_1,
    }
  }
  #[doc = "Checks if the value of the field is `SPDP_0`"]
  #[inline(always)]
  pub fn is_spdp_0(&self) -> bool {
    *self == SPDP_A::SPDP_0
  }
  #[doc = "Checks if the value of the field is `SPDP_1`"]
  #[inline(always)]
  pub fn is_spdp_1(&self) -> bool {
    *self == SPDP_A::SPDP_1
  }
}
#[doc = "SIM Presence Detect Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDES_A {
  #[doc = "0: Falling edge on the pin (default)"]
  SPDES_0,
  #[doc = "1: Rising edge on the pin"]
  SPDES_1,
}
impl From<SPDES_A> for bool {
  #[inline(always)]
  fn from(variant: SPDES_A) -> Self {
    match variant {
      SPDES_A::SPDES_0 => false,
      SPDES_A::SPDES_1 => true,
    }
  }
}
#[doc = "Reader of field `SPDES`"]
pub type SPDES_R = crate::R<bool, SPDES_A>;
impl SPDES_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SPDES_A {
    match self.bits {
      false => SPDES_A::SPDES_0,
      true => SPDES_A::SPDES_1,
    }
  }
  #[doc = "Checks if the value of the field is `SPDES_0`"]
  #[inline(always)]
  pub fn is_spdes_0(&self) -> bool {
    *self == SPDES_A::SPDES_0
  }
  #[doc = "Checks if the value of the field is `SPDES_1`"]
  #[inline(always)]
  pub fn is_spdes_1(&self) -> bool {
    *self == SPDES_A::SPDES_1
  }
}
#[doc = "Write proxy for field `SPDES`"]
pub struct SPDES_W<'a> {
  w: &'a mut W,
}
impl<'a> SPDES_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SPDES_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Falling edge on the pin (default)"]
  #[inline(always)]
  pub fn spdes_0(self) -> &'a mut W {
    self.variant(SPDES_A::SPDES_0)
  }
  #[doc = "Rising edge on the pin"]
  #[inline(always)]
  pub fn spdes_1(self) -> &'a mut W {
    self.variant(SPDES_A::SPDES_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Auto Power Down Enable"]
  #[inline(always)]
  pub fn sapd(&self) -> SAPD_R {
    SAPD_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Vcc Enable for Smart Card"]
  #[inline(always)]
  pub fn svcc_en(&self) -> SVCC_EN_R {
    SVCC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - VCC Enable Polarity Control"]
  #[inline(always)]
  pub fn vccenp(&self) -> VCCENP_R {
    VCCENP_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Reset to Smart Card"]
  #[inline(always)]
  pub fn srst(&self) -> SRST_R {
    SRST_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Clock Enable for Smart Card"]
  #[inline(always)]
  pub fn scen(&self) -> SCEN_R {
    SCEN_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Smart Card Clock Stop Polarity"]
  #[inline(always)]
  pub fn scsp(&self) -> SCSP_R {
    SCSP_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Auto Power Down Control"]
  #[inline(always)]
  pub fn spd(&self) -> SPD_R {
    SPD_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Smart Card Presence Detect Interrupt Mask"]
  #[inline(always)]
  pub fn spdim(&self) -> SPDIM_R {
    SPDIM_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Smart Card Presence Detect Interrupt Flag"]
  #[inline(always)]
  pub fn spdif(&self) -> SPDIF_R {
    SPDIF_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Smart Card Presence Detect Pin Status"]
  #[inline(always)]
  pub fn spdp(&self) -> SPDP_R {
    SPDP_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - SIM Presence Detect Edge Select"]
  #[inline(always)]
  pub fn spdes(&self) -> SPDES_R {
    SPDES_R::new(((self.bits >> 27) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Auto Power Down Enable"]
  #[inline(always)]
  pub fn sapd(&mut self) -> SAPD_W {
    SAPD_W { w: self }
  }
  #[doc = "Bit 1 - Vcc Enable for Smart Card"]
  #[inline(always)]
  pub fn svcc_en(&mut self) -> SVCC_EN_W {
    SVCC_EN_W { w: self }
  }
  #[doc = "Bit 2 - VCC Enable Polarity Control"]
  #[inline(always)]
  pub fn vccenp(&mut self) -> VCCENP_W {
    VCCENP_W { w: self }
  }
  #[doc = "Bit 3 - Reset to Smart Card"]
  #[inline(always)]
  pub fn srst(&mut self) -> SRST_W {
    SRST_W { w: self }
  }
  #[doc = "Bit 4 - Clock Enable for Smart Card"]
  #[inline(always)]
  pub fn scen(&mut self) -> SCEN_W {
    SCEN_W { w: self }
  }
  #[doc = "Bit 5 - Smart Card Clock Stop Polarity"]
  #[inline(always)]
  pub fn scsp(&mut self) -> SCSP_W {
    SCSP_W { w: self }
  }
  #[doc = "Bit 7 - Auto Power Down Control"]
  #[inline(always)]
  pub fn spd(&mut self) -> SPD_W {
    SPD_W { w: self }
  }
  #[doc = "Bit 24 - Smart Card Presence Detect Interrupt Mask"]
  #[inline(always)]
  pub fn spdim(&mut self) -> SPDIM_W {
    SPDIM_W { w: self }
  }
  #[doc = "Bit 25 - Smart Card Presence Detect Interrupt Flag"]
  #[inline(always)]
  pub fn spdif(&mut self) -> SPDIF_W {
    SPDIF_W { w: self }
  }
  #[doc = "Bit 27 - SIM Presence Detect Edge Select"]
  #[inline(always)]
  pub fn spdes(&mut self) -> SPDES_W {
    SPDES_W { w: self }
  }
}
