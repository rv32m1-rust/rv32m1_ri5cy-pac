#[doc = "Reader of register RF_OSC_CTRL"]
pub type R = crate::R<u32, super::RF_OSC_CTRL>;
#[doc = "Writer for register RF_OSC_CTRL"]
pub type W = crate::W<u32, super::RF_OSC_CTRL>;
#[doc = "Register RF_OSC_CTRL `reset()`'s with value 0xa020_3806"]
impl crate::ResetValue for super::RF_OSC_CTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xa020_3806
  }
}
#[doc = "rmap_bb_xtal_alc_count_sel_hv\\[1:0\\]\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_XTAL_ALC_COUNT_SEL_A {
  #[doc = "0: 2048 (64 us @ 32 MHz)"]
  BB_XTAL_ALC_COUNT_SEL_0,
  #[doc = "1: 4096 (128 us @ 32 MHz)"]
  BB_XTAL_ALC_COUNT_SEL_1,
  #[doc = "2: 8192 (256 us @ 32 MHz)"]
  BB_XTAL_ALC_COUNT_SEL_2,
  #[doc = "3: 16384 (512 us @ 32 MHz)"]
  BB_XTAL_ALC_COUNT_SEL_3,
}
impl From<BB_XTAL_ALC_COUNT_SEL_A> for u8 {
  #[inline(always)]
  fn from(variant: BB_XTAL_ALC_COUNT_SEL_A) -> Self {
    match variant {
      BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_0 => 0,
      BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_1 => 1,
      BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_2 => 2,
      BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `BB_XTAL_ALC_COUNT_SEL`"]
pub type BB_XTAL_ALC_COUNT_SEL_R = crate::R<u8, BB_XTAL_ALC_COUNT_SEL_A>;
impl BB_XTAL_ALC_COUNT_SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BB_XTAL_ALC_COUNT_SEL_A {
    match self.bits {
      0 => BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_0,
      1 => BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_1,
      2 => BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_2,
      3 => BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_ALC_COUNT_SEL_0`"]
  #[inline(always)]
  pub fn is_bb_xtal_alc_count_sel_0(&self) -> bool {
    *self == BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_0
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_ALC_COUNT_SEL_1`"]
  #[inline(always)]
  pub fn is_bb_xtal_alc_count_sel_1(&self) -> bool {
    *self == BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_1
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_ALC_COUNT_SEL_2`"]
  #[inline(always)]
  pub fn is_bb_xtal_alc_count_sel_2(&self) -> bool {
    *self == BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_2
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_ALC_COUNT_SEL_3`"]
  #[inline(always)]
  pub fn is_bb_xtal_alc_count_sel_3(&self) -> bool {
    *self == BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_3
  }
}
#[doc = "Write proxy for field `BB_XTAL_ALC_COUNT_SEL`"]
pub struct BB_XTAL_ALC_COUNT_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_ALC_COUNT_SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BB_XTAL_ALC_COUNT_SEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "2048 (64 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_alc_count_sel_0(self) -> &'a mut W {
    self.variant(BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_0)
  }
  #[doc = "4096 (128 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_alc_count_sel_1(self) -> &'a mut W {
    self.variant(BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_1)
  }
  #[doc = "8192 (256 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_alc_count_sel_2(self) -> &'a mut W {
    self.variant(BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_2)
  }
  #[doc = "16384 (512 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_alc_count_sel_3(self) -> &'a mut W {
    self.variant(BB_XTAL_ALC_COUNT_SEL_A::BB_XTAL_ALC_COUNT_SEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Reader of field `BB_XTAL_ALC_ON`"]
pub type BB_XTAL_ALC_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_XTAL_ALC_ON`"]
pub struct BB_XTAL_ALC_ON_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_ALC_ON_W<'a> {
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
#[doc = "Reader of field `RF_OSC_BYPASS_EN`"]
pub type RF_OSC_BYPASS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_OSC_BYPASS_EN`"]
pub struct RF_OSC_BYPASS_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_OSC_BYPASS_EN_W<'a> {
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
#[doc = "Reader of field `BB_XTAL_COMP_BIAS`"]
pub type BB_XTAL_COMP_BIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_XTAL_COMP_BIAS`"]
pub struct BB_XTAL_COMP_BIAS_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_COMP_BIAS_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
    self.w
  }
}
#[doc = "Reader of field `BB_XTAL_DC_COUP_MODE_EN`"]
pub type BB_XTAL_DC_COUP_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_XTAL_DC_COUP_MODE_EN`"]
pub struct BB_XTAL_DC_COUP_MODE_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_DC_COUP_MODE_EN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
#[doc = "Reader of field `BB_XTAL_DIAGSEL`"]
pub type BB_XTAL_DIAGSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_XTAL_DIAGSEL`"]
pub struct BB_XTAL_DIAGSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_DIAGSEL_W<'a> {
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
#[doc = "Reader of field `BB_XTAL_DIG_CLK_ON`"]
pub type BB_XTAL_DIG_CLK_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_XTAL_DIG_CLK_ON`"]
pub struct BB_XTAL_DIG_CLK_ON_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_DIG_CLK_ON_W<'a> {
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
#[doc = "Reader of field `BB_XTAL_GM`"]
pub type BB_XTAL_GM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_XTAL_GM`"]
pub struct BB_XTAL_GM_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_GM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
    self.w
  }
}
#[doc = "Reader of field `BB_XTAL_ON_OVRD`"]
pub type BB_XTAL_ON_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_XTAL_ON_OVRD`"]
pub struct BB_XTAL_ON_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_ON_OVRD_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
#[doc = "rmap_bb_xtal_on_ovrd_on_hv\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_XTAL_ON_OVRD_ON_A {
  #[doc = "0: rfctrl_bb_xtal_on_hv is asserted"]
  BB_XTAL_ON_OVRD_ON_0,
  #[doc = "1: rfctrl_bb_xtal_on_ovrd_hv is asserted"]
  BB_XTAL_ON_OVRD_ON_1,
}
impl From<BB_XTAL_ON_OVRD_ON_A> for bool {
  #[inline(always)]
  fn from(variant: BB_XTAL_ON_OVRD_ON_A) -> Self {
    match variant {
      BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_0 => false,
      BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_1 => true,
    }
  }
}
#[doc = "Reader of field `BB_XTAL_ON_OVRD_ON`"]
pub type BB_XTAL_ON_OVRD_ON_R = crate::R<bool, BB_XTAL_ON_OVRD_ON_A>;
impl BB_XTAL_ON_OVRD_ON_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BB_XTAL_ON_OVRD_ON_A {
    match self.bits {
      false => BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_0,
      true => BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_1,
    }
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_ON_OVRD_ON_0`"]
  #[inline(always)]
  pub fn is_bb_xtal_on_ovrd_on_0(&self) -> bool {
    *self == BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_0
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_ON_OVRD_ON_1`"]
  #[inline(always)]
  pub fn is_bb_xtal_on_ovrd_on_1(&self) -> bool {
    *self == BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_1
  }
}
#[doc = "Write proxy for field `BB_XTAL_ON_OVRD_ON`"]
pub struct BB_XTAL_ON_OVRD_ON_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_ON_OVRD_ON_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BB_XTAL_ON_OVRD_ON_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "rfctrl_bb_xtal_on_hv is asserted"]
  #[inline(always)]
  pub fn bb_xtal_on_ovrd_on_0(self) -> &'a mut W {
    self.variant(BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_0)
  }
  #[doc = "rfctrl_bb_xtal_on_ovrd_hv is asserted"]
  #[inline(always)]
  pub fn bb_xtal_on_ovrd_on_1(self) -> &'a mut W {
    self.variant(BB_XTAL_ON_OVRD_ON_A::BB_XTAL_ON_OVRD_ON_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
#[doc = "rmap_bb_xtal_ready_count_sel_hv\\[1:0\\]\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_XTAL_READY_COUNT_SEL_A {
  #[doc = "0: 1024 counts (32 us @ 32 MHz)"]
  BB_XTAL_READY_COUNT_SEL_0,
  #[doc = "1: 2048 (64 us @ 32 MHz)"]
  BB_XTAL_READY_COUNT_SEL_1,
  #[doc = "2: 4096 (128 us @ 32 MHz)"]
  BB_XTAL_READY_COUNT_SEL_2,
  #[doc = "3: 8192 (256 us @ 32 MHz)"]
  BB_XTAL_READY_COUNT_SEL_3,
}
impl From<BB_XTAL_READY_COUNT_SEL_A> for u8 {
  #[inline(always)]
  fn from(variant: BB_XTAL_READY_COUNT_SEL_A) -> Self {
    match variant {
      BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_0 => 0,
      BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_1 => 1,
      BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_2 => 2,
      BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `BB_XTAL_READY_COUNT_SEL`"]
pub type BB_XTAL_READY_COUNT_SEL_R = crate::R<u8, BB_XTAL_READY_COUNT_SEL_A>;
impl BB_XTAL_READY_COUNT_SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BB_XTAL_READY_COUNT_SEL_A {
    match self.bits {
      0 => BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_0,
      1 => BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_1,
      2 => BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_2,
      3 => BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_READY_COUNT_SEL_0`"]
  #[inline(always)]
  pub fn is_bb_xtal_ready_count_sel_0(&self) -> bool {
    *self == BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_0
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_READY_COUNT_SEL_1`"]
  #[inline(always)]
  pub fn is_bb_xtal_ready_count_sel_1(&self) -> bool {
    *self == BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_1
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_READY_COUNT_SEL_2`"]
  #[inline(always)]
  pub fn is_bb_xtal_ready_count_sel_2(&self) -> bool {
    *self == BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_2
  }
  #[doc = "Checks if the value of the field is `BB_XTAL_READY_COUNT_SEL_3`"]
  #[inline(always)]
  pub fn is_bb_xtal_ready_count_sel_3(&self) -> bool {
    *self == BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_3
  }
}
#[doc = "Write proxy for field `BB_XTAL_READY_COUNT_SEL`"]
pub struct BB_XTAL_READY_COUNT_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_READY_COUNT_SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BB_XTAL_READY_COUNT_SEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "1024 counts (32 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_ready_count_sel_0(self) -> &'a mut W {
    self.variant(BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_0)
  }
  #[doc = "2048 (64 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_ready_count_sel_1(self) -> &'a mut W {
    self.variant(BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_1)
  }
  #[doc = "4096 (128 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_ready_count_sel_2(self) -> &'a mut W {
    self.variant(BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_2)
  }
  #[doc = "8192 (256 us @ 32 MHz)"]
  #[inline(always)]
  pub fn bb_xtal_ready_count_sel_3(self) -> &'a mut W {
    self.variant(BB_XTAL_READY_COUNT_SEL_A::BB_XTAL_READY_COUNT_SEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
    self.w
  }
}
#[doc = "Reader of field `RADIO_EXT_OSC_RF_EN_SEL`"]
pub type RADIO_EXT_OSC_RF_EN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_EXT_OSC_RF_EN_SEL`"]
pub struct RADIO_EXT_OSC_RF_EN_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_EXT_OSC_RF_EN_SEL_W<'a> {
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
#[doc = "Reader of field `RADIO_EXT_OSC_OVRD`"]
pub type RADIO_EXT_OSC_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_EXT_OSC_OVRD`"]
pub struct RADIO_EXT_OSC_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_EXT_OSC_OVRD_W<'a> {
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
#[doc = "Reader of field `RADIO_EXT_OSC_OVRD_EN`"]
pub type RADIO_EXT_OSC_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADIO_EXT_OSC_OVRD_EN`"]
pub struct RADIO_EXT_OSC_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_EXT_OSC_OVRD_EN_W<'a> {
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
#[doc = "Reader of field `RF_NOT_ALLOWED_OVRD`"]
pub type RF_NOT_ALLOWED_OVRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_NOT_ALLOWED_OVRD`"]
pub struct RF_NOT_ALLOWED_OVRD_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_NOT_ALLOWED_OVRD_W<'a> {
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
#[doc = "Reader of field `RF_NOT_ALLOWED_OVRD_EN`"]
pub type RF_NOT_ALLOWED_OVRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF_NOT_ALLOWED_OVRD_EN`"]
pub struct RF_NOT_ALLOWED_OVRD_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RF_NOT_ALLOWED_OVRD_EN_W<'a> {
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
  #[doc = "Bits 0:1 - rmap_bb_xtal_alc_count_sel_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_alc_count_sel(&self) -> BB_XTAL_ALC_COUNT_SEL_R {
    BB_XTAL_ALC_COUNT_SEL_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bit 2 - rmap_bb_xtal_alc_on_hv"]
  #[inline(always)]
  pub fn bb_xtal_alc_on(&self) -> BB_XTAL_ALC_ON_R {
    BB_XTAL_ALC_ON_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - RF Ref Osc Bypass Enable"]
  #[inline(always)]
  pub fn rf_osc_bypass_en(&self) -> RF_OSC_BYPASS_EN_R {
    RF_OSC_BYPASS_EN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 4:8 - rmap_bb_xtal_comp_bias_hv\\[4:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_comp_bias(&self) -> BB_XTAL_COMP_BIAS_R {
    BB_XTAL_COMP_BIAS_R::new(((self.bits >> 4) & 0x1f) as u8)
  }
  #[doc = "Bit 9 - rmap_bb_xtal_dc_coup_mode_en_hv"]
  #[inline(always)]
  pub fn bb_xtal_dc_coup_mode_en(&self) -> BB_XTAL_DC_COUP_MODE_EN_R {
    BB_XTAL_DC_COUP_MODE_EN_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - rmap_bb_xtal_diagsel_hv"]
  #[inline(always)]
  pub fn bb_xtal_diagsel(&self) -> BB_XTAL_DIAGSEL_R {
    BB_XTAL_DIAGSEL_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - rmap_bb_xtal_dig_clk_on_hv"]
  #[inline(always)]
  pub fn bb_xtal_dig_clk_on(&self) -> BB_XTAL_DIG_CLK_ON_R {
    BB_XTAL_DIG_CLK_ON_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bits 12:16 - rmap_bb_xtal_gm_hv\\[4:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_gm(&self) -> BB_XTAL_GM_R {
    BB_XTAL_GM_R::new(((self.bits >> 12) & 0x1f) as u8)
  }
  #[doc = "Bit 17 - rmap_bb_xtal_on_ovrd_hv"]
  #[inline(always)]
  pub fn bb_xtal_on_ovrd(&self) -> BB_XTAL_ON_OVRD_R {
    BB_XTAL_ON_OVRD_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - rmap_bb_xtal_on_ovrd_on_hv"]
  #[inline(always)]
  pub fn bb_xtal_on_ovrd_on(&self) -> BB_XTAL_ON_OVRD_ON_R {
    BB_XTAL_ON_OVRD_ON_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bits 20:21 - rmap_bb_xtal_ready_count_sel_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_ready_count_sel(&self) -> BB_XTAL_READY_COUNT_SEL_R {
    BB_XTAL_READY_COUNT_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bit 27 - Radio External Request for RF OSC Select"]
  #[inline(always)]
  pub fn radio_ext_osc_rf_en_sel(&self) -> RADIO_EXT_OSC_RF_EN_SEL_R {
    RADIO_EXT_OSC_RF_EN_SEL_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Radio External Request for RF OSC Override"]
  #[inline(always)]
  pub fn radio_ext_osc_ovrd(&self) -> RADIO_EXT_OSC_OVRD_R {
    RADIO_EXT_OSC_OVRD_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Radio External Request for RF OSC Override Enable"]
  #[inline(always)]
  pub fn radio_ext_osc_ovrd_en(&self) -> RADIO_EXT_OSC_OVRD_EN_R {
    RADIO_EXT_OSC_OVRD_EN_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - RF Not Allowed Override"]
  #[inline(always)]
  pub fn rf_not_allowed_ovrd(&self) -> RF_NOT_ALLOWED_OVRD_R {
    RF_NOT_ALLOWED_OVRD_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - RF Not Allowed Override Enable"]
  #[inline(always)]
  pub fn rf_not_allowed_ovrd_en(&self) -> RF_NOT_ALLOWED_OVRD_EN_R {
    RF_NOT_ALLOWED_OVRD_EN_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:1 - rmap_bb_xtal_alc_count_sel_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_alc_count_sel(&mut self) -> BB_XTAL_ALC_COUNT_SEL_W {
    BB_XTAL_ALC_COUNT_SEL_W { w: self }
  }
  #[doc = "Bit 2 - rmap_bb_xtal_alc_on_hv"]
  #[inline(always)]
  pub fn bb_xtal_alc_on(&mut self) -> BB_XTAL_ALC_ON_W {
    BB_XTAL_ALC_ON_W { w: self }
  }
  #[doc = "Bit 3 - RF Ref Osc Bypass Enable"]
  #[inline(always)]
  pub fn rf_osc_bypass_en(&mut self) -> RF_OSC_BYPASS_EN_W {
    RF_OSC_BYPASS_EN_W { w: self }
  }
  #[doc = "Bits 4:8 - rmap_bb_xtal_comp_bias_hv\\[4:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_comp_bias(&mut self) -> BB_XTAL_COMP_BIAS_W {
    BB_XTAL_COMP_BIAS_W { w: self }
  }
  #[doc = "Bit 9 - rmap_bb_xtal_dc_coup_mode_en_hv"]
  #[inline(always)]
  pub fn bb_xtal_dc_coup_mode_en(&mut self) -> BB_XTAL_DC_COUP_MODE_EN_W {
    BB_XTAL_DC_COUP_MODE_EN_W { w: self }
  }
  #[doc = "Bit 10 - rmap_bb_xtal_diagsel_hv"]
  #[inline(always)]
  pub fn bb_xtal_diagsel(&mut self) -> BB_XTAL_DIAGSEL_W {
    BB_XTAL_DIAGSEL_W { w: self }
  }
  #[doc = "Bit 11 - rmap_bb_xtal_dig_clk_on_hv"]
  #[inline(always)]
  pub fn bb_xtal_dig_clk_on(&mut self) -> BB_XTAL_DIG_CLK_ON_W {
    BB_XTAL_DIG_CLK_ON_W { w: self }
  }
  #[doc = "Bits 12:16 - rmap_bb_xtal_gm_hv\\[4:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_gm(&mut self) -> BB_XTAL_GM_W {
    BB_XTAL_GM_W { w: self }
  }
  #[doc = "Bit 17 - rmap_bb_xtal_on_ovrd_hv"]
  #[inline(always)]
  pub fn bb_xtal_on_ovrd(&mut self) -> BB_XTAL_ON_OVRD_W {
    BB_XTAL_ON_OVRD_W { w: self }
  }
  #[doc = "Bit 18 - rmap_bb_xtal_on_ovrd_on_hv"]
  #[inline(always)]
  pub fn bb_xtal_on_ovrd_on(&mut self) -> BB_XTAL_ON_OVRD_ON_W {
    BB_XTAL_ON_OVRD_ON_W { w: self }
  }
  #[doc = "Bits 20:21 - rmap_bb_xtal_ready_count_sel_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_ready_count_sel(&mut self) -> BB_XTAL_READY_COUNT_SEL_W {
    BB_XTAL_READY_COUNT_SEL_W { w: self }
  }
  #[doc = "Bit 27 - Radio External Request for RF OSC Select"]
  #[inline(always)]
  pub fn radio_ext_osc_rf_en_sel(&mut self) -> RADIO_EXT_OSC_RF_EN_SEL_W {
    RADIO_EXT_OSC_RF_EN_SEL_W { w: self }
  }
  #[doc = "Bit 28 - Radio External Request for RF OSC Override"]
  #[inline(always)]
  pub fn radio_ext_osc_ovrd(&mut self) -> RADIO_EXT_OSC_OVRD_W {
    RADIO_EXT_OSC_OVRD_W { w: self }
  }
  #[doc = "Bit 29 - Radio External Request for RF OSC Override Enable"]
  #[inline(always)]
  pub fn radio_ext_osc_ovrd_en(&mut self) -> RADIO_EXT_OSC_OVRD_EN_W {
    RADIO_EXT_OSC_OVRD_EN_W { w: self }
  }
  #[doc = "Bit 30 - RF Not Allowed Override"]
  #[inline(always)]
  pub fn rf_not_allowed_ovrd(&mut self) -> RF_NOT_ALLOWED_OVRD_W {
    RF_NOT_ALLOWED_OVRD_W { w: self }
  }
  #[doc = "Bit 31 - RF Not Allowed Override Enable"]
  #[inline(always)]
  pub fn rf_not_allowed_ovrd_en(&mut self) -> RF_NOT_ALLOWED_OVRD_EN_W {
    RF_NOT_ALLOWED_OVRD_EN_W { w: self }
  }
}
