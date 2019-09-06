#[doc = "Reader of register DCDCC1"]
pub type R = crate::R<u32, super::DCDCC1>;
#[doc = "Writer for register DCDCC1"]
pub type W = crate::W<u32, super::DCDCC1>;
#[doc = "Register DCDCC1 `reset()`'s with value 0x005f_001c"]
impl crate::ResetValue for super::DCDCC1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x005f_001c
  }
}
#[doc = "Reader of field `POSLIMIT_BUCK_IN`"]
pub type POSLIMIT_BUCK_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSLIMIT_BUCK_IN`"]
pub struct POSLIMIT_BUCK_IN_W<'a> {
  w: &'a mut W,
}
impl<'a> POSLIMIT_BUCK_IN_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
    self.w
  }
}
#[doc = "Reader of field `DCDC_LOOPCTRL_EN_CM_HYST`"]
pub type DCDC_LOOPCTRL_EN_CM_HYST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_LOOPCTRL_EN_CM_HYST`"]
pub struct DCDC_LOOPCTRL_EN_CM_HYST_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_LOOPCTRL_EN_CM_HYST_W<'a> {
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
#[doc = "Reader of field `DCDC_LOOPCTRL_EN_DF_HYST`"]
pub type DCDC_LOOPCTRL_EN_DF_HYST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_LOOPCTRL_EN_DF_HYST`"]
pub struct DCDC_LOOPCTRL_EN_DF_HYST_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_LOOPCTRL_EN_DF_HYST_W<'a> {
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
  #[doc = "Bits 0:6 - POSLIMIT_BUCK_IN"]
  #[inline(always)]
  pub fn poslimit_buck_in(&self) -> POSLIMIT_BUCK_IN_R {
    POSLIMIT_BUCK_IN_R::new((self.bits & 0x7f) as u8)
  }
  #[doc = "Bit 26 - DCDC_LOOPCTRL_EN_CM_HYST"]
  #[inline(always)]
  pub fn dcdc_loopctrl_en_cm_hyst(&self) -> DCDC_LOOPCTRL_EN_CM_HYST_R {
    DCDC_LOOPCTRL_EN_CM_HYST_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - DCDC_LOOPCTRL_EN_DF_HYST"]
  #[inline(always)]
  pub fn dcdc_loopctrl_en_df_hyst(&self) -> DCDC_LOOPCTRL_EN_DF_HYST_R {
    DCDC_LOOPCTRL_EN_DF_HYST_R::new(((self.bits >> 27) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:6 - POSLIMIT_BUCK_IN"]
  #[inline(always)]
  pub fn poslimit_buck_in(&mut self) -> POSLIMIT_BUCK_IN_W {
    POSLIMIT_BUCK_IN_W { w: self }
  }
  #[doc = "Bit 26 - DCDC_LOOPCTRL_EN_CM_HYST"]
  #[inline(always)]
  pub fn dcdc_loopctrl_en_cm_hyst(&mut self) -> DCDC_LOOPCTRL_EN_CM_HYST_W {
    DCDC_LOOPCTRL_EN_CM_HYST_W { w: self }
  }
  #[doc = "Bit 27 - DCDC_LOOPCTRL_EN_DF_HYST"]
  #[inline(always)]
  pub fn dcdc_loopctrl_en_df_hyst(&mut self) -> DCDC_LOOPCTRL_EN_DF_HYST_W {
    DCDC_LOOPCTRL_EN_DF_HYST_W { w: self }
  }
}
