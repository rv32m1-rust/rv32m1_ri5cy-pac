#[doc = "Reader of register DCDCC2"]
pub type R = crate::R<u32, super::DCDCC2>;
#[doc = "Writer for register DCDCC2"]
pub type W = crate::W<u32, super::DCDCC2>;
#[doc = "Register DCDCC2 `reset()`'s with value 0x4009"]
impl crate::ResetValue for super::DCDCC2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x4009
  }
}
#[doc = "Reader of field `DCDC_LOOPCTRL_HYST_SIGN`"]
pub type DCDC_LOOPCTRL_HYST_SIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_LOOPCTRL_HYST_SIGN`"]
pub struct DCDC_LOOPCTRL_HYST_SIGN_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_LOOPCTRL_HYST_SIGN_W<'a> {
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
#[doc = "Reader of field `DCDC_BATTMONITOR_EN_BATADJ`"]
pub type DCDC_BATTMONITOR_EN_BATADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_BATTMONITOR_EN_BATADJ`"]
pub struct DCDC_BATTMONITOR_EN_BATADJ_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_BATTMONITOR_EN_BATADJ_W<'a> {
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
#[doc = "Reader of field `DCDC_BATTMONITOR_BATT_VAL`"]
pub type DCDC_BATTMONITOR_BATT_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DCDC_BATTMONITOR_BATT_VAL`"]
pub struct DCDC_BATTMONITOR_BATT_VAL_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_BATTMONITOR_BATT_VAL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 13 - DCDC_LOOPCTRL_HYST_SIGN"]
  #[inline(always)]
  pub fn dcdc_loopctrl_hyst_sign(&self) -> DCDC_LOOPCTRL_HYST_SIGN_R {
    DCDC_LOOPCTRL_HYST_SIGN_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 15 - DCDC_BATTMONITOR_EN_BATADJ"]
  #[inline(always)]
  pub fn dcdc_battmonitor_en_batadj(&self) -> DCDC_BATTMONITOR_EN_BATADJ_R {
    DCDC_BATTMONITOR_EN_BATADJ_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bits 16:25 - DCDC_BATTMONITOR_BATT_VAL"]
  #[inline(always)]
  pub fn dcdc_battmonitor_batt_val(&self) -> DCDC_BATTMONITOR_BATT_VAL_R {
    DCDC_BATTMONITOR_BATT_VAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
  }
}
impl W {
  #[doc = "Bit 13 - DCDC_LOOPCTRL_HYST_SIGN"]
  #[inline(always)]
  pub fn dcdc_loopctrl_hyst_sign(&mut self) -> DCDC_LOOPCTRL_HYST_SIGN_W {
    DCDC_LOOPCTRL_HYST_SIGN_W { w: self }
  }
  #[doc = "Bit 15 - DCDC_BATTMONITOR_EN_BATADJ"]
  #[inline(always)]
  pub fn dcdc_battmonitor_en_batadj(&mut self) -> DCDC_BATTMONITOR_EN_BATADJ_W {
    DCDC_BATTMONITOR_EN_BATADJ_W { w: self }
  }
  #[doc = "Bits 16:25 - DCDC_BATTMONITOR_BATT_VAL"]
  #[inline(always)]
  pub fn dcdc_battmonitor_batt_val(&mut self) -> DCDC_BATTMONITOR_BATT_VAL_W {
    DCDC_BATTMONITOR_BATT_VAL_W { w: self }
  }
}
