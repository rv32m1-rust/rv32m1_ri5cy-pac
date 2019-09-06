#[doc = "Reader of register DCDCC6"]
pub type R = crate::R<u32, super::DCDCC6>;
#[doc = "Writer for register DCDCC6"]
pub type W = crate::W<u32, super::DCDCC6>;
#[doc = "Register DCDCC6 `reset()`'s with value 0x0d00_0608"]
impl crate::ResetValue for super::DCDCC6 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0d00_0608
  }
}
#[doc = "Reader of field `DCDC_VDD1P8CTRL_TRG`"]
pub type DCDC_VDD1P8CTRL_TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_VDD1P8CTRL_TRG`"]
pub struct DCDC_VDD1P8CTRL_TRG_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VDD1P8CTRL_TRG_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
    self.w
  }
}
#[doc = "Reader of field `DCDC_VDD1P2CTRL_TRG_BUCK`"]
pub type DCDC_VDD1P2CTRL_TRG_BUCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_VDD1P2CTRL_TRG_BUCK`"]
pub struct DCDC_VDD1P2CTRL_TRG_BUCK_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VDD1P2CTRL_TRG_BUCK_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
    self.w
  }
}
#[doc = "Reader of field `DCDC_HSVDD_TRIM`"]
pub type DCDC_HSVDD_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_HSVDD_TRIM`"]
pub struct DCDC_HSVDD_TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_HSVDD_TRIM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:4 - DCDC_VDD1P8CTRL_TRG"]
  #[inline(always)]
  pub fn dcdc_vdd1p8ctrl_trg(&self) -> DCDC_VDD1P8CTRL_TRG_R {
    DCDC_VDD1P8CTRL_TRG_R::new((self.bits & 0x1f) as u8)
  }
  #[doc = "Bits 8:11 - DCDC_VDD1P2CTRL_TRG_BUCK"]
  #[inline(always)]
  pub fn dcdc_vdd1p2ctrl_trg_buck(&self) -> DCDC_VDD1P2CTRL_TRG_BUCK_R {
    DCDC_VDD1P2CTRL_TRG_BUCK_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - DCDC_HSVDD_TRIM"]
  #[inline(always)]
  pub fn dcdc_hsvdd_trim(&self) -> DCDC_HSVDD_TRIM_R {
    DCDC_HSVDD_TRIM_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:4 - DCDC_VDD1P8CTRL_TRG"]
  #[inline(always)]
  pub fn dcdc_vdd1p8ctrl_trg(&mut self) -> DCDC_VDD1P8CTRL_TRG_W {
    DCDC_VDD1P8CTRL_TRG_W { w: self }
  }
  #[doc = "Bits 8:11 - DCDC_VDD1P2CTRL_TRG_BUCK"]
  #[inline(always)]
  pub fn dcdc_vdd1p2ctrl_trg_buck(&mut self) -> DCDC_VDD1P2CTRL_TRG_BUCK_W {
    DCDC_VDD1P2CTRL_TRG_BUCK_W { w: self }
  }
  #[doc = "Bits 24:27 - DCDC_HSVDD_TRIM"]
  #[inline(always)]
  pub fn dcdc_hsvdd_trim(&mut self) -> DCDC_HSVDD_TRIM_W {
    DCDC_HSVDD_TRIM_W { w: self }
  }
}
