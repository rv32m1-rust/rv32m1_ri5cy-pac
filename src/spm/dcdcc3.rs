#[doc = "Reader of register DCDCC3"]
pub type R = crate::R<u32, super::DCDCC3>;
#[doc = "Writer for register DCDCC3"]
pub type W = crate::W<u32, super::DCDCC3>;
#[doc = "Register DCDCC3 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DCDCC3 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x01
  }
}
#[doc = "Reader of field `DCDC_BYPASS_ADC_MEAS`"]
pub type DCDC_BYPASS_ADC_MEAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_BYPASS_ADC_MEAS`"]
pub struct DCDC_BYPASS_ADC_MEAS_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_BYPASS_ADC_MEAS_W<'a> {
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
#[doc = "Reader of field `DCDC_VBAT_VALUE`"]
pub type DCDC_VBAT_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_VBAT_VALUE`"]
pub struct DCDC_VBAT_VALUE_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VBAT_VALUE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
    self.w
  }
}
#[doc = "Reader of field `DCDC_VDD1P2CTRL_ADJTN`"]
pub type DCDC_VDD1P2CTRL_ADJTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_VDD1P2CTRL_ADJTN`"]
pub struct DCDC_VDD1P2CTRL_ADJTN_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VDD1P2CTRL_ADJTN_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
#[doc = "Reader of field `DCDC_MINPWR_DC_HALFCLK`"]
pub type DCDC_MINPWR_DC_HALFCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_MINPWR_DC_HALFCLK`"]
pub struct DCDC_MINPWR_DC_HALFCLK_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_MINPWR_DC_HALFCLK_W<'a> {
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
#[doc = "Reader of field `DCDC_MINPWR_EXTRA_DOUBLE_FETS`"]
pub type DCDC_MINPWR_EXTRA_DOUBLE_FETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_MINPWR_EXTRA_DOUBLE_FETS`"]
pub struct DCDC_MINPWR_EXTRA_DOUBLE_FETS_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_MINPWR_EXTRA_DOUBLE_FETS_W<'a> {
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
#[doc = "Reader of field `DCDC_MINPWR_DOUBLE_FETS`"]
pub type DCDC_MINPWR_DOUBLE_FETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_MINPWR_DOUBLE_FETS`"]
pub struct DCDC_MINPWR_DOUBLE_FETS_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_MINPWR_DOUBLE_FETS_W<'a> {
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
#[doc = "Reader of field `DCDC_MINPWR_HALF_FETS`"]
pub type DCDC_MINPWR_HALF_FETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_MINPWR_HALF_FETS`"]
pub struct DCDC_MINPWR_HALF_FETS_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_MINPWR_HALF_FETS_W<'a> {
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
#[doc = "Reader of field `DCDC_VDD1P2CTRL_DISABLE_STEP`"]
pub type DCDC_VDD1P2CTRL_DISABLE_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_VDD1P2CTRL_DISABLE_STEP`"]
pub struct DCDC_VDD1P2CTRL_DISABLE_STEP_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VDD1P2CTRL_DISABLE_STEP_W<'a> {
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
#[doc = "Reader of field `DCDC_VDD1P8CTRL_DISABLE_STEP`"]
pub type DCDC_VDD1P8CTRL_DISABLE_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_VDD1P8CTRL_DISABLE_STEP`"]
pub struct DCDC_VDD1P8CTRL_DISABLE_STEP_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VDD1P8CTRL_DISABLE_STEP_W<'a> {
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
  #[doc = "Bit 0 - DCDC_BYPASS_ADC_MEAS"]
  #[inline(always)]
  pub fn dcdc_bypass_adc_meas(&self) -> DCDC_BYPASS_ADC_MEAS_R {
    DCDC_BYPASS_ADC_MEAS_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bits 2:4 - DCDC_VBAT_VALUE"]
  #[inline(always)]
  pub fn dcdc_vbat_value(&self) -> DCDC_VBAT_VALUE_R {
    DCDC_VBAT_VALUE_R::new(((self.bits >> 2) & 0x07) as u8)
  }
  #[doc = "Bits 16:19 - DCDC_VDD1P2CTRL_ADJTN"]
  #[inline(always)]
  pub fn dcdc_vdd1p2ctrl_adjtn(&self) -> DCDC_VDD1P2CTRL_ADJTN_R {
    DCDC_VDD1P2CTRL_ADJTN_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bit 24 - DCDC_MINPWR_DC_HALFCLK"]
  #[inline(always)]
  pub fn dcdc_minpwr_dc_halfclk(&self) -> DCDC_MINPWR_DC_HALFCLK_R {
    DCDC_MINPWR_DC_HALFCLK_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - DCDC_MINPWR_EXTRA_DOUBLE_FETS"]
  #[inline(always)]
  pub fn dcdc_minpwr_extra_double_fets(&self) -> DCDC_MINPWR_EXTRA_DOUBLE_FETS_R {
    DCDC_MINPWR_EXTRA_DOUBLE_FETS_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - DCDC_MINPWR_DOUBLE_FETS"]
  #[inline(always)]
  pub fn dcdc_minpwr_double_fets(&self) -> DCDC_MINPWR_DOUBLE_FETS_R {
    DCDC_MINPWR_DOUBLE_FETS_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - DCDC_MINPWR_HALF_FETS"]
  #[inline(always)]
  pub fn dcdc_minpwr_half_fets(&self) -> DCDC_MINPWR_HALF_FETS_R {
    DCDC_MINPWR_HALF_FETS_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 30 - DCDC_VDD1P2CTRL_DISABLE_STEP"]
  #[inline(always)]
  pub fn dcdc_vdd1p2ctrl_disable_step(&self) -> DCDC_VDD1P2CTRL_DISABLE_STEP_R {
    DCDC_VDD1P2CTRL_DISABLE_STEP_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - DCDC_VDD1P8CTRL_DISABLE_STEP"]
  #[inline(always)]
  pub fn dcdc_vdd1p8ctrl_disable_step(&self) -> DCDC_VDD1P8CTRL_DISABLE_STEP_R {
    DCDC_VDD1P8CTRL_DISABLE_STEP_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - DCDC_BYPASS_ADC_MEAS"]
  #[inline(always)]
  pub fn dcdc_bypass_adc_meas(&mut self) -> DCDC_BYPASS_ADC_MEAS_W {
    DCDC_BYPASS_ADC_MEAS_W { w: self }
  }
  #[doc = "Bits 2:4 - DCDC_VBAT_VALUE"]
  #[inline(always)]
  pub fn dcdc_vbat_value(&mut self) -> DCDC_VBAT_VALUE_W {
    DCDC_VBAT_VALUE_W { w: self }
  }
  #[doc = "Bits 16:19 - DCDC_VDD1P2CTRL_ADJTN"]
  #[inline(always)]
  pub fn dcdc_vdd1p2ctrl_adjtn(&mut self) -> DCDC_VDD1P2CTRL_ADJTN_W {
    DCDC_VDD1P2CTRL_ADJTN_W { w: self }
  }
  #[doc = "Bit 24 - DCDC_MINPWR_DC_HALFCLK"]
  #[inline(always)]
  pub fn dcdc_minpwr_dc_halfclk(&mut self) -> DCDC_MINPWR_DC_HALFCLK_W {
    DCDC_MINPWR_DC_HALFCLK_W { w: self }
  }
  #[doc = "Bit 25 - DCDC_MINPWR_EXTRA_DOUBLE_FETS"]
  #[inline(always)]
  pub fn dcdc_minpwr_extra_double_fets(&mut self) -> DCDC_MINPWR_EXTRA_DOUBLE_FETS_W {
    DCDC_MINPWR_EXTRA_DOUBLE_FETS_W { w: self }
  }
  #[doc = "Bit 26 - DCDC_MINPWR_DOUBLE_FETS"]
  #[inline(always)]
  pub fn dcdc_minpwr_double_fets(&mut self) -> DCDC_MINPWR_DOUBLE_FETS_W {
    DCDC_MINPWR_DOUBLE_FETS_W { w: self }
  }
  #[doc = "Bit 27 - DCDC_MINPWR_HALF_FETS"]
  #[inline(always)]
  pub fn dcdc_minpwr_half_fets(&mut self) -> DCDC_MINPWR_HALF_FETS_W {
    DCDC_MINPWR_HALF_FETS_W { w: self }
  }
  #[doc = "Bit 30 - DCDC_VDD1P2CTRL_DISABLE_STEP"]
  #[inline(always)]
  pub fn dcdc_vdd1p2ctrl_disable_step(&mut self) -> DCDC_VDD1P2CTRL_DISABLE_STEP_W {
    DCDC_VDD1P2CTRL_DISABLE_STEP_W { w: self }
  }
  #[doc = "Bit 31 - DCDC_VDD1P8CTRL_DISABLE_STEP"]
  #[inline(always)]
  pub fn dcdc_vdd1p8ctrl_disable_step(&mut self) -> DCDC_VDD1P8CTRL_DISABLE_STEP_W {
    DCDC_VDD1P8CTRL_DISABLE_STEP_W { w: self }
  }
}
