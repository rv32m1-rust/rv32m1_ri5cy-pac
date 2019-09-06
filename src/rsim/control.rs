#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `BLE_RF_POWER_REQ_EN`"]
pub type BLE_RF_POWER_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLE_RF_POWER_REQ_EN`"]
pub struct BLE_RF_POWER_REQ_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> BLE_RF_POWER_REQ_EN_W<'a> {
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
#[doc = "Reader of field `BLE_RF_POWER_REQ_STAT`"]
pub type BLE_RF_POWER_REQ_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLE_RF_POWER_REQ_INT_EN`"]
pub type BLE_RF_POWER_REQ_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLE_RF_POWER_REQ_INT_EN`"]
pub struct BLE_RF_POWER_REQ_INT_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> BLE_RF_POWER_REQ_INT_EN_W<'a> {
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
#[doc = "Reader of field `BLE_RF_POWER_REQ_INT`"]
pub type BLE_RF_POWER_REQ_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLE_RF_POWER_REQ_INT`"]
pub struct BLE_RF_POWER_REQ_INT_W<'a> {
  w: &'a mut W,
}
impl<'a> BLE_RF_POWER_REQ_INT_W<'a> {
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
#[doc = "Reader of field `RF_OSC_EN`"]
pub type RF_OSC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_OSC_EN`"]
pub struct RF_OSC_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_OSC_EN_W<'a> {
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
#[doc = "Reader of field `RADIO_GASKET_BYPASS_OVRD_EN`"]
pub type RADIO_GASKET_BYPASS_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_GASKET_BYPASS_OVRD_EN`"]
pub struct RADIO_GASKET_BYPASS_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_GASKET_BYPASS_OVRD_EN_W<'a> {
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
#[doc = "Radio Gasket Bypass Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO_GASKET_BYPASS_OVRD_A {
  #[doc = "0: XCVR and Link Layer Register Clock is the RF Ref Osc Clock"]
  RADIO_GASKET_BYPASS_OVRD_0,
  #[doc = "1: XCVR and Link Layer Register Clock is the SoC IPG Clock"]
  RADIO_GASKET_BYPASS_OVRD_1,
}
impl From<RADIO_GASKET_BYPASS_OVRD_A> for bool {
  #[inline(always)]
  fn from(variant: RADIO_GASKET_BYPASS_OVRD_A) -> Self {
    match variant {
      RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_0 => false,
      RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_1 => true,
    }
  }
}
#[doc = "Reader of field `RADIO_GASKET_BYPASS_OVRD`"]
pub type RADIO_GASKET_BYPASS_OVRD_R = crate::R<bool, RADIO_GASKET_BYPASS_OVRD_A>;
impl RADIO_GASKET_BYPASS_OVRD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RADIO_GASKET_BYPASS_OVRD_A {
    match self.bits {
      false => RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_0,
      true => RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_1,
    }
  }
  #[doc = "Checks if the value of the field is `RADIO_GASKET_BYPASS_OVRD_0`"]
  #[inline(always)]
  pub fn is_radio_gasket_bypass_ovrd_0(&self) -> bool {
    *self == RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_0
  }
  #[doc = "Checks if the value of the field is `RADIO_GASKET_BYPASS_OVRD_1`"]
  #[inline(always)]
  pub fn is_radio_gasket_bypass_ovrd_1(&self) -> bool {
    *self == RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_1
  }
}
#[doc = "Write proxy for field `RADIO_GASKET_BYPASS_OVRD`"]
pub struct RADIO_GASKET_BYPASS_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_GASKET_BYPASS_OVRD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RADIO_GASKET_BYPASS_OVRD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "XCVR and Link Layer Register Clock is the RF Ref Osc Clock"]
  #[inline(always)]
  pub fn radio_gasket_bypass_ovrd_0(self) -> &'a mut W {
    self.variant(RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_0)
  }
  #[doc = "XCVR and Link Layer Register Clock is the SoC IPG Clock"]
  #[inline(always)]
  pub fn radio_gasket_bypass_ovrd_1(self) -> &'a mut W {
    self.variant(RADIO_GASKET_BYPASS_OVRD_A::RADIO_GASKET_BYPASS_OVRD_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
    self.w
  }
}
#[doc = "Reader of field `IPP_OBE_BLE_EARLY_WARNING`"]
pub type IPP_OBE_BLE_EARLY_WARNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPP_OBE_BLE_EARLY_WARNING`"]
pub struct IPP_OBE_BLE_EARLY_WARNING_W<'a> {
  w: &'a mut W,
}
impl<'a> IPP_OBE_BLE_EARLY_WARNING_W<'a> {
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
#[doc = "Reader of field `IPP_OBE_RF_ACTIVE`"]
pub type IPP_OBE_RF_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPP_OBE_RF_ACTIVE`"]
pub struct IPP_OBE_RF_ACTIVE_W<'a> {
  w: &'a mut W,
}
impl<'a> IPP_OBE_RF_ACTIVE_W<'a> {
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
#[doc = "Reader of field `IPP_OBE_RF_OSC_EN`"]
pub type IPP_OBE_RF_OSC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPP_OBE_RF_OSC_EN`"]
pub struct IPP_OBE_RF_OSC_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> IPP_OBE_RF_OSC_EN_W<'a> {
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
#[doc = "Reader of field `IPP_OBE_RF_STATUS`"]
pub type IPP_OBE_RF_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPP_OBE_RF_STATUS`"]
pub struct IPP_OBE_RF_STATUS_W<'a> {
  w: &'a mut W,
}
impl<'a> IPP_OBE_RF_STATUS_W<'a> {
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
#[doc = "Reader of field `IPP_OBE_RF_PRIORITY`"]
pub type IPP_OBE_RF_PRIORITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPP_OBE_RF_PRIORITY`"]
pub struct IPP_OBE_RF_PRIORITY_W<'a> {
  w: &'a mut W,
}
impl<'a> IPP_OBE_RF_PRIORITY_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
    self.w
  }
}
#[doc = "Reader of field `BLE_DSM_EXIT`"]
pub type BLE_DSM_EXIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLE_DSM_EXIT`"]
pub struct BLE_DSM_EXIT_W<'a> {
  w: &'a mut W,
}
impl<'a> BLE_DSM_EXIT_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
    self.w
  }
}
#[doc = "Reader of field `WOR_DSM_EXIT`"]
pub type WOR_DSM_EXIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOR_DSM_EXIT`"]
pub struct WOR_DSM_EXIT_W<'a> {
  w: &'a mut W,
}
impl<'a> WOR_DSM_EXIT_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
    self.w
  }
}
#[doc = "Reader of field `RF_OSC_READY`"]
pub type RF_OSC_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF_OSC_READY_OVRD_EN`"]
pub type RF_OSC_READY_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_OSC_READY_OVRD_EN`"]
pub struct RF_OSC_READY_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_OSC_READY_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `RF_OSC_READY_OVRD`"]
pub type RF_OSC_READY_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_OSC_READY_OVRD`"]
pub struct RF_OSC_READY_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_OSC_READY_OVRD_W<'a> {
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
#[doc = "BLE Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIM_CGC_BLE_EN_A {
  #[doc = "0: Clock disabled"]
  RSIM_CGC_BLE_EN_0,
  #[doc = "1: Clock enabled"]
  RSIM_CGC_BLE_EN_1,
}
impl From<RSIM_CGC_BLE_EN_A> for bool {
  #[inline(always)]
  fn from(variant: RSIM_CGC_BLE_EN_A) -> Self {
    match variant {
      RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_0 => false,
      RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `RSIM_CGC_BLE_EN`"]
pub type RSIM_CGC_BLE_EN_R = crate::R<bool, RSIM_CGC_BLE_EN_A>;
impl RSIM_CGC_BLE_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSIM_CGC_BLE_EN_A {
    match self.bits {
      false => RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_0,
      true => RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_BLE_EN_0`"]
  #[inline(always)]
  pub fn is_rsim_cgc_ble_en_0(&self) -> bool {
    *self == RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_0
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_BLE_EN_1`"]
  #[inline(always)]
  pub fn is_rsim_cgc_ble_en_1(&self) -> bool {
    *self == RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_1
  }
}
#[doc = "Write proxy for field `RSIM_CGC_BLE_EN`"]
pub struct RSIM_CGC_BLE_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_CGC_BLE_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSIM_CGC_BLE_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn rsim_cgc_ble_en_0(self) -> &'a mut W {
    self.variant(RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_0)
  }
  #[doc = "Clock enabled"]
  #[inline(always)]
  pub fn rsim_cgc_ble_en_1(self) -> &'a mut W {
    self.variant(RSIM_CGC_BLE_EN_A::RSIM_CGC_BLE_EN_1)
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
#[doc = "XCVR Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIM_CGC_XCVR_EN_A {
  #[doc = "0: Clock disabled"]
  RSIM_CGC_XCVR_EN_0,
  #[doc = "1: Clock enabled"]
  RSIM_CGC_XCVR_EN_1,
}
impl From<RSIM_CGC_XCVR_EN_A> for bool {
  #[inline(always)]
  fn from(variant: RSIM_CGC_XCVR_EN_A) -> Self {
    match variant {
      RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_0 => false,
      RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `RSIM_CGC_XCVR_EN`"]
pub type RSIM_CGC_XCVR_EN_R = crate::R<bool, RSIM_CGC_XCVR_EN_A>;
impl RSIM_CGC_XCVR_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSIM_CGC_XCVR_EN_A {
    match self.bits {
      false => RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_0,
      true => RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_XCVR_EN_0`"]
  #[inline(always)]
  pub fn is_rsim_cgc_xcvr_en_0(&self) -> bool {
    *self == RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_0
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_XCVR_EN_1`"]
  #[inline(always)]
  pub fn is_rsim_cgc_xcvr_en_1(&self) -> bool {
    *self == RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_1
  }
}
#[doc = "Write proxy for field `RSIM_CGC_XCVR_EN`"]
pub struct RSIM_CGC_XCVR_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_CGC_XCVR_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSIM_CGC_XCVR_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn rsim_cgc_xcvr_en_0(self) -> &'a mut W {
    self.variant(RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_0)
  }
  #[doc = "Clock enabled"]
  #[inline(always)]
  pub fn rsim_cgc_xcvr_en_1(self) -> &'a mut W {
    self.variant(RSIM_CGC_XCVR_EN_A::RSIM_CGC_XCVR_EN_1)
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
#[doc = "ZIG Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIM_CGC_ZIG_EN_A {
  #[doc = "0: Clock disabled"]
  RSIM_CGC_ZIG_EN_0,
  #[doc = "1: Clock enabled"]
  RSIM_CGC_ZIG_EN_1,
}
impl From<RSIM_CGC_ZIG_EN_A> for bool {
  #[inline(always)]
  fn from(variant: RSIM_CGC_ZIG_EN_A) -> Self {
    match variant {
      RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_0 => false,
      RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `RSIM_CGC_ZIG_EN`"]
pub type RSIM_CGC_ZIG_EN_R = crate::R<bool, RSIM_CGC_ZIG_EN_A>;
impl RSIM_CGC_ZIG_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSIM_CGC_ZIG_EN_A {
    match self.bits {
      false => RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_0,
      true => RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_ZIG_EN_0`"]
  #[inline(always)]
  pub fn is_rsim_cgc_zig_en_0(&self) -> bool {
    *self == RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_0
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_ZIG_EN_1`"]
  #[inline(always)]
  pub fn is_rsim_cgc_zig_en_1(&self) -> bool {
    *self == RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_1
  }
}
#[doc = "Write proxy for field `RSIM_CGC_ZIG_EN`"]
pub struct RSIM_CGC_ZIG_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_CGC_ZIG_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSIM_CGC_ZIG_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn rsim_cgc_zig_en_0(self) -> &'a mut W {
    self.variant(RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_0)
  }
  #[doc = "Clock enabled"]
  #[inline(always)]
  pub fn rsim_cgc_zig_en_1(self) -> &'a mut W {
    self.variant(RSIM_CGC_ZIG_EN_A::RSIM_CGC_ZIG_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "GEN Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIM_CGC_GEN_EN_A {
  #[doc = "0: Clock disabled"]
  RSIM_CGC_GEN_EN_0,
  #[doc = "1: Clock enabled"]
  RSIM_CGC_GEN_EN_1,
}
impl From<RSIM_CGC_GEN_EN_A> for bool {
  #[inline(always)]
  fn from(variant: RSIM_CGC_GEN_EN_A) -> Self {
    match variant {
      RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_0 => false,
      RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `RSIM_CGC_GEN_EN`"]
pub type RSIM_CGC_GEN_EN_R = crate::R<bool, RSIM_CGC_GEN_EN_A>;
impl RSIM_CGC_GEN_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSIM_CGC_GEN_EN_A {
    match self.bits {
      false => RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_0,
      true => RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_GEN_EN_0`"]
  #[inline(always)]
  pub fn is_rsim_cgc_gen_en_0(&self) -> bool {
    *self == RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_0
  }
  #[doc = "Checks if the value of the field is `RSIM_CGC_GEN_EN_1`"]
  #[inline(always)]
  pub fn is_rsim_cgc_gen_en_1(&self) -> bool {
    *self == RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_1
  }
}
#[doc = "Write proxy for field `RSIM_CGC_GEN_EN`"]
pub struct RSIM_CGC_GEN_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_CGC_GEN_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSIM_CGC_GEN_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn rsim_cgc_gen_en_0(self) -> &'a mut W {
    self.variant(RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_0)
  }
  #[doc = "Clock enabled"]
  #[inline(always)]
  pub fn rsim_cgc_gen_en_1(self) -> &'a mut W {
    self.variant(RSIM_CGC_GEN_EN_A::RSIM_CGC_GEN_EN_1)
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
  #[doc = "Bit 0 - BLE RF Power Request Enable"]
  #[inline(always)]
  pub fn ble_rf_power_req_en(&self) -> BLE_RF_POWER_REQ_EN_R {
    BLE_RF_POWER_REQ_EN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - BLE RF Power Request Status"]
  #[inline(always)]
  pub fn ble_rf_power_req_stat(&self) -> BLE_RF_POWER_REQ_STAT_R {
    BLE_RF_POWER_REQ_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 4 - BLE RF Power Request Interrupt Enable"]
  #[inline(always)]
  pub fn ble_rf_power_req_int_en(&self) -> BLE_RF_POWER_REQ_INT_EN_R {
    BLE_RF_POWER_REQ_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - BLE RF Power Request Interrupt Flag"]
  #[inline(always)]
  pub fn ble_rf_power_req_int(&self) -> BLE_RF_POWER_REQ_INT_R {
    BLE_RF_POWER_REQ_INT_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 8 - RF Ref Osc Enable"]
  #[inline(always)]
  pub fn rf_osc_en(&self) -> RF_OSC_EN_R {
    RF_OSC_EN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Radio Gasket Bypass Override Enable"]
  #[inline(always)]
  pub fn radio_gasket_bypass_ovrd_en(&self) -> RADIO_GASKET_BYPASS_OVRD_EN_R {
    RADIO_GASKET_BYPASS_OVRD_EN_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Radio Gasket Bypass Override"]
  #[inline(always)]
  pub fn radio_gasket_bypass_ovrd(&self) -> RADIO_GASKET_BYPASS_OVRD_R {
    RADIO_GASKET_BYPASS_OVRD_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - IPP_OBE_BLE_EARLY_WARNING"]
  #[inline(always)]
  pub fn ipp_obe_ble_early_warning(&self) -> IPP_OBE_BLE_EARLY_WARNING_R {
    IPP_OBE_BLE_EARLY_WARNING_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - IPP_OBE_RF_ACTIVE"]
  #[inline(always)]
  pub fn ipp_obe_rf_active(&self) -> IPP_OBE_RF_ACTIVE_R {
    IPP_OBE_RF_ACTIVE_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - IPP_OBE_RF_OSC_EN"]
  #[inline(always)]
  pub fn ipp_obe_rf_osc_en(&self) -> IPP_OBE_RF_OSC_EN_R {
    IPP_OBE_RF_OSC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 18 - IPP_OBE_RF_STATUS"]
  #[inline(always)]
  pub fn ipp_obe_rf_status(&self) -> IPP_OBE_RF_STATUS_R {
    IPP_OBE_RF_STATUS_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - IPP_OBE_RF_PRIORITY"]
  #[inline(always)]
  pub fn ipp_obe_rf_priority(&self) -> IPP_OBE_RF_PRIORITY_R {
    IPP_OBE_RF_PRIORITY_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 20 - BLE Force Deep Sleep Mode Exit"]
  #[inline(always)]
  pub fn ble_dsm_exit(&self) -> BLE_DSM_EXIT_R {
    BLE_DSM_EXIT_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - Wake on Radio Force Deep Sleep Mode Exit"]
  #[inline(always)]
  pub fn wor_dsm_exit(&self) -> WOR_DSM_EXIT_R {
    WOR_DSM_EXIT_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 24 - RF Ref Osc Ready"]
  #[inline(always)]
  pub fn rf_osc_ready(&self) -> RF_OSC_READY_R {
    RF_OSC_READY_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - RF Ref Osc Ready Override Enable"]
  #[inline(always)]
  pub fn rf_osc_ready_ovrd_en(&self) -> RF_OSC_READY_OVRD_EN_R {
    RF_OSC_READY_OVRD_EN_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - RF Ref Osc Ready Override"]
  #[inline(always)]
  pub fn rf_osc_ready_ovrd(&self) -> RF_OSC_READY_OVRD_R {
    RF_OSC_READY_OVRD_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - BLE Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_ble_en(&self) -> RSIM_CGC_BLE_EN_R {
    RSIM_CGC_BLE_EN_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - XCVR Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_xcvr_en(&self) -> RSIM_CGC_XCVR_EN_R {
    RSIM_CGC_XCVR_EN_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - ZIG Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_zig_en(&self) -> RSIM_CGC_ZIG_EN_R {
    RSIM_CGC_ZIG_EN_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 31 - GEN Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_gen_en(&self) -> RSIM_CGC_GEN_EN_R {
    RSIM_CGC_GEN_EN_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - BLE RF Power Request Enable"]
  #[inline(always)]
  pub fn ble_rf_power_req_en(&mut self) -> BLE_RF_POWER_REQ_EN_W {
    BLE_RF_POWER_REQ_EN_W { w: self }
  }
  #[doc = "Bit 4 - BLE RF Power Request Interrupt Enable"]
  #[inline(always)]
  pub fn ble_rf_power_req_int_en(&mut self) -> BLE_RF_POWER_REQ_INT_EN_W {
    BLE_RF_POWER_REQ_INT_EN_W { w: self }
  }
  #[doc = "Bit 5 - BLE RF Power Request Interrupt Flag"]
  #[inline(always)]
  pub fn ble_rf_power_req_int(&mut self) -> BLE_RF_POWER_REQ_INT_W {
    BLE_RF_POWER_REQ_INT_W { w: self }
  }
  #[doc = "Bit 8 - RF Ref Osc Enable"]
  #[inline(always)]
  pub fn rf_osc_en(&mut self) -> RF_OSC_EN_W {
    RF_OSC_EN_W { w: self }
  }
  #[doc = "Bit 12 - Radio Gasket Bypass Override Enable"]
  #[inline(always)]
  pub fn radio_gasket_bypass_ovrd_en(&mut self) -> RADIO_GASKET_BYPASS_OVRD_EN_W {
    RADIO_GASKET_BYPASS_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 13 - Radio Gasket Bypass Override"]
  #[inline(always)]
  pub fn radio_gasket_bypass_ovrd(&mut self) -> RADIO_GASKET_BYPASS_OVRD_W {
    RADIO_GASKET_BYPASS_OVRD_W { w: self }
  }
  #[doc = "Bit 14 - IPP_OBE_BLE_EARLY_WARNING"]
  #[inline(always)]
  pub fn ipp_obe_ble_early_warning(&mut self) -> IPP_OBE_BLE_EARLY_WARNING_W {
    IPP_OBE_BLE_EARLY_WARNING_W { w: self }
  }
  #[doc = "Bit 15 - IPP_OBE_RF_ACTIVE"]
  #[inline(always)]
  pub fn ipp_obe_rf_active(&mut self) -> IPP_OBE_RF_ACTIVE_W {
    IPP_OBE_RF_ACTIVE_W { w: self }
  }
  #[doc = "Bit 16 - IPP_OBE_RF_OSC_EN"]
  #[inline(always)]
  pub fn ipp_obe_rf_osc_en(&mut self) -> IPP_OBE_RF_OSC_EN_W {
    IPP_OBE_RF_OSC_EN_W { w: self }
  }
  #[doc = "Bit 18 - IPP_OBE_RF_STATUS"]
  #[inline(always)]
  pub fn ipp_obe_rf_status(&mut self) -> IPP_OBE_RF_STATUS_W {
    IPP_OBE_RF_STATUS_W { w: self }
  }
  #[doc = "Bit 19 - IPP_OBE_RF_PRIORITY"]
  #[inline(always)]
  pub fn ipp_obe_rf_priority(&mut self) -> IPP_OBE_RF_PRIORITY_W {
    IPP_OBE_RF_PRIORITY_W { w: self }
  }
  #[doc = "Bit 20 - BLE Force Deep Sleep Mode Exit"]
  #[inline(always)]
  pub fn ble_dsm_exit(&mut self) -> BLE_DSM_EXIT_W {
    BLE_DSM_EXIT_W { w: self }
  }
  #[doc = "Bit 21 - Wake on Radio Force Deep Sleep Mode Exit"]
  #[inline(always)]
  pub fn wor_dsm_exit(&mut self) -> WOR_DSM_EXIT_W {
    WOR_DSM_EXIT_W { w: self }
  }
  #[doc = "Bit 25 - RF Ref Osc Ready Override Enable"]
  #[inline(always)]
  pub fn rf_osc_ready_ovrd_en(&mut self) -> RF_OSC_READY_OVRD_EN_W {
    RF_OSC_READY_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 26 - RF Ref Osc Ready Override"]
  #[inline(always)]
  pub fn rf_osc_ready_ovrd(&mut self) -> RF_OSC_READY_OVRD_W {
    RF_OSC_READY_OVRD_W { w: self }
  }
  #[doc = "Bit 27 - BLE Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_ble_en(&mut self) -> RSIM_CGC_BLE_EN_W {
    RSIM_CGC_BLE_EN_W { w: self }
  }
  #[doc = "Bit 28 - XCVR Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_xcvr_en(&mut self) -> RSIM_CGC_XCVR_EN_W {
    RSIM_CGC_XCVR_EN_W { w: self }
  }
  #[doc = "Bit 29 - ZIG Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_zig_en(&mut self) -> RSIM_CGC_ZIG_EN_W {
    RSIM_CGC_ZIG_EN_W { w: self }
  }
  #[doc = "Bit 31 - GEN Clock Gate Control"]
  #[inline(always)]
  pub fn rsim_cgc_gen_en(&mut self) -> RSIM_CGC_GEN_EN_W {
    RSIM_CGC_GEN_EN_W { w: self }
  }
}
