#[doc = "Reader of register SW_CONFIG"]
pub type R = crate::R<u32, super::SW_CONFIG>;
#[doc = "Writer for register SW_CONFIG"]
pub type W = crate::W<u32, super::SW_CONFIG>;
#[doc = "Register SW_CONFIG `reset()`'s with value 0x20"]
impl crate::ResetValue for super::SW_CONFIG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x20
  }
}
#[doc = "Reader of field `RADIO_CONFIGURED_POR_RESET`"]
pub type RADIO_CONFIGURED_POR_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_CONFIGURED_POR_RESET`"]
pub struct RADIO_CONFIGURED_POR_RESET_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_CONFIGURED_POR_RESET_W<'a> {
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
#[doc = "Reader of field `RADIO_CONFIGURED_SYS_RESET`"]
pub type RADIO_CONFIGURED_SYS_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_CONFIGURED_SYS_RESET`"]
pub struct RADIO_CONFIGURED_SYS_RESET_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_CONFIGURED_SYS_RESET_W<'a> {
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
#[doc = "Reader of field `RSIM_RF_ACTIVE_OVRD`"]
pub type RSIM_RF_ACTIVE_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_RF_ACTIVE_OVRD`"]
pub struct RSIM_RF_ACTIVE_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_RF_ACTIVE_OVRD_W<'a> {
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
#[doc = "Reader of field `RSIM_RF_ACTIVE_OVRD_EN`"]
pub type RSIM_RF_ACTIVE_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_RF_ACTIVE_OVRD_EN`"]
pub struct RSIM_RF_ACTIVE_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_RF_ACTIVE_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `RADIO_POR_BIT`"]
pub type RADIO_POR_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_POR_BIT`"]
pub struct RADIO_POR_BIT_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_POR_BIT_W<'a> {
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
#[doc = "Reader of field `RSIM_RADIO_ISO_POR_OVRD`"]
pub type RSIM_RADIO_ISO_POR_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_RADIO_ISO_POR_OVRD`"]
pub struct RSIM_RADIO_ISO_POR_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_RADIO_ISO_POR_OVRD_W<'a> {
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
#[doc = "Reader of field `RADIO_RESET_BIT`"]
pub type RADIO_RESET_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_RESET_BIT`"]
pub struct RADIO_RESET_BIT_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_RESET_BIT_W<'a> {
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
#[doc = "RSIM Wakeup Interrupt Source Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_INTERRUPT_SOURCE_A {
  #[doc = "0: No Radio Power-On Sequence interrupt will be generated."]
  WAKEUP_INTERRUPT_SOURCE_0,
  #[doc = "1: A Power-On Sequence interrupt will be generated when the RF Power Request occurs, including unblocked requests from an external source to use the RF OSC."]
  WAKEUP_INTERRUPT_SOURCE_1,
  #[doc = "2: A Power-On Sequence interrupt will be generated when the RF OSC Request occurs, but not if the RF OSC request was from an external source."]
  WAKEUP_INTERRUPT_SOURCE_2,
  #[doc = "3: A Power-On Sequence interrupt will be generated when the RSIM RF Active Warning occurs"]
  WAKEUP_INTERRUPT_SOURCE_3,
}
impl From<WAKEUP_INTERRUPT_SOURCE_A> for u8 {
  #[inline(always)]
  fn from(variant: WAKEUP_INTERRUPT_SOURCE_A) -> Self {
    match variant {
      WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_0 => 0,
      WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_1 => 1,
      WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_2 => 2,
      WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_3 => 3,
    }
  }
}
#[doc = "Reader of field `WAKEUP_INTERRUPT_SOURCE`"]
pub type WAKEUP_INTERRUPT_SOURCE_R = crate::R<u8, WAKEUP_INTERRUPT_SOURCE_A>;
impl WAKEUP_INTERRUPT_SOURCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WAKEUP_INTERRUPT_SOURCE_A {
    match self.bits {
      0 => WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_0,
      1 => WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_1,
      2 => WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_2,
      3 => WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WAKEUP_INTERRUPT_SOURCE_0`"]
  #[inline(always)]
  pub fn is_wakeup_interrupt_source_0(&self) -> bool {
    *self == WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_0
  }
  #[doc = "Checks if the value of the field is `WAKEUP_INTERRUPT_SOURCE_1`"]
  #[inline(always)]
  pub fn is_wakeup_interrupt_source_1(&self) -> bool {
    *self == WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_1
  }
  #[doc = "Checks if the value of the field is `WAKEUP_INTERRUPT_SOURCE_2`"]
  #[inline(always)]
  pub fn is_wakeup_interrupt_source_2(&self) -> bool {
    *self == WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_2
  }
  #[doc = "Checks if the value of the field is `WAKEUP_INTERRUPT_SOURCE_3`"]
  #[inline(always)]
  pub fn is_wakeup_interrupt_source_3(&self) -> bool {
    *self == WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_3
  }
}
#[doc = "Write proxy for field `WAKEUP_INTERRUPT_SOURCE`"]
pub struct WAKEUP_INTERRUPT_SOURCE_W<'a> {
  w: &'a mut W,
}
impl<'a> WAKEUP_INTERRUPT_SOURCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WAKEUP_INTERRUPT_SOURCE_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "No Radio Power-On Sequence interrupt will be generated."]
  #[inline(always)]
  pub fn wakeup_interrupt_source_0(self) -> &'a mut W {
    self.variant(WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_0)
  }
  #[doc = "A Power-On Sequence interrupt will be generated when the RF Power Request occurs, including unblocked requests from an external source to use the RF OSC."]
  #[inline(always)]
  pub fn wakeup_interrupt_source_1(self) -> &'a mut W {
    self.variant(WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_1)
  }
  #[doc = "A Power-On Sequence interrupt will be generated when the RF OSC Request occurs, but not if the RF OSC request was from an external source."]
  #[inline(always)]
  pub fn wakeup_interrupt_source_2(self) -> &'a mut W {
    self.variant(WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_2)
  }
  #[doc = "A Power-On Sequence interrupt will be generated when the RSIM RF Active Warning occurs"]
  #[inline(always)]
  pub fn wakeup_interrupt_source_3(self) -> &'a mut W {
    self.variant(WAKEUP_INTERRUPT_SOURCE_A::WAKEUP_INTERRUPT_SOURCE_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
    self.w
  }
}
#[doc = "Reader of field `RADIO0_INTERRUPT_EN`"]
pub type RADIO0_INTERRUPT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO0_INTERRUPT_EN`"]
pub struct RADIO0_INTERRUPT_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO0_INTERRUPT_EN_W<'a> {
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
#[doc = "Reader of field `RADIO1_INTERRUPT_EN`"]
pub type RADIO1_INTERRUPT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO1_INTERRUPT_EN`"]
pub struct RADIO1_INTERRUPT_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO1_INTERRUPT_EN_W<'a> {
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
#[doc = "Reader of field `BLOCK_SOC_RESETS`"]
pub type BLOCK_SOC_RESETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK_SOC_RESETS`"]
pub struct BLOCK_SOC_RESETS_W<'a> {
  w: &'a mut W,
}
impl<'a> BLOCK_SOC_RESETS_W<'a> {
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
#[doc = "Reader of field `BLOCK_RADIO_OUTPUTS`"]
pub type BLOCK_RADIO_OUTPUTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK_RADIO_OUTPUTS`"]
pub struct BLOCK_RADIO_OUTPUTS_W<'a> {
  w: &'a mut W,
}
impl<'a> BLOCK_RADIO_OUTPUTS_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "Reader of field `ALLOW_DFT_RESETS`"]
pub type ALLOW_DFT_RESETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLOW_DFT_RESETS`"]
pub struct ALLOW_DFT_RESETS_W<'a> {
  w: &'a mut W,
}
impl<'a> ALLOW_DFT_RESETS_W<'a> {
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
#[doc = "Reader of field `BLOCK_EXT_OSC_PWR_REQ`"]
pub type BLOCK_EXT_OSC_PWR_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK_EXT_OSC_PWR_REQ`"]
pub struct BLOCK_EXT_OSC_PWR_REQ_W<'a> {
  w: &'a mut W,
}
impl<'a> BLOCK_EXT_OSC_PWR_REQ_W<'a> {
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
  #[doc = "Bit 0 - Radio Configuration Bit, cleared by Radio Power On Reset"]
  #[inline(always)]
  pub fn radio_configured_por_reset(&self) -> RADIO_CONFIGURED_POR_RESET_R {
    RADIO_CONFIGURED_POR_RESET_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Radio Configuration Bit, cleared by Radio System Reset"]
  #[inline(always)]
  pub fn radio_configured_sys_reset(&self) -> RADIO_CONFIGURED_SYS_RESET_R {
    RADIO_CONFIGURED_SYS_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 4 - RF Active Internal Override"]
  #[inline(always)]
  pub fn rsim_rf_active_ovrd(&self) -> RSIM_RF_ACTIVE_OVRD_R {
    RSIM_RF_ACTIVE_OVRD_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - RF Active Internal Override Enable"]
  #[inline(always)]
  pub fn rsim_rf_active_ovrd_en(&self) -> RSIM_RF_ACTIVE_OVRD_EN_R {
    RSIM_RF_ACTIVE_OVRD_EN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Software Power On Reset for the Radio"]
  #[inline(always)]
  pub fn radio_por_bit(&self) -> RADIO_POR_BIT_R {
    RADIO_POR_BIT_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 12 - RSIM ISO_POR Override"]
  #[inline(always)]
  pub fn rsim_radio_iso_por_ovrd(&self) -> RSIM_RADIO_ISO_POR_OVRD_R {
    RSIM_RADIO_ISO_POR_OVRD_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Software System Reset for the Radio"]
  #[inline(always)]
  pub fn radio_reset_bit(&self) -> RADIO_RESET_BIT_R {
    RADIO_RESET_BIT_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bits 20:21 - RSIM Wakeup Interrupt Source Selector"]
  #[inline(always)]
  pub fn wakeup_interrupt_source(&self) -> WAKEUP_INTERRUPT_SOURCE_R {
    WAKEUP_INTERRUPT_SOURCE_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bit 24 - Radio0 Interrupt Enable"]
  #[inline(always)]
  pub fn radio0_interrupt_en(&self) -> RADIO0_INTERRUPT_EN_R {
    RADIO0_INTERRUPT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Radio1 Interrupt Enable"]
  #[inline(always)]
  pub fn radio1_interrupt_en(&self) -> RADIO1_INTERRUPT_EN_R {
    RADIO1_INTERRUPT_EN_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Block SoC Resets of the Radio, cleared by Radio System Reset"]
  #[inline(always)]
  pub fn block_soc_resets(&self) -> BLOCK_SOC_RESETS_R {
    BLOCK_SOC_RESETS_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Block Radio Outputs"]
  #[inline(always)]
  pub fn block_radio_outputs(&self) -> BLOCK_RADIO_OUTPUTS_R {
    BLOCK_RADIO_OUTPUTS_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Allow the DFT Reset Pin to Reset the Radio"]
  #[inline(always)]
  pub fn allow_dft_resets(&self) -> ALLOW_DFT_RESETS_R {
    ALLOW_DFT_RESETS_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Block External Requests for RF OSC from starting a Radio Power Wakeup Sequence"]
  #[inline(always)]
  pub fn block_ext_osc_pwr_req(&self) -> BLOCK_EXT_OSC_PWR_REQ_R {
    BLOCK_EXT_OSC_PWR_REQ_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Radio Configuration Bit, cleared by Radio Power On Reset"]
  #[inline(always)]
  pub fn radio_configured_por_reset(&mut self) -> RADIO_CONFIGURED_POR_RESET_W {
    RADIO_CONFIGURED_POR_RESET_W { w: self }
  }
  #[doc = "Bit 1 - Radio Configuration Bit, cleared by Radio System Reset"]
  #[inline(always)]
  pub fn radio_configured_sys_reset(&mut self) -> RADIO_CONFIGURED_SYS_RESET_W {
    RADIO_CONFIGURED_SYS_RESET_W { w: self }
  }
  #[doc = "Bit 4 - RF Active Internal Override"]
  #[inline(always)]
  pub fn rsim_rf_active_ovrd(&mut self) -> RSIM_RF_ACTIVE_OVRD_W {
    RSIM_RF_ACTIVE_OVRD_W { w: self }
  }
  #[doc = "Bit 5 - RF Active Internal Override Enable"]
  #[inline(always)]
  pub fn rsim_rf_active_ovrd_en(&mut self) -> RSIM_RF_ACTIVE_OVRD_EN_W {
    RSIM_RF_ACTIVE_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 8 - Software Power On Reset for the Radio"]
  #[inline(always)]
  pub fn radio_por_bit(&mut self) -> RADIO_POR_BIT_W {
    RADIO_POR_BIT_W { w: self }
  }
  #[doc = "Bit 12 - RSIM ISO_POR Override"]
  #[inline(always)]
  pub fn rsim_radio_iso_por_ovrd(&mut self) -> RSIM_RADIO_ISO_POR_OVRD_W {
    RSIM_RADIO_ISO_POR_OVRD_W { w: self }
  }
  #[doc = "Bit 16 - Software System Reset for the Radio"]
  #[inline(always)]
  pub fn radio_reset_bit(&mut self) -> RADIO_RESET_BIT_W {
    RADIO_RESET_BIT_W { w: self }
  }
  #[doc = "Bits 20:21 - RSIM Wakeup Interrupt Source Selector"]
  #[inline(always)]
  pub fn wakeup_interrupt_source(&mut self) -> WAKEUP_INTERRUPT_SOURCE_W {
    WAKEUP_INTERRUPT_SOURCE_W { w: self }
  }
  #[doc = "Bit 24 - Radio0 Interrupt Enable"]
  #[inline(always)]
  pub fn radio0_interrupt_en(&mut self) -> RADIO0_INTERRUPT_EN_W {
    RADIO0_INTERRUPT_EN_W { w: self }
  }
  #[doc = "Bit 25 - Radio1 Interrupt Enable"]
  #[inline(always)]
  pub fn radio1_interrupt_en(&mut self) -> RADIO1_INTERRUPT_EN_W {
    RADIO1_INTERRUPT_EN_W { w: self }
  }
  #[doc = "Bit 28 - Block SoC Resets of the Radio, cleared by Radio System Reset"]
  #[inline(always)]
  pub fn block_soc_resets(&mut self) -> BLOCK_SOC_RESETS_W {
    BLOCK_SOC_RESETS_W { w: self }
  }
  #[doc = "Bit 29 - Block Radio Outputs"]
  #[inline(always)]
  pub fn block_radio_outputs(&mut self) -> BLOCK_RADIO_OUTPUTS_W {
    BLOCK_RADIO_OUTPUTS_W { w: self }
  }
  #[doc = "Bit 30 - Allow the DFT Reset Pin to Reset the Radio"]
  #[inline(always)]
  pub fn allow_dft_resets(&mut self) -> ALLOW_DFT_RESETS_W {
    ALLOW_DFT_RESETS_W { w: self }
  }
  #[doc = "Bit 31 - Block External Requests for RF OSC from starting a Radio Power Wakeup Sequence"]
  #[inline(always)]
  pub fn block_ext_osc_pwr_req(&mut self) -> BLOCK_EXT_OSC_PWR_REQ_W {
    BLOCK_EXT_OSC_PWR_REQ_W { w: self }
  }
}
