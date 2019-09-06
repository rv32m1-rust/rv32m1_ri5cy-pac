#[doc = "Reader of register POWER"]
pub type R = crate::R<u32, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u32, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0x7000_0307"]
impl crate::ResetValue for super::POWER {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x7000_0307
  }
}
#[doc = "Reader of field `RADIO_STOP_MODE_STAT`"]
pub type RADIO_STOP_MODE_STAT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPM_STOP_ACK_STAT`"]
pub type SPM_STOP_ACK_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RADIO_STOP_MODE_OVRD`"]
pub type RADIO_STOP_MODE_OVRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RADIO_STOP_MODE_OVRD`"]
pub struct RADIO_STOP_MODE_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_STOP_MODE_OVRD_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
    self.w
  }
}
#[doc = "Reader of field `RADIO_STOP_MODE_OVRD_EN`"]
pub type RADIO_STOP_MODE_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_STOP_MODE_OVRD_EN`"]
pub struct RADIO_STOP_MODE_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_STOP_MODE_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `RADIO_STOP_ACK_STAT`"]
pub type RADIO_STOP_ACK_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RADIO_STOP_REQ_STAT`"]
pub type RADIO_STOP_REQ_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSIM_STOP_REQ_OVRD`"]
pub type RSIM_STOP_REQ_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_STOP_REQ_OVRD`"]
pub struct RSIM_STOP_REQ_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_STOP_REQ_OVRD_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
    self.w
  }
}
#[doc = "Reader of field `RSIM_STOP_REQ_OVRD_EN`"]
pub type RSIM_STOP_REQ_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_STOP_REQ_OVRD_EN`"]
pub struct RSIM_STOP_REQ_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_STOP_REQ_OVRD_EN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
    self.w
  }
}
#[doc = "Reader of field `RF_OSC_EN_OVRD`"]
pub type RF_OSC_EN_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_OSC_EN_OVRD`"]
pub struct RF_OSC_EN_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_OSC_EN_OVRD_W<'a> {
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
#[doc = "Reader of field `RF_OSC_EN_OVRD_EN`"]
pub type RF_OSC_EN_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_OSC_EN_OVRD_EN`"]
pub struct RF_OSC_EN_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_OSC_EN_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `RF_POWER_EN_OVRD`"]
pub type RF_POWER_EN_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_POWER_EN_OVRD`"]
pub struct RF_POWER_EN_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_POWER_EN_OVRD_W<'a> {
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
#[doc = "Reader of field `RF_POWER_EN_OVRD_EN`"]
pub type RF_POWER_EN_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_POWER_EN_OVRD_EN`"]
pub struct RF_POWER_EN_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_POWER_EN_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `SPM_ISO_STAT`"]
pub type SPM_ISO_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RADIO_ISO_STAT`"]
pub type RADIO_ISO_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSIM_ISO_OVRD`"]
pub type RSIM_ISO_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_ISO_OVRD`"]
pub struct RSIM_ISO_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_ISO_OVRD_W<'a> {
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
#[doc = "Reader of field `RSIM_ISO_OVRD_EN`"]
pub type RSIM_ISO_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_ISO_OVRD_EN`"]
pub struct RSIM_ISO_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_ISO_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `SPM_RUN_ACK_STAT`"]
pub type SPM_RUN_ACK_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RADIO_RUN_REQ_STAT`"]
pub type RADIO_RUN_REQ_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSIM_RUN_REQ_OVRD`"]
pub type RSIM_RUN_REQ_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_RUN_REQ_OVRD`"]
pub struct RSIM_RUN_REQ_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_RUN_REQ_OVRD_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
    self.w
  }
}
#[doc = "Reader of field `RSIM_RUN_REQ_OVRD_EN`"]
pub type RSIM_RUN_REQ_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_RUN_REQ_OVRD_EN`"]
pub struct RSIM_RUN_REQ_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_RUN_REQ_OVRD_EN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "Reader of field `SPM_STOP_REQ_ACK_OVRD`"]
pub type SPM_STOP_REQ_ACK_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPM_STOP_REQ_ACK_OVRD`"]
pub struct SPM_STOP_REQ_ACK_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> SPM_STOP_REQ_ACK_OVRD_W<'a> {
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
#[doc = "Reader of field `SPM_STOP_REQ_ACK_OVRD_EN`"]
pub type SPM_STOP_REQ_ACK_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPM_STOP_REQ_ACK_OVRD_EN`"]
pub struct SPM_STOP_REQ_ACK_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> SPM_STOP_REQ_ACK_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `SPM_RUN_REQ_ACK_OVRD`"]
pub type SPM_RUN_REQ_ACK_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPM_RUN_REQ_ACK_OVRD`"]
pub struct SPM_RUN_REQ_ACK_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> SPM_RUN_REQ_ACK_OVRD_W<'a> {
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
#[doc = "Reader of field `SPM_RUN_REQ_ACK_OVRD_EN`"]
pub type SPM_RUN_REQ_ACK_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPM_RUN_REQ_ACK_OVRD_EN`"]
pub struct SPM_RUN_REQ_ACK_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> SPM_RUN_REQ_ACK_OVRD_EN_W<'a> {
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
#[doc = "RSIM lowest allowed Stop Mode\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIM_STOP_MODE_A {
  #[doc = "3: RLLS mode (Radio State Retention mode)"]
  RSIM_STOP_MODE_3,
  #[doc = "7: RVLLS mode (This is the POR setting)"]
  RSIM_STOP_MODE_7,
}
impl From<RSIM_STOP_MODE_A> for u8 {
  #[inline(always)]
  fn from(variant: RSIM_STOP_MODE_A) -> Self {
    match variant {
      RSIM_STOP_MODE_A::RSIM_STOP_MODE_3 => 3,
      RSIM_STOP_MODE_A::RSIM_STOP_MODE_7 => 7,
    }
  }
}
#[doc = "Reader of field `RSIM_STOP_MODE`"]
pub type RSIM_STOP_MODE_R = crate::R<u8, RSIM_STOP_MODE_A>;
impl RSIM_STOP_MODE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RSIM_STOP_MODE_A> {
    use crate::Variant::*;
    match self.bits {
      3 => Val(RSIM_STOP_MODE_A::RSIM_STOP_MODE_3),
      7 => Val(RSIM_STOP_MODE_A::RSIM_STOP_MODE_7),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RSIM_STOP_MODE_3`"]
  #[inline(always)]
  pub fn is_rsim_stop_mode_3(&self) -> bool {
    *self == RSIM_STOP_MODE_A::RSIM_STOP_MODE_3
  }
  #[doc = "Checks if the value of the field is `RSIM_STOP_MODE_7`"]
  #[inline(always)]
  pub fn is_rsim_stop_mode_7(&self) -> bool {
    *self == RSIM_STOP_MODE_A::RSIM_STOP_MODE_7
  }
}
#[doc = "Write proxy for field `RSIM_STOP_MODE`"]
pub struct RSIM_STOP_MODE_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_STOP_MODE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSIM_STOP_MODE_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "RLLS mode (Radio State Retention mode)"]
  #[inline(always)]
  pub fn rsim_stop_mode_3(self) -> &'a mut W {
    self.variant(RSIM_STOP_MODE_A::RSIM_STOP_MODE_3)
  }
  #[doc = "RVLLS mode (This is the POR setting)"]
  #[inline(always)]
  pub fn rsim_stop_mode_7(self) -> &'a mut W {
    self.variant(RSIM_STOP_MODE_A::RSIM_STOP_MODE_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
    self.w
  }
}
#[doc = "Reader of field `RSIM_RUN_REQUEST`"]
pub type RSIM_RUN_REQUEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSIM_RUN_REQUEST`"]
pub struct RSIM_RUN_REQUEST_W<'a> {
  w: &'a mut W,
}
impl<'a> RSIM_RUN_REQUEST_W<'a> {
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
  #[doc = "Bits 0:2 - Radio Stop Mode Status"]
  #[inline(always)]
  pub fn radio_stop_mode_stat(&self) -> RADIO_STOP_MODE_STAT_R {
    RADIO_STOP_MODE_STAT_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bit 3 - SPM Stop Acknowledge Status"]
  #[inline(always)]
  pub fn spm_stop_ack_stat(&self) -> SPM_STOP_ACK_STAT_R {
    SPM_STOP_ACK_STAT_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 4:6 - Radio Stop Mode Override"]
  #[inline(always)]
  pub fn radio_stop_mode_ovrd(&self) -> RADIO_STOP_MODE_OVRD_R {
    RADIO_STOP_MODE_OVRD_R::new(((self.bits >> 4) & 0x07) as u8)
  }
  #[doc = "Bit 7 - Radio Stop Mode Override Enable"]
  #[inline(always)]
  pub fn radio_stop_mode_ovrd_en(&self) -> RADIO_STOP_MODE_OVRD_EN_R {
    RADIO_STOP_MODE_OVRD_EN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Radio Stop Acknowledge Status"]
  #[inline(always)]
  pub fn radio_stop_ack_stat(&self) -> RADIO_STOP_ACK_STAT_R {
    RADIO_STOP_ACK_STAT_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Radio Stop Request Status"]
  #[inline(always)]
  pub fn radio_stop_req_stat(&self) -> RADIO_STOP_REQ_STAT_R {
    RADIO_STOP_REQ_STAT_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Radio Stop Request Override"]
  #[inline(always)]
  pub fn rsim_stop_req_ovrd(&self) -> RSIM_STOP_REQ_OVRD_R {
    RSIM_STOP_REQ_OVRD_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Radio Stop Request Override Enable"]
  #[inline(always)]
  pub fn rsim_stop_req_ovrd_en(&self) -> RSIM_STOP_REQ_OVRD_EN_R {
    RSIM_STOP_REQ_OVRD_EN_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Radio Osc Enable Override"]
  #[inline(always)]
  pub fn rf_osc_en_ovrd(&self) -> RF_OSC_EN_OVRD_R {
    RF_OSC_EN_OVRD_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Radio Osc Enable Override Enable"]
  #[inline(always)]
  pub fn rf_osc_en_ovrd_en(&self) -> RF_OSC_EN_OVRD_EN_R {
    RF_OSC_EN_OVRD_EN_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Radio Power Enable Override"]
  #[inline(always)]
  pub fn rf_power_en_ovrd(&self) -> RF_POWER_EN_OVRD_R {
    RF_POWER_EN_OVRD_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Radio Power Enable Override Enable"]
  #[inline(always)]
  pub fn rf_power_en_ovrd_en(&self) -> RF_POWER_EN_OVRD_EN_R {
    RF_POWER_EN_OVRD_EN_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - SPM ISO Status"]
  #[inline(always)]
  pub fn spm_iso_stat(&self) -> SPM_ISO_STAT_R {
    SPM_ISO_STAT_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Radio Isolation Status"]
  #[inline(always)]
  pub fn radio_iso_stat(&self) -> RADIO_ISO_STAT_R {
    RADIO_ISO_STAT_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - RSIM ISO Override"]
  #[inline(always)]
  pub fn rsim_iso_ovrd(&self) -> RSIM_ISO_OVRD_R {
    RSIM_ISO_OVRD_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - RSIM ISO Override Enable"]
  #[inline(always)]
  pub fn rsim_iso_ovrd_en(&self) -> RSIM_ISO_OVRD_EN_R {
    RSIM_ISO_OVRD_EN_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 20 - SPM Run Request Acknowledge Status"]
  #[inline(always)]
  pub fn spm_run_ack_stat(&self) -> SPM_RUN_ACK_STAT_R {
    SPM_RUN_ACK_STAT_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - Radio Run Request Status"]
  #[inline(always)]
  pub fn radio_run_req_stat(&self) -> RADIO_RUN_REQ_STAT_R {
    RADIO_RUN_REQ_STAT_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 22 - RSIM Run Request Override"]
  #[inline(always)]
  pub fn rsim_run_req_ovrd(&self) -> RSIM_RUN_REQ_OVRD_R {
    RSIM_RUN_REQ_OVRD_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - RSIM Run Request Override Enable"]
  #[inline(always)]
  pub fn rsim_run_req_ovrd_en(&self) -> RSIM_RUN_REQ_OVRD_EN_R {
    RSIM_RUN_REQ_OVRD_EN_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - SPM Stop Request Acknowledge Override"]
  #[inline(always)]
  pub fn spm_stop_req_ack_ovrd(&self) -> SPM_STOP_REQ_ACK_OVRD_R {
    SPM_STOP_REQ_ACK_OVRD_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - SPM Stop Request Acknowledge Override Enable"]
  #[inline(always)]
  pub fn spm_stop_req_ack_ovrd_en(&self) -> SPM_STOP_REQ_ACK_OVRD_EN_R {
    SPM_STOP_REQ_ACK_OVRD_EN_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - SPM Run Request Acknowledge Override"]
  #[inline(always)]
  pub fn spm_run_req_ack_ovrd(&self) -> SPM_RUN_REQ_ACK_OVRD_R {
    SPM_RUN_REQ_ACK_OVRD_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - SPM Run Request Acknowledge Override Enable"]
  #[inline(always)]
  pub fn spm_run_req_ack_ovrd_en(&self) -> SPM_RUN_REQ_ACK_OVRD_EN_R {
    SPM_RUN_REQ_ACK_OVRD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bits 28:30 - RSIM lowest allowed Stop Mode"]
  #[inline(always)]
  pub fn rsim_stop_mode(&self) -> RSIM_STOP_MODE_R {
    RSIM_STOP_MODE_R::new(((self.bits >> 28) & 0x07) as u8)
  }
  #[doc = "Bit 31 - RSIM Run Regulator Request"]
  #[inline(always)]
  pub fn rsim_run_request(&self) -> RSIM_RUN_REQUEST_R {
    RSIM_RUN_REQUEST_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 4:6 - Radio Stop Mode Override"]
  #[inline(always)]
  pub fn radio_stop_mode_ovrd(&mut self) -> RADIO_STOP_MODE_OVRD_W {
    RADIO_STOP_MODE_OVRD_W { w: self }
  }
  #[doc = "Bit 7 - Radio Stop Mode Override Enable"]
  #[inline(always)]
  pub fn radio_stop_mode_ovrd_en(&mut self) -> RADIO_STOP_MODE_OVRD_EN_W {
    RADIO_STOP_MODE_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 10 - Radio Stop Request Override"]
  #[inline(always)]
  pub fn rsim_stop_req_ovrd(&mut self) -> RSIM_STOP_REQ_OVRD_W {
    RSIM_STOP_REQ_OVRD_W { w: self }
  }
  #[doc = "Bit 11 - Radio Stop Request Override Enable"]
  #[inline(always)]
  pub fn rsim_stop_req_ovrd_en(&mut self) -> RSIM_STOP_REQ_OVRD_EN_W {
    RSIM_STOP_REQ_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 12 - Radio Osc Enable Override"]
  #[inline(always)]
  pub fn rf_osc_en_ovrd(&mut self) -> RF_OSC_EN_OVRD_W {
    RF_OSC_EN_OVRD_W { w: self }
  }
  #[doc = "Bit 13 - Radio Osc Enable Override Enable"]
  #[inline(always)]
  pub fn rf_osc_en_ovrd_en(&mut self) -> RF_OSC_EN_OVRD_EN_W {
    RF_OSC_EN_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 14 - Radio Power Enable Override"]
  #[inline(always)]
  pub fn rf_power_en_ovrd(&mut self) -> RF_POWER_EN_OVRD_W {
    RF_POWER_EN_OVRD_W { w: self }
  }
  #[doc = "Bit 15 - Radio Power Enable Override Enable"]
  #[inline(always)]
  pub fn rf_power_en_ovrd_en(&mut self) -> RF_POWER_EN_OVRD_EN_W {
    RF_POWER_EN_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 18 - RSIM ISO Override"]
  #[inline(always)]
  pub fn rsim_iso_ovrd(&mut self) -> RSIM_ISO_OVRD_W {
    RSIM_ISO_OVRD_W { w: self }
  }
  #[doc = "Bit 19 - RSIM ISO Override Enable"]
  #[inline(always)]
  pub fn rsim_iso_ovrd_en(&mut self) -> RSIM_ISO_OVRD_EN_W {
    RSIM_ISO_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 22 - RSIM Run Request Override"]
  #[inline(always)]
  pub fn rsim_run_req_ovrd(&mut self) -> RSIM_RUN_REQ_OVRD_W {
    RSIM_RUN_REQ_OVRD_W { w: self }
  }
  #[doc = "Bit 23 - RSIM Run Request Override Enable"]
  #[inline(always)]
  pub fn rsim_run_req_ovrd_en(&mut self) -> RSIM_RUN_REQ_OVRD_EN_W {
    RSIM_RUN_REQ_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 24 - SPM Stop Request Acknowledge Override"]
  #[inline(always)]
  pub fn spm_stop_req_ack_ovrd(&mut self) -> SPM_STOP_REQ_ACK_OVRD_W {
    SPM_STOP_REQ_ACK_OVRD_W { w: self }
  }
  #[doc = "Bit 25 - SPM Stop Request Acknowledge Override Enable"]
  #[inline(always)]
  pub fn spm_stop_req_ack_ovrd_en(&mut self) -> SPM_STOP_REQ_ACK_OVRD_EN_W {
    SPM_STOP_REQ_ACK_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 26 - SPM Run Request Acknowledge Override"]
  #[inline(always)]
  pub fn spm_run_req_ack_ovrd(&mut self) -> SPM_RUN_REQ_ACK_OVRD_W {
    SPM_RUN_REQ_ACK_OVRD_W { w: self }
  }
  #[doc = "Bit 27 - SPM Run Request Acknowledge Override Enable"]
  #[inline(always)]
  pub fn spm_run_req_ack_ovrd_en(&mut self) -> SPM_RUN_REQ_ACK_OVRD_EN_W {
    SPM_RUN_REQ_ACK_OVRD_EN_W { w: self }
  }
  #[doc = "Bits 28:30 - RSIM lowest allowed Stop Mode"]
  #[inline(always)]
  pub fn rsim_stop_mode(&mut self) -> RSIM_STOP_MODE_W {
    RSIM_STOP_MODE_W { w: self }
  }
  #[doc = "Bit 31 - RSIM Run Regulator Request"]
  #[inline(always)]
  pub fn rsim_run_request(&mut self) -> RSIM_RUN_REQUEST_W {
    RSIM_RUN_REQUEST_W { w: self }
  }
}
