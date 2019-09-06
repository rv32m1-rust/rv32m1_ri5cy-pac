#[doc = "Reader of register DSM_CONTROL"]
pub type R = crate::R<u32, super::DSM_CONTROL>;
#[doc = "Writer for register DSM_CONTROL"]
pub type W = crate::W<u32, super::DSM_CONTROL>;
#[doc = "Register DSM_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::DSM_CONTROL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `DSM_WOR_READY`"]
pub type DSM_WOR_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `WOR_DEEP_SLEEP_STATUS`"]
pub type WOR_DEEP_SLEEP_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSM_WOR_FINISHED`"]
pub type DSM_WOR_FINISHED_R = crate::R<bool, bool>;
#[doc = "Reader of field `WOR_WAKEUP_REQUEST_EN`"]
pub type WOR_WAKEUP_REQUEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOR_WAKEUP_REQUEST_EN`"]
pub struct WOR_WAKEUP_REQUEST_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> WOR_WAKEUP_REQUEST_EN_W<'a> {
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
#[doc = "Reader of field `WOR_SLEEP_REQUEST`"]
pub type WOR_SLEEP_REQUEST_R = crate::R<bool, bool>;
#[doc = "Reader of field `WOR_WAKEUP_REQ`"]
pub type WOR_WAKEUP_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `WOR_WAKEUP_INTERRUPT_EN`"]
pub type WOR_WAKEUP_INTERRUPT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOR_WAKEUP_INTERRUPT_EN`"]
pub struct WOR_WAKEUP_INTERRUPT_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> WOR_WAKEUP_INTERRUPT_EN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "Reader of field `WOR_WAKEUP_REQ_INT`"]
pub type WOR_WAKEUP_REQ_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOR_WAKEUP_REQ_INT`"]
pub struct WOR_WAKEUP_REQ_INT_W<'a> {
  w: &'a mut W,
}
impl<'a> WOR_WAKEUP_REQ_INT_W<'a> {
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
#[doc = "Reader of field `DSM_MAN_READY`"]
pub type DSM_MAN_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAN_DEEP_SLEEP_STATUS`"]
pub type MAN_DEEP_SLEEP_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSM_MAN_FINISHED`"]
pub type DSM_MAN_FINISHED_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAN_WAKEUP_REQUEST_EN`"]
pub type MAN_WAKEUP_REQUEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAN_WAKEUP_REQUEST_EN`"]
pub struct MAN_WAKEUP_REQUEST_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> MAN_WAKEUP_REQUEST_EN_W<'a> {
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
#[doc = "Reader of field `MAN_SLEEP_REQUEST`"]
pub type MAN_SLEEP_REQUEST_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAN_WAKEUP_REQ`"]
pub type MAN_WAKEUP_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAN_WAKEUP_INTERRUPT_EN`"]
pub type MAN_WAKEUP_INTERRUPT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAN_WAKEUP_INTERRUPT_EN`"]
pub struct MAN_WAKEUP_INTERRUPT_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> MAN_WAKEUP_INTERRUPT_EN_W<'a> {
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
#[doc = "Reader of field `MAN_WAKEUP_REQ_INT`"]
pub type MAN_WAKEUP_REQ_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAN_WAKEUP_REQ_INT`"]
pub struct MAN_WAKEUP_REQ_INT_W<'a> {
  w: &'a mut W,
}
impl<'a> MAN_WAKEUP_REQ_INT_W<'a> {
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
#[doc = "Reader of field `WIFI_COEXIST_1`"]
pub type WIFI_COEXIST_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIFI_COEXIST_2`"]
pub type WIFI_COEXIST_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIFI_COEXIST_3`"]
pub type WIFI_COEXIST_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF_ACTIVE_ENDS_WITH_TSM`"]
pub type RF_ACTIVE_ENDS_WITH_TSM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_RF_ACTIVE_ENDS_WITH_TSM`"]
pub type SW_RF_ACTIVE_ENDS_WITH_TSM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_RF_ACTIVE_BIT`"]
pub type SW_RF_ACTIVE_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RF_ACTIVE_BIT`"]
pub struct SW_RF_ACTIVE_BIT_W<'a> {
  w: &'a mut W,
}
impl<'a> SW_RF_ACTIVE_BIT_W<'a> {
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
#[doc = "Reader of field `SW_RF_ACTIVE_EN`"]
pub type SW_RF_ACTIVE_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSM_TIMER_CLR`"]
pub type DSM_TIMER_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_TIMER_CLR`"]
pub struct DSM_TIMER_CLR_W<'a> {
  w: &'a mut W,
}
impl<'a> DSM_TIMER_CLR_W<'a> {
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
#[doc = "Reader of field `DSM_TIMER_EN`"]
pub type DSM_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_TIMER_EN`"]
pub struct DSM_TIMER_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> DSM_TIMER_EN_W<'a> {
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
  #[doc = "Bit 0 - WOR Ready for Deep Sleep Mode"]
  #[inline(always)]
  pub fn dsm_wor_ready(&self) -> DSM_WOR_READY_R {
    DSM_WOR_READY_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - WOR Deep Sleep Mode Status"]
  #[inline(always)]
  pub fn wor_deep_sleep_status(&self) -> WOR_DEEP_SLEEP_STATUS_R {
    WOR_DEEP_SLEEP_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - WOR Deep Sleep Time Finished"]
  #[inline(always)]
  pub fn dsm_wor_finished(&self) -> DSM_WOR_FINISHED_R {
    DSM_WOR_FINISHED_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Enable WOR Deep Sleep Module to initiate a Radio Wakeup"]
  #[inline(always)]
  pub fn wor_wakeup_request_en(&self) -> WOR_WAKEUP_REQUEST_EN_R {
    WOR_WAKEUP_REQUEST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - WOR Deep Sleep Requested"]
  #[inline(always)]
  pub fn wor_sleep_request(&self) -> WOR_SLEEP_REQUEST_R {
    WOR_SLEEP_REQUEST_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - WOR Deep Sleep Module Radio Wakeup Status"]
  #[inline(always)]
  pub fn wor_wakeup_req(&self) -> WOR_WAKEUP_REQ_R {
    WOR_WAKEUP_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - WOR Deep Sleep Module Radio Wakeup Interrupt Enable"]
  #[inline(always)]
  pub fn wor_wakeup_interrupt_en(&self) -> WOR_WAKEUP_INTERRUPT_EN_R {
    WOR_WAKEUP_INTERRUPT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Interrupt Flag from an WOR Deep Sleep Module Radio Wakeup"]
  #[inline(always)]
  pub fn wor_wakeup_req_int(&self) -> WOR_WAKEUP_REQ_INT_R {
    WOR_WAKEUP_REQ_INT_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - MAN Ready for Deep Sleep Mode"]
  #[inline(always)]
  pub fn dsm_man_ready(&self) -> DSM_MAN_READY_R {
    DSM_MAN_READY_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - MAN Deep Sleep Mode Status"]
  #[inline(always)]
  pub fn man_deep_sleep_status(&self) -> MAN_DEEP_SLEEP_STATUS_R {
    MAN_DEEP_SLEEP_STATUS_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - MAN Deep Sleep Time Finished"]
  #[inline(always)]
  pub fn dsm_man_finished(&self) -> DSM_MAN_FINISHED_R {
    DSM_MAN_FINISHED_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Enable MAN Deep Sleep Module to initiate a Radio Wakeup"]
  #[inline(always)]
  pub fn man_wakeup_request_en(&self) -> MAN_WAKEUP_REQUEST_EN_R {
    MAN_WAKEUP_REQUEST_EN_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - MAN Deep Sleep Requested"]
  #[inline(always)]
  pub fn man_sleep_request(&self) -> MAN_SLEEP_REQUEST_R {
    MAN_SLEEP_REQUEST_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - MAN Deep Sleep Module Radio Wakeup Status"]
  #[inline(always)]
  pub fn man_wakeup_req(&self) -> MAN_WAKEUP_REQ_R {
    MAN_WAKEUP_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - MAN Deep Sleep Module Radio Wakeup Interrupt Enable"]
  #[inline(always)]
  pub fn man_wakeup_interrupt_en(&self) -> MAN_WAKEUP_INTERRUPT_EN_R {
    MAN_WAKEUP_INTERRUPT_EN_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Interrupt Flag from an MAN Deep Sleep Module Radio Wakeup"]
  #[inline(always)]
  pub fn man_wakeup_req_int(&self) -> MAN_WAKEUP_REQ_INT_R {
    MAN_WAKEUP_REQ_INT_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - RF_ACTIVE Source"]
  #[inline(always)]
  pub fn wifi_coexist_1(&self) -> WIFI_COEXIST_1_R {
    WIFI_COEXIST_1_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - RF_STATUS Source"]
  #[inline(always)]
  pub fn wifi_coexist_2(&self) -> WIFI_COEXIST_2_R {
    WIFI_COEXIST_2_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - RF_EARLY_WARNING Source"]
  #[inline(always)]
  pub fn wifi_coexist_3(&self) -> WIFI_COEXIST_3_R {
    WIFI_COEXIST_3_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 20 - RF_ACTIVE clearing mechanism"]
  #[inline(always)]
  pub fn rf_active_ends_with_tsm(&self) -> RF_ACTIVE_ENDS_WITH_TSM_R {
    RF_ACTIVE_ENDS_WITH_TSM_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - Software RF_ACTIVE clearing mechanism"]
  #[inline(always)]
  pub fn sw_rf_active_ends_with_tsm(&self) -> SW_RF_ACTIVE_ENDS_WITH_TSM_R {
    SW_RF_ACTIVE_ENDS_WITH_TSM_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 22 - Software RF_ACTIVE Control Bit"]
  #[inline(always)]
  pub fn sw_rf_active_bit(&self) -> SW_RF_ACTIVE_BIT_R {
    SW_RF_ACTIVE_BIT_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Software RF_ACTIVE Control Enable"]
  #[inline(always)]
  pub fn sw_rf_active_en(&self) -> SW_RF_ACTIVE_EN_R {
    SW_RF_ACTIVE_EN_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 27 - Deep Sleep Mode Timer Clear"]
  #[inline(always)]
  pub fn dsm_timer_clr(&self) -> DSM_TIMER_CLR_R {
    DSM_TIMER_CLR_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Deep Sleep Mode Timer Enable"]
  #[inline(always)]
  pub fn dsm_timer_en(&self) -> DSM_TIMER_EN_R {
    DSM_TIMER_EN_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 3 - Enable WOR Deep Sleep Module to initiate a Radio Wakeup"]
  #[inline(always)]
  pub fn wor_wakeup_request_en(&mut self) -> WOR_WAKEUP_REQUEST_EN_W {
    WOR_WAKEUP_REQUEST_EN_W { w: self }
  }
  #[doc = "Bit 6 - WOR Deep Sleep Module Radio Wakeup Interrupt Enable"]
  #[inline(always)]
  pub fn wor_wakeup_interrupt_en(&mut self) -> WOR_WAKEUP_INTERRUPT_EN_W {
    WOR_WAKEUP_INTERRUPT_EN_W { w: self }
  }
  #[doc = "Bit 7 - Interrupt Flag from an WOR Deep Sleep Module Radio Wakeup"]
  #[inline(always)]
  pub fn wor_wakeup_req_int(&mut self) -> WOR_WAKEUP_REQ_INT_W {
    WOR_WAKEUP_REQ_INT_W { w: self }
  }
  #[doc = "Bit 11 - Enable MAN Deep Sleep Module to initiate a Radio Wakeup"]
  #[inline(always)]
  pub fn man_wakeup_request_en(&mut self) -> MAN_WAKEUP_REQUEST_EN_W {
    MAN_WAKEUP_REQUEST_EN_W { w: self }
  }
  #[doc = "Bit 14 - MAN Deep Sleep Module Radio Wakeup Interrupt Enable"]
  #[inline(always)]
  pub fn man_wakeup_interrupt_en(&mut self) -> MAN_WAKEUP_INTERRUPT_EN_W {
    MAN_WAKEUP_INTERRUPT_EN_W { w: self }
  }
  #[doc = "Bit 15 - Interrupt Flag from an MAN Deep Sleep Module Radio Wakeup"]
  #[inline(always)]
  pub fn man_wakeup_req_int(&mut self) -> MAN_WAKEUP_REQ_INT_W {
    MAN_WAKEUP_REQ_INT_W { w: self }
  }
  #[doc = "Bit 22 - Software RF_ACTIVE Control Bit"]
  #[inline(always)]
  pub fn sw_rf_active_bit(&mut self) -> SW_RF_ACTIVE_BIT_W {
    SW_RF_ACTIVE_BIT_W { w: self }
  }
  #[doc = "Bit 27 - Deep Sleep Mode Timer Clear"]
  #[inline(always)]
  pub fn dsm_timer_clr(&mut self) -> DSM_TIMER_CLR_W {
    DSM_TIMER_CLR_W { w: self }
  }
  #[doc = "Bit 31 - Deep Sleep Mode Timer Enable"]
  #[inline(always)]
  pub fn dsm_timer_en(&mut self) -> DSM_TIMER_EN_W {
    DSM_TIMER_EN_W { w: self }
  }
}
