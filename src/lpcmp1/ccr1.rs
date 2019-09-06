#[doc = "Reader of register CCR1"]
pub type R = crate::R<u32, super::CCR1>;
#[doc = "Writer for register CCR1"]
pub type W = crate::W<u32, super::CCR1>;
#[doc = "Register CCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINDOW_EN_A {
  #[doc = "0: Windowing mode is not selected."]
  WINDOW_EN_0,
  #[doc = "1: Windowing mode is selected."]
  WINDOW_EN_1,
}
impl From<WINDOW_EN_A> for bool {
  #[inline(always)]
  fn from(variant: WINDOW_EN_A) -> Self {
    match variant {
      WINDOW_EN_A::WINDOW_EN_0 => false,
      WINDOW_EN_A::WINDOW_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `WINDOW_EN`"]
pub type WINDOW_EN_R = crate::R<bool, WINDOW_EN_A>;
impl WINDOW_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WINDOW_EN_A {
    match self.bits {
      false => WINDOW_EN_A::WINDOW_EN_0,
      true => WINDOW_EN_A::WINDOW_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `WINDOW_EN_0`"]
  #[inline(always)]
  pub fn is_window_en_0(&self) -> bool {
    *self == WINDOW_EN_A::WINDOW_EN_0
  }
  #[doc = "Checks if the value of the field is `WINDOW_EN_1`"]
  #[inline(always)]
  pub fn is_window_en_1(&self) -> bool {
    *self == WINDOW_EN_A::WINDOW_EN_1
  }
}
#[doc = "Write proxy for field `WINDOW_EN`"]
pub struct WINDOW_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> WINDOW_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WINDOW_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Windowing mode is not selected."]
  #[inline(always)]
  pub fn window_en_0(self) -> &'a mut W {
    self.variant(WINDOW_EN_A::WINDOW_EN_0)
  }
  #[doc = "Windowing mode is selected."]
  #[inline(always)]
  pub fn window_en_1(self) -> &'a mut W {
    self.variant(WINDOW_EN_A::WINDOW_EN_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLE_EN_A {
  #[doc = "0: Sampling mode is not selected."]
  SAMPLE_EN_0,
  #[doc = "1: Sampling mode is selected."]
  SAMPLE_EN_1,
}
impl From<SAMPLE_EN_A> for bool {
  #[inline(always)]
  fn from(variant: SAMPLE_EN_A) -> Self {
    match variant {
      SAMPLE_EN_A::SAMPLE_EN_0 => false,
      SAMPLE_EN_A::SAMPLE_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `SAMPLE_EN`"]
pub type SAMPLE_EN_R = crate::R<bool, SAMPLE_EN_A>;
impl SAMPLE_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SAMPLE_EN_A {
    match self.bits {
      false => SAMPLE_EN_A::SAMPLE_EN_0,
      true => SAMPLE_EN_A::SAMPLE_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SAMPLE_EN_0`"]
  #[inline(always)]
  pub fn is_sample_en_0(&self) -> bool {
    *self == SAMPLE_EN_A::SAMPLE_EN_0
  }
  #[doc = "Checks if the value of the field is `SAMPLE_EN_1`"]
  #[inline(always)]
  pub fn is_sample_en_1(&self) -> bool {
    *self == SAMPLE_EN_A::SAMPLE_EN_1
  }
}
#[doc = "Write proxy for field `SAMPLE_EN`"]
pub struct SAMPLE_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> SAMPLE_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SAMPLE_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Sampling mode is not selected."]
  #[inline(always)]
  pub fn sample_en_0(self) -> &'a mut W {
    self.variant(SAMPLE_EN_A::SAMPLE_EN_0)
  }
  #[doc = "Sampling mode is selected."]
  #[inline(always)]
  pub fn sample_en_1(self) -> &'a mut W {
    self.variant(SAMPLE_EN_A::SAMPLE_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_EN_A {
  #[doc = "0: DMA is disabled."]
  DMA_EN_0,
  #[doc = "1: DMA is enabled."]
  DMA_EN_1,
}
impl From<DMA_EN_A> for bool {
  #[inline(always)]
  fn from(variant: DMA_EN_A) -> Self {
    match variant {
      DMA_EN_A::DMA_EN_0 => false,
      DMA_EN_A::DMA_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `DMA_EN`"]
pub type DMA_EN_R = crate::R<bool, DMA_EN_A>;
impl DMA_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DMA_EN_A {
    match self.bits {
      false => DMA_EN_A::DMA_EN_0,
      true => DMA_EN_A::DMA_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DMA_EN_0`"]
  #[inline(always)]
  pub fn is_dma_en_0(&self) -> bool {
    *self == DMA_EN_A::DMA_EN_0
  }
  #[doc = "Checks if the value of the field is `DMA_EN_1`"]
  #[inline(always)]
  pub fn is_dma_en_1(&self) -> bool {
    *self == DMA_EN_A::DMA_EN_1
  }
}
#[doc = "Write proxy for field `DMA_EN`"]
pub struct DMA_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DMA_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DMA is disabled."]
  #[inline(always)]
  pub fn dma_en_0(self) -> &'a mut W {
    self.variant(DMA_EN_A::DMA_EN_0)
  }
  #[doc = "DMA is enabled."]
  #[inline(always)]
  pub fn dma_en_1(self) -> &'a mut W {
    self.variant(DMA_EN_A::DMA_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Comparator invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUT_INV_A {
  #[doc = "0: Does not invert the comparator output."]
  COUT_INV_0,
  #[doc = "1: Inverts the comparator output."]
  COUT_INV_1,
}
impl From<COUT_INV_A> for bool {
  #[inline(always)]
  fn from(variant: COUT_INV_A) -> Self {
    match variant {
      COUT_INV_A::COUT_INV_0 => false,
      COUT_INV_A::COUT_INV_1 => true,
    }
  }
}
#[doc = "Reader of field `COUT_INV`"]
pub type COUT_INV_R = crate::R<bool, COUT_INV_A>;
impl COUT_INV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COUT_INV_A {
    match self.bits {
      false => COUT_INV_A::COUT_INV_0,
      true => COUT_INV_A::COUT_INV_1,
    }
  }
  #[doc = "Checks if the value of the field is `COUT_INV_0`"]
  #[inline(always)]
  pub fn is_cout_inv_0(&self) -> bool {
    *self == COUT_INV_A::COUT_INV_0
  }
  #[doc = "Checks if the value of the field is `COUT_INV_1`"]
  #[inline(always)]
  pub fn is_cout_inv_1(&self) -> bool {
    *self == COUT_INV_A::COUT_INV_1
  }
}
#[doc = "Write proxy for field `COUT_INV`"]
pub struct COUT_INV_W<'a> {
  w: &'a mut W,
}
impl<'a> COUT_INV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COUT_INV_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Does not invert the comparator output."]
  #[inline(always)]
  pub fn cout_inv_0(self) -> &'a mut W {
    self.variant(COUT_INV_A::COUT_INV_0)
  }
  #[doc = "Inverts the comparator output."]
  #[inline(always)]
  pub fn cout_inv_1(self) -> &'a mut W {
    self.variant(COUT_INV_A::COUT_INV_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUT_SEL_A {
  #[doc = "0: Set CMPO to equal COUT (filtered comparator output)."]
  COUT_SEL_0,
  #[doc = "1: Set CMPO to equal COUTA (unfiltered comparator output)."]
  COUT_SEL_1,
}
impl From<COUT_SEL_A> for bool {
  #[inline(always)]
  fn from(variant: COUT_SEL_A) -> Self {
    match variant {
      COUT_SEL_A::COUT_SEL_0 => false,
      COUT_SEL_A::COUT_SEL_1 => true,
    }
  }
}
#[doc = "Reader of field `COUT_SEL`"]
pub type COUT_SEL_R = crate::R<bool, COUT_SEL_A>;
impl COUT_SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COUT_SEL_A {
    match self.bits {
      false => COUT_SEL_A::COUT_SEL_0,
      true => COUT_SEL_A::COUT_SEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `COUT_SEL_0`"]
  #[inline(always)]
  pub fn is_cout_sel_0(&self) -> bool {
    *self == COUT_SEL_A::COUT_SEL_0
  }
  #[doc = "Checks if the value of the field is `COUT_SEL_1`"]
  #[inline(always)]
  pub fn is_cout_sel_1(&self) -> bool {
    *self == COUT_SEL_A::COUT_SEL_1
  }
}
#[doc = "Write proxy for field `COUT_SEL`"]
pub struct COUT_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> COUT_SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COUT_SEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
  #[inline(always)]
  pub fn cout_sel_0(self) -> &'a mut W {
    self.variant(COUT_SEL_A::COUT_SEL_0)
  }
  #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
  #[inline(always)]
  pub fn cout_sel_1(self) -> &'a mut W {
    self.variant(COUT_SEL_A::COUT_SEL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUT_PEN_A {
  #[doc = "0: no description available"]
  COUT_PEN_0,
  #[doc = "1: no description available"]
  COUT_PEN_1,
}
impl From<COUT_PEN_A> for bool {
  #[inline(always)]
  fn from(variant: COUT_PEN_A) -> Self {
    match variant {
      COUT_PEN_A::COUT_PEN_0 => false,
      COUT_PEN_A::COUT_PEN_1 => true,
    }
  }
}
#[doc = "Reader of field `COUT_PEN`"]
pub type COUT_PEN_R = crate::R<bool, COUT_PEN_A>;
impl COUT_PEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COUT_PEN_A {
    match self.bits {
      false => COUT_PEN_A::COUT_PEN_0,
      true => COUT_PEN_A::COUT_PEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `COUT_PEN_0`"]
  #[inline(always)]
  pub fn is_cout_pen_0(&self) -> bool {
    *self == COUT_PEN_A::COUT_PEN_0
  }
  #[doc = "Checks if the value of the field is `COUT_PEN_1`"]
  #[inline(always)]
  pub fn is_cout_pen_1(&self) -> bool {
    *self == COUT_PEN_A::COUT_PEN_1
  }
}
#[doc = "Write proxy for field `COUT_PEN`"]
pub struct COUT_PEN_W<'a> {
  w: &'a mut W,
}
impl<'a> COUT_PEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COUT_PEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cout_pen_0(self) -> &'a mut W {
    self.variant(COUT_PEN_A::COUT_PEN_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cout_pen_1(self) -> &'a mut W {
    self.variant(COUT_PEN_A::COUT_PEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILT_CNT_A {
  #[doc = "0: no description available"]
  FILT_CNT_0,
  #[doc = "1: 1 consecutive sample must agree (comparator output is simply sampled)."]
  FILT_CNT_1,
  #[doc = "2: 2 consecutive samples must agree."]
  FILT_CNT_2,
  #[doc = "3: 3 consecutive samples must agree."]
  FILT_CNT_3,
  #[doc = "4: 4 consecutive samples must agree."]
  FILT_CNT_4,
  #[doc = "5: 5 consecutive samples must agree."]
  FILT_CNT_5,
  #[doc = "6: 6 consecutive samples must agree."]
  FILT_CNT_6,
  #[doc = "7: 7 consecutive samples must agree."]
  FILT_CNT_7,
}
impl From<FILT_CNT_A> for u8 {
  #[inline(always)]
  fn from(variant: FILT_CNT_A) -> Self {
    match variant {
      FILT_CNT_A::FILT_CNT_0 => 0,
      FILT_CNT_A::FILT_CNT_1 => 1,
      FILT_CNT_A::FILT_CNT_2 => 2,
      FILT_CNT_A::FILT_CNT_3 => 3,
      FILT_CNT_A::FILT_CNT_4 => 4,
      FILT_CNT_A::FILT_CNT_5 => 5,
      FILT_CNT_A::FILT_CNT_6 => 6,
      FILT_CNT_A::FILT_CNT_7 => 7,
    }
  }
}
#[doc = "Reader of field `FILT_CNT`"]
pub type FILT_CNT_R = crate::R<u8, FILT_CNT_A>;
impl FILT_CNT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILT_CNT_A {
    match self.bits {
      0 => FILT_CNT_A::FILT_CNT_0,
      1 => FILT_CNT_A::FILT_CNT_1,
      2 => FILT_CNT_A::FILT_CNT_2,
      3 => FILT_CNT_A::FILT_CNT_3,
      4 => FILT_CNT_A::FILT_CNT_4,
      5 => FILT_CNT_A::FILT_CNT_5,
      6 => FILT_CNT_A::FILT_CNT_6,
      7 => FILT_CNT_A::FILT_CNT_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_0`"]
  #[inline(always)]
  pub fn is_filt_cnt_0(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_0
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_1`"]
  #[inline(always)]
  pub fn is_filt_cnt_1(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_1
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_2`"]
  #[inline(always)]
  pub fn is_filt_cnt_2(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_2
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_3`"]
  #[inline(always)]
  pub fn is_filt_cnt_3(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_3
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_4`"]
  #[inline(always)]
  pub fn is_filt_cnt_4(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_4
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_5`"]
  #[inline(always)]
  pub fn is_filt_cnt_5(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_5
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_6`"]
  #[inline(always)]
  pub fn is_filt_cnt_6(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_6
  }
  #[doc = "Checks if the value of the field is `FILT_CNT_7`"]
  #[inline(always)]
  pub fn is_filt_cnt_7(&self) -> bool {
    *self == FILT_CNT_A::FILT_CNT_7
  }
}
#[doc = "Write proxy for field `FILT_CNT`"]
pub struct FILT_CNT_W<'a> {
  w: &'a mut W,
}
impl<'a> FILT_CNT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILT_CNT_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn filt_cnt_0(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_0)
  }
  #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
  #[inline(always)]
  pub fn filt_cnt_1(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_1)
  }
  #[doc = "2 consecutive samples must agree."]
  #[inline(always)]
  pub fn filt_cnt_2(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_2)
  }
  #[doc = "3 consecutive samples must agree."]
  #[inline(always)]
  pub fn filt_cnt_3(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_3)
  }
  #[doc = "4 consecutive samples must agree."]
  #[inline(always)]
  pub fn filt_cnt_4(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_4)
  }
  #[doc = "5 consecutive samples must agree."]
  #[inline(always)]
  pub fn filt_cnt_5(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_5)
  }
  #[doc = "6 consecutive samples must agree."]
  #[inline(always)]
  pub fn filt_cnt_6(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_6)
  }
  #[doc = "7 consecutive samples must agree."]
  #[inline(always)]
  pub fn filt_cnt_7(self) -> &'a mut W {
    self.variant(FILT_CNT_A::FILT_CNT_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
#[doc = "Reader of field `FILT_PER`"]
pub type FILT_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT_PER`"]
pub struct FILT_PER_W<'a> {
  w: &'a mut W,
}
impl<'a> FILT_PER_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Windowing Enable"]
  #[inline(always)]
  pub fn window_en(&self) -> WINDOW_EN_R {
    WINDOW_EN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Sample Enable"]
  #[inline(always)]
  pub fn sample_en(&self) -> SAMPLE_EN_R {
    SAMPLE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - DMA Enable"]
  #[inline(always)]
  pub fn dma_en(&self) -> DMA_EN_R {
    DMA_EN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Comparator invert"]
  #[inline(always)]
  pub fn cout_inv(&self) -> COUT_INV_R {
    COUT_INV_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Comparator Output Select"]
  #[inline(always)]
  pub fn cout_sel(&self) -> COUT_SEL_R {
    COUT_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Comparator Output Pin Enable"]
  #[inline(always)]
  pub fn cout_pen(&self) -> COUT_PEN_R {
    COUT_PEN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bits 16:18 - Filter Sample Count"]
  #[inline(always)]
  pub fn filt_cnt(&self) -> FILT_CNT_R {
    FILT_CNT_R::new(((self.bits >> 16) & 0x07) as u8)
  }
  #[doc = "Bits 24:31 - Filter Sample Period"]
  #[inline(always)]
  pub fn filt_per(&self) -> FILT_PER_R {
    FILT_PER_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Windowing Enable"]
  #[inline(always)]
  pub fn window_en(&mut self) -> WINDOW_EN_W {
    WINDOW_EN_W { w: self }
  }
  #[doc = "Bit 1 - Sample Enable"]
  #[inline(always)]
  pub fn sample_en(&mut self) -> SAMPLE_EN_W {
    SAMPLE_EN_W { w: self }
  }
  #[doc = "Bit 2 - DMA Enable"]
  #[inline(always)]
  pub fn dma_en(&mut self) -> DMA_EN_W {
    DMA_EN_W { w: self }
  }
  #[doc = "Bit 3 - Comparator invert"]
  #[inline(always)]
  pub fn cout_inv(&mut self) -> COUT_INV_W {
    COUT_INV_W { w: self }
  }
  #[doc = "Bit 4 - Comparator Output Select"]
  #[inline(always)]
  pub fn cout_sel(&mut self) -> COUT_SEL_W {
    COUT_SEL_W { w: self }
  }
  #[doc = "Bit 5 - Comparator Output Pin Enable"]
  #[inline(always)]
  pub fn cout_pen(&mut self) -> COUT_PEN_W {
    COUT_PEN_W { w: self }
  }
  #[doc = "Bits 16:18 - Filter Sample Count"]
  #[inline(always)]
  pub fn filt_cnt(&mut self) -> FILT_CNT_W {
    FILT_CNT_W { w: self }
  }
  #[doc = "Bits 24:31 - Filter Sample Period"]
  #[inline(always)]
  pub fn filt_per(&mut self) -> FILT_PER_W {
    FILT_PER_W { w: self }
  }
}
