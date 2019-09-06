#[doc = "Reader of register DCDCSC"]
pub type R = crate::R<u32, super::DCDCSC>;
#[doc = "Writer for register DCDCSC"]
pub type W = crate::W<u32, super::DCDCSC>;
#[doc = "Register DCDCSC `reset()`'s with value 0x0418_0000"]
impl crate::ResetValue for super::DCDCSC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0418_0000
  }
}
#[doc = "Reader of field `DCDC_DISABLE_AUTO_CLK_SWITCH`"]
pub type DCDC_DISABLE_AUTO_CLK_SWITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_DISABLE_AUTO_CLK_SWITCH`"]
pub struct DCDC_DISABLE_AUTO_CLK_SWITCH_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_DISABLE_AUTO_CLK_SWITCH_W<'a> {
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
#[doc = "Reader of field `DCDC_SEL_CLK`"]
pub type DCDC_SEL_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_SEL_CLK`"]
pub struct DCDC_SEL_CLK_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_SEL_CLK_W<'a> {
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
#[doc = "Reader of field `DCDC_PWD_OSC_INT`"]
pub type DCDC_PWD_OSC_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_PWD_OSC_INT`"]
pub struct DCDC_PWD_OSC_INT_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_PWD_OSC_INT_W<'a> {
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
#[doc = "DCDC_VBAT_DIV_CTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VBAT_DIV_CTRL_A {
  #[doc = "0: OFF"]
  DCDC_VBAT_DIV_CTRL_0,
  #[doc = "1: VBAT"]
  DCDC_VBAT_DIV_CTRL_1,
  #[doc = "2: VBAT / 2"]
  DCDC_VBAT_DIV_CTRL_2,
  #[doc = "3: VBAT / 4"]
  DCDC_VBAT_DIV_CTRL_3,
}
impl From<DCDC_VBAT_DIV_CTRL_A> for u8 {
  #[inline(always)]
  fn from(variant: DCDC_VBAT_DIV_CTRL_A) -> Self {
    match variant {
      DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_0 => 0,
      DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_1 => 1,
      DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_2 => 2,
      DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_3 => 3,
    }
  }
}
#[doc = "Reader of field `DCDC_VBAT_DIV_CTRL`"]
pub type DCDC_VBAT_DIV_CTRL_R = crate::R<u8, DCDC_VBAT_DIV_CTRL_A>;
impl DCDC_VBAT_DIV_CTRL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DCDC_VBAT_DIV_CTRL_A {
    match self.bits {
      0 => DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_0,
      1 => DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_1,
      2 => DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_2,
      3 => DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `DCDC_VBAT_DIV_CTRL_0`"]
  #[inline(always)]
  pub fn is_dcdc_vbat_div_ctrl_0(&self) -> bool {
    *self == DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_0
  }
  #[doc = "Checks if the value of the field is `DCDC_VBAT_DIV_CTRL_1`"]
  #[inline(always)]
  pub fn is_dcdc_vbat_div_ctrl_1(&self) -> bool {
    *self == DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_1
  }
  #[doc = "Checks if the value of the field is `DCDC_VBAT_DIV_CTRL_2`"]
  #[inline(always)]
  pub fn is_dcdc_vbat_div_ctrl_2(&self) -> bool {
    *self == DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_2
  }
  #[doc = "Checks if the value of the field is `DCDC_VBAT_DIV_CTRL_3`"]
  #[inline(always)]
  pub fn is_dcdc_vbat_div_ctrl_3(&self) -> bool {
    *self == DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_3
  }
}
#[doc = "Write proxy for field `DCDC_VBAT_DIV_CTRL`"]
pub struct DCDC_VBAT_DIV_CTRL_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_VBAT_DIV_CTRL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DCDC_VBAT_DIV_CTRL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "OFF"]
  #[inline(always)]
  pub fn dcdc_vbat_div_ctrl_0(self) -> &'a mut W {
    self.variant(DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_0)
  }
  #[doc = "VBAT"]
  #[inline(always)]
  pub fn dcdc_vbat_div_ctrl_1(self) -> &'a mut W {
    self.variant(DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_1)
  }
  #[doc = "VBAT / 2"]
  #[inline(always)]
  pub fn dcdc_vbat_div_ctrl_2(self) -> &'a mut W {
    self.variant(DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_2)
  }
  #[doc = "VBAT / 4"]
  #[inline(always)]
  pub fn dcdc_vbat_div_ctrl_3(self) -> &'a mut W {
    self.variant(DCDC_VBAT_DIV_CTRL_A::DCDC_VBAT_DIV_CTRL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
    self.w
  }
}
#[doc = "Reader of field `DCDC_LESS_I`"]
pub type DCDC_LESS_I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_LESS_I`"]
pub struct DCDC_LESS_I_W<'a> {
  w: &'a mut W,
}
impl<'a> DCDC_LESS_I_W<'a> {
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
#[doc = "Reader of field `PWD_CMP_OFFSET`"]
pub type PWD_CMP_OFFSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWD_CMP_OFFSET`"]
pub struct PWD_CMP_OFFSET_W<'a> {
  w: &'a mut W,
}
impl<'a> PWD_CMP_OFFSET_W<'a> {
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
#[doc = "Reader of field `CLKFLT_FAULT`"]
pub type CLKFLT_FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKFLT_FAULT`"]
pub struct CLKFLT_FAULT_W<'a> {
  w: &'a mut W,
}
impl<'a> CLKFLT_FAULT_W<'a> {
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
#[doc = "Reader of field `DCDC_STS_DC_OK`"]
pub type DCDC_STS_DC_OK_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 1 - DCDC_DISABLE_AUTO_CLK_SWITCH"]
  #[inline(always)]
  pub fn dcdc_disable_auto_clk_switch(&self) -> DCDC_DISABLE_AUTO_CLK_SWITCH_R {
    DCDC_DISABLE_AUTO_CLK_SWITCH_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - DCDC_SEL_CLK"]
  #[inline(always)]
  pub fn dcdc_sel_clk(&self) -> DCDC_SEL_CLK_R {
    DCDC_SEL_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - DCDC_PWD_OSC_INT"]
  #[inline(always)]
  pub fn dcdc_pwd_osc_int(&self) -> DCDC_PWD_OSC_INT_R {
    DCDC_PWD_OSC_INT_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 10:11 - DCDC_VBAT_DIV_CTRL"]
  #[inline(always)]
  pub fn dcdc_vbat_div_ctrl(&self) -> DCDC_VBAT_DIV_CTRL_R {
    DCDC_VBAT_DIV_CTRL_R::new(((self.bits >> 10) & 0x03) as u8)
  }
  #[doc = "Bit 25 - DCDC_LESS_I"]
  #[inline(always)]
  pub fn dcdc_less_i(&self) -> DCDC_LESS_I_R {
    DCDC_LESS_I_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - PWD_CMP_OFFSET"]
  #[inline(always)]
  pub fn pwd_cmp_offset(&self) -> PWD_CMP_OFFSET_R {
    PWD_CMP_OFFSET_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 30 - DCDC CLKFLT Fault Status Flag"]
  #[inline(always)]
  pub fn clkflt_fault(&self) -> CLKFLT_FAULT_R {
    CLKFLT_FAULT_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - DCDC_STS_DC_OK"]
  #[inline(always)]
  pub fn dcdc_sts_dc_ok(&self) -> DCDC_STS_DC_OK_R {
    DCDC_STS_DC_OK_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 1 - DCDC_DISABLE_AUTO_CLK_SWITCH"]
  #[inline(always)]
  pub fn dcdc_disable_auto_clk_switch(&mut self) -> DCDC_DISABLE_AUTO_CLK_SWITCH_W {
    DCDC_DISABLE_AUTO_CLK_SWITCH_W { w: self }
  }
  #[doc = "Bit 2 - DCDC_SEL_CLK"]
  #[inline(always)]
  pub fn dcdc_sel_clk(&mut self) -> DCDC_SEL_CLK_W {
    DCDC_SEL_CLK_W { w: self }
  }
  #[doc = "Bit 3 - DCDC_PWD_OSC_INT"]
  #[inline(always)]
  pub fn dcdc_pwd_osc_int(&mut self) -> DCDC_PWD_OSC_INT_W {
    DCDC_PWD_OSC_INT_W { w: self }
  }
  #[doc = "Bits 10:11 - DCDC_VBAT_DIV_CTRL"]
  #[inline(always)]
  pub fn dcdc_vbat_div_ctrl(&mut self) -> DCDC_VBAT_DIV_CTRL_W {
    DCDC_VBAT_DIV_CTRL_W { w: self }
  }
  #[doc = "Bit 25 - DCDC_LESS_I"]
  #[inline(always)]
  pub fn dcdc_less_i(&mut self) -> DCDC_LESS_I_W {
    DCDC_LESS_I_W { w: self }
  }
  #[doc = "Bit 26 - PWD_CMP_OFFSET"]
  #[inline(always)]
  pub fn pwd_cmp_offset(&mut self) -> PWD_CMP_OFFSET_W {
    PWD_CMP_OFFSET_W { w: self }
  }
  #[doc = "Bit 30 - DCDC CLKFLT Fault Status Flag"]
  #[inline(always)]
  pub fn clkflt_fault(&mut self) -> CLKFLT_FAULT_W {
    CLKFLT_FAULT_W { w: self }
  }
}
