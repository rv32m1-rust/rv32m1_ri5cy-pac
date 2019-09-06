#[doc = "Reader of register ANA_TRIM"]
pub type R = crate::R<u32, super::ANA_TRIM>;
#[doc = "Writer for register ANA_TRIM"]
pub type W = crate::W<u32, super::ANA_TRIM>;
#[doc = "Register ANA_TRIM `reset()`'s with value 0x784b_0000"]
impl crate::ResetValue for super::ANA_TRIM {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x784b_0000
  }
}
#[doc = "Reader of field `BB_LDO_LS_SPARE`"]
pub type BB_LDO_LS_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_LDO_LS_SPARE`"]
pub struct BB_LDO_LS_SPARE_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_LDO_LS_SPARE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "rmap_bb_ldo_ls_trim_hv\\[2:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_LS_TRIM_A {
  #[doc = "0: 1.20 V (Default)"]
  BB_LDO_LS_TRIM_0,
  #[doc = "1: 1.25 V"]
  BB_LDO_LS_TRIM_1,
  #[doc = "2: 1.28 V"]
  BB_LDO_LS_TRIM_2,
  #[doc = "3: 1.33 V"]
  BB_LDO_LS_TRIM_3,
  #[doc = "4: 1.40 V"]
  BB_LDO_LS_TRIM_4,
  #[doc = "5: 1.44 V"]
  BB_LDO_LS_TRIM_5,
  #[doc = "6: 1.50 V"]
  BB_LDO_LS_TRIM_6,
  #[doc = "7: 1.66 V"]
  BB_LDO_LS_TRIM_7,
}
impl From<BB_LDO_LS_TRIM_A> for u8 {
  #[inline(always)]
  fn from(variant: BB_LDO_LS_TRIM_A) -> Self {
    match variant {
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_0 => 0,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_1 => 1,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_2 => 2,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_3 => 3,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_4 => 4,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_5 => 5,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_6 => 6,
      BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_7 => 7,
    }
  }
}
#[doc = "Reader of field `BB_LDO_LS_TRIM`"]
pub type BB_LDO_LS_TRIM_R = crate::R<u8, BB_LDO_LS_TRIM_A>;
impl BB_LDO_LS_TRIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BB_LDO_LS_TRIM_A {
    match self.bits {
      0 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_0,
      1 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_1,
      2 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_2,
      3 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_3,
      4 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_4,
      5 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_5,
      6 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_6,
      7 => BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_0`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_0(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_0
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_1`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_1(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_1
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_2`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_2(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_2
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_3`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_3(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_3
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_4`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_4(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_4
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_5`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_5(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_5
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_6`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_6(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_6
  }
  #[doc = "Checks if the value of the field is `BB_LDO_LS_TRIM_7`"]
  #[inline(always)]
  pub fn is_bb_ldo_ls_trim_7(&self) -> bool {
    *self == BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_7
  }
}
#[doc = "Write proxy for field `BB_LDO_LS_TRIM`"]
pub struct BB_LDO_LS_TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_LDO_LS_TRIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BB_LDO_LS_TRIM_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "1.20 V (Default)"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_0(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_0)
  }
  #[doc = "1.25 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_1(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_1)
  }
  #[doc = "1.28 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_2(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_2)
  }
  #[doc = "1.33 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_3(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_3)
  }
  #[doc = "1.40 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_4(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_4)
  }
  #[doc = "1.44 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_5(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_5)
  }
  #[doc = "1.50 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_6(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_6)
  }
  #[doc = "1.66 V"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim_7(self) -> &'a mut W {
    self.variant(BB_LDO_LS_TRIM_A::BB_LDO_LS_TRIM_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
    self.w
  }
}
#[doc = "Reader of field `BB_LDO_XO_SPARE`"]
pub type BB_LDO_XO_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_LDO_XO_SPARE`"]
pub struct BB_LDO_XO_SPARE_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_LDO_XO_SPARE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "rmap_bb_ldo_xo_trim_hv\\[2:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_XO_TRIM_A {
  #[doc = "0: 1.20 V (Default)"]
  BB_LDO_XO_TRIM_0,
  #[doc = "1: 1.25 V"]
  BB_LDO_XO_TRIM_1,
  #[doc = "2: 1.28 V"]
  BB_LDO_XO_TRIM_2,
  #[doc = "3: 1.33 V"]
  BB_LDO_XO_TRIM_3,
  #[doc = "4: 1.40 V"]
  BB_LDO_XO_TRIM_4,
  #[doc = "5: 1.44 V"]
  BB_LDO_XO_TRIM_5,
  #[doc = "6: 1.50 V"]
  BB_LDO_XO_TRIM_6,
  #[doc = "7: 1.66 V"]
  BB_LDO_XO_TRIM_7,
}
impl From<BB_LDO_XO_TRIM_A> for u8 {
  #[inline(always)]
  fn from(variant: BB_LDO_XO_TRIM_A) -> Self {
    match variant {
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_0 => 0,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_1 => 1,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_2 => 2,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_3 => 3,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_4 => 4,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_5 => 5,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_6 => 6,
      BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_7 => 7,
    }
  }
}
#[doc = "Reader of field `BB_LDO_XO_TRIM`"]
pub type BB_LDO_XO_TRIM_R = crate::R<u8, BB_LDO_XO_TRIM_A>;
impl BB_LDO_XO_TRIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BB_LDO_XO_TRIM_A {
    match self.bits {
      0 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_0,
      1 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_1,
      2 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_2,
      3 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_3,
      4 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_4,
      5 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_5,
      6 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_6,
      7 => BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_0`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_0(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_0
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_1`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_1(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_1
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_2`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_2(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_2
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_3`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_3(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_3
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_4`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_4(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_4
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_5`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_5(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_5
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_6`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_6(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_6
  }
  #[doc = "Checks if the value of the field is `BB_LDO_XO_TRIM_7`"]
  #[inline(always)]
  pub fn is_bb_ldo_xo_trim_7(&self) -> bool {
    *self == BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_7
  }
}
#[doc = "Write proxy for field `BB_LDO_XO_TRIM`"]
pub struct BB_LDO_XO_TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_LDO_XO_TRIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BB_LDO_XO_TRIM_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "1.20 V (Default)"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_0(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_0)
  }
  #[doc = "1.25 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_1(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_1)
  }
  #[doc = "1.28 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_2(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_2)
  }
  #[doc = "1.33 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_3(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_3)
  }
  #[doc = "1.40 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_4(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_4)
  }
  #[doc = "1.44 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_5(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_5)
  }
  #[doc = "1.50 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_6(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_6)
  }
  #[doc = "1.66 V"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim_7(self) -> &'a mut W {
    self.variant(BB_LDO_XO_TRIM_A::BB_LDO_XO_TRIM_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "Reader of field `BB_XTAL_SPARE`"]
pub type BB_XTAL_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_XTAL_SPARE`"]
pub struct BB_XTAL_SPARE_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_SPARE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
    self.w
  }
}
#[doc = "Reader of field `BB_XTAL_TRIM`"]
pub type BB_XTAL_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_XTAL_TRIM`"]
pub struct BB_XTAL_TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> BB_XTAL_TRIM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
    self.w
  }
}
#[doc = "rmap_bg_1v_trim_hv\\[3:0\\]\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BG_1V_TRIM_A {
  #[doc = "0: 954.14 mV"]
  BG_1V_TRIM_0,
  #[doc = "1: 959.26 mV"]
  BG_1V_TRIM_1,
  #[doc = "2: 964.38 mV"]
  BG_1V_TRIM_2,
  #[doc = "3: 969.5 mV"]
  BG_1V_TRIM_3,
  #[doc = "4: 974.6 mV"]
  BG_1V_TRIM_4,
  #[doc = "5: 979.7 mV"]
  BG_1V_TRIM_5,
  #[doc = "6: 984.8 mV"]
  BG_1V_TRIM_6,
  #[doc = "7: 989.9 mV"]
  BG_1V_TRIM_7,
  #[doc = "8: 995 mV (Default)"]
  BG_1V_TRIM_8,
  #[doc = "9: 1 V"]
  BG_1V_TRIM_9,
  #[doc = "10: 1.005 V"]
  BG_1V_TRIM_10,
  #[doc = "11: 1.01 V"]
  BG_1V_TRIM_11,
  #[doc = "12: 1.015 V"]
  BG_1V_TRIM_12,
  #[doc = "13: 1.02 V"]
  BG_1V_TRIM_13,
  #[doc = "14: 1.025 V"]
  BG_1V_TRIM_14,
  #[doc = "15: 1.031 V"]
  BG_1V_TRIM_15,
}
impl From<BG_1V_TRIM_A> for u8 {
  #[inline(always)]
  fn from(variant: BG_1V_TRIM_A) -> Self {
    match variant {
      BG_1V_TRIM_A::BG_1V_TRIM_0 => 0,
      BG_1V_TRIM_A::BG_1V_TRIM_1 => 1,
      BG_1V_TRIM_A::BG_1V_TRIM_2 => 2,
      BG_1V_TRIM_A::BG_1V_TRIM_3 => 3,
      BG_1V_TRIM_A::BG_1V_TRIM_4 => 4,
      BG_1V_TRIM_A::BG_1V_TRIM_5 => 5,
      BG_1V_TRIM_A::BG_1V_TRIM_6 => 6,
      BG_1V_TRIM_A::BG_1V_TRIM_7 => 7,
      BG_1V_TRIM_A::BG_1V_TRIM_8 => 8,
      BG_1V_TRIM_A::BG_1V_TRIM_9 => 9,
      BG_1V_TRIM_A::BG_1V_TRIM_10 => 10,
      BG_1V_TRIM_A::BG_1V_TRIM_11 => 11,
      BG_1V_TRIM_A::BG_1V_TRIM_12 => 12,
      BG_1V_TRIM_A::BG_1V_TRIM_13 => 13,
      BG_1V_TRIM_A::BG_1V_TRIM_14 => 14,
      BG_1V_TRIM_A::BG_1V_TRIM_15 => 15,
    }
  }
}
#[doc = "Reader of field `BG_1V_TRIM`"]
pub type BG_1V_TRIM_R = crate::R<u8, BG_1V_TRIM_A>;
impl BG_1V_TRIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BG_1V_TRIM_A {
    match self.bits {
      0 => BG_1V_TRIM_A::BG_1V_TRIM_0,
      1 => BG_1V_TRIM_A::BG_1V_TRIM_1,
      2 => BG_1V_TRIM_A::BG_1V_TRIM_2,
      3 => BG_1V_TRIM_A::BG_1V_TRIM_3,
      4 => BG_1V_TRIM_A::BG_1V_TRIM_4,
      5 => BG_1V_TRIM_A::BG_1V_TRIM_5,
      6 => BG_1V_TRIM_A::BG_1V_TRIM_6,
      7 => BG_1V_TRIM_A::BG_1V_TRIM_7,
      8 => BG_1V_TRIM_A::BG_1V_TRIM_8,
      9 => BG_1V_TRIM_A::BG_1V_TRIM_9,
      10 => BG_1V_TRIM_A::BG_1V_TRIM_10,
      11 => BG_1V_TRIM_A::BG_1V_TRIM_11,
      12 => BG_1V_TRIM_A::BG_1V_TRIM_12,
      13 => BG_1V_TRIM_A::BG_1V_TRIM_13,
      14 => BG_1V_TRIM_A::BG_1V_TRIM_14,
      15 => BG_1V_TRIM_A::BG_1V_TRIM_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_0`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_0(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_0
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_1`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_1(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_1
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_2`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_2(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_2
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_3`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_3(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_3
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_4`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_4(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_4
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_5`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_5(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_5
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_6`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_6(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_6
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_7`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_7(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_7
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_8`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_8(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_8
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_9`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_9(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_9
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_10`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_10(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_10
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_11`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_11(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_11
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_12`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_12(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_12
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_13`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_13(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_13
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_14`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_14(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_14
  }
  #[doc = "Checks if the value of the field is `BG_1V_TRIM_15`"]
  #[inline(always)]
  pub fn is_bg_1v_trim_15(&self) -> bool {
    *self == BG_1V_TRIM_A::BG_1V_TRIM_15
  }
}
#[doc = "Write proxy for field `BG_1V_TRIM`"]
pub struct BG_1V_TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> BG_1V_TRIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BG_1V_TRIM_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "954.14 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_0(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_0)
  }
  #[doc = "959.26 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_1(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_1)
  }
  #[doc = "964.38 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_2(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_2)
  }
  #[doc = "969.5 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_3(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_3)
  }
  #[doc = "974.6 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_4(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_4)
  }
  #[doc = "979.7 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_5(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_5)
  }
  #[doc = "984.8 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_6(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_6)
  }
  #[doc = "989.9 mV"]
  #[inline(always)]
  pub fn bg_1v_trim_7(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_7)
  }
  #[doc = "995 mV (Default)"]
  #[inline(always)]
  pub fn bg_1v_trim_8(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_8)
  }
  #[doc = "1 V"]
  #[inline(always)]
  pub fn bg_1v_trim_9(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_9)
  }
  #[doc = "1.005 V"]
  #[inline(always)]
  pub fn bg_1v_trim_10(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_10)
  }
  #[doc = "1.01 V"]
  #[inline(always)]
  pub fn bg_1v_trim_11(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_11)
  }
  #[doc = "1.015 V"]
  #[inline(always)]
  pub fn bg_1v_trim_12(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_12)
  }
  #[doc = "1.02 V"]
  #[inline(always)]
  pub fn bg_1v_trim_13(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_13)
  }
  #[doc = "1.025 V"]
  #[inline(always)]
  pub fn bg_1v_trim_14(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_14)
  }
  #[doc = "1.031 V"]
  #[inline(always)]
  pub fn bg_1v_trim_15(self) -> &'a mut W {
    self.variant(BG_1V_TRIM_A::BG_1V_TRIM_15)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
#[doc = "rmap_bg_ibias_5u_trim_hv\\[3:0\\]\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BG_IBIAS_5U_TRIM_A {
  #[doc = "0: 3.55 uA"]
  BG_IBIAS_5U_TRIM_0,
  #[doc = "1: 3.73 uA"]
  BG_IBIAS_5U_TRIM_1,
  #[doc = "2: 4.04 uA"]
  BG_IBIAS_5U_TRIM_2,
  #[doc = "3: 4.22 uA"]
  BG_IBIAS_5U_TRIM_3,
  #[doc = "4: 4.39 uA"]
  BG_IBIAS_5U_TRIM_4,
  #[doc = "5: 4.57 uA"]
  BG_IBIAS_5U_TRIM_5,
  #[doc = "6: 4.89 uA"]
  BG_IBIAS_5U_TRIM_6,
  #[doc = "7: 5.06 (Default)"]
  BG_IBIAS_5U_TRIM_7,
  #[doc = "8: 5.23 uA"]
  BG_IBIAS_5U_TRIM_8,
  #[doc = "9: 5.41 uA"]
  BG_IBIAS_5U_TRIM_9,
  #[doc = "10: 5.72 uA"]
  BG_IBIAS_5U_TRIM_10,
  #[doc = "11: 5.9 uA"]
  BG_IBIAS_5U_TRIM_11,
  #[doc = "12: 6.07 uA"]
  BG_IBIAS_5U_TRIM_12,
  #[doc = "13: 6.25 uA"]
  BG_IBIAS_5U_TRIM_13,
  #[doc = "14: 6.56 uA"]
  BG_IBIAS_5U_TRIM_14,
  #[doc = "15: 6.74 uA"]
  BG_IBIAS_5U_TRIM_15,
}
impl From<BG_IBIAS_5U_TRIM_A> for u8 {
  #[inline(always)]
  fn from(variant: BG_IBIAS_5U_TRIM_A) -> Self {
    match variant {
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_0 => 0,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_1 => 1,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_2 => 2,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_3 => 3,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_4 => 4,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_5 => 5,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_6 => 6,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_7 => 7,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_8 => 8,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_9 => 9,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_10 => 10,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_11 => 11,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_12 => 12,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_13 => 13,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_14 => 14,
      BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_15 => 15,
    }
  }
}
#[doc = "Reader of field `BG_IBIAS_5U_TRIM`"]
pub type BG_IBIAS_5U_TRIM_R = crate::R<u8, BG_IBIAS_5U_TRIM_A>;
impl BG_IBIAS_5U_TRIM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BG_IBIAS_5U_TRIM_A {
    match self.bits {
      0 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_0,
      1 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_1,
      2 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_2,
      3 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_3,
      4 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_4,
      5 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_5,
      6 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_6,
      7 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_7,
      8 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_8,
      9 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_9,
      10 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_10,
      11 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_11,
      12 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_12,
      13 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_13,
      14 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_14,
      15 => BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_0`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_0(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_0
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_1`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_1(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_1
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_2`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_2(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_2
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_3`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_3(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_3
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_4`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_4(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_4
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_5`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_5(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_5
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_6`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_6(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_6
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_7`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_7(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_7
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_8`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_8(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_8
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_9`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_9(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_9
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_10`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_10(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_10
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_11`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_11(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_11
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_12`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_12(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_12
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_13`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_13(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_13
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_14`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_14(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_14
  }
  #[doc = "Checks if the value of the field is `BG_IBIAS_5U_TRIM_15`"]
  #[inline(always)]
  pub fn is_bg_ibias_5u_trim_15(&self) -> bool {
    *self == BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_15
  }
}
#[doc = "Write proxy for field `BG_IBIAS_5U_TRIM`"]
pub struct BG_IBIAS_5U_TRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> BG_IBIAS_5U_TRIM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BG_IBIAS_5U_TRIM_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "3.55 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_0(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_0)
  }
  #[doc = "3.73 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_1(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_1)
  }
  #[doc = "4.04 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_2(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_2)
  }
  #[doc = "4.22 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_3(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_3)
  }
  #[doc = "4.39 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_4(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_4)
  }
  #[doc = "4.57 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_5(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_5)
  }
  #[doc = "4.89 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_6(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_6)
  }
  #[doc = "5.06 (Default)"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_7(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_7)
  }
  #[doc = "5.23 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_8(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_8)
  }
  #[doc = "5.41 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_9(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_9)
  }
  #[doc = "5.72 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_10(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_10)
  }
  #[doc = "5.9 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_11(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_11)
  }
  #[doc = "6.07 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_12(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_12)
  }
  #[doc = "6.25 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_13(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_13)
  }
  #[doc = "6.56 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_14(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_14)
  }
  #[doc = "6.74 uA"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim_15(self) -> &'a mut W {
    self.variant(BG_IBIAS_5U_TRIM_A::BG_IBIAS_5U_TRIM_15)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - rmap_bb_ldo_ls_spare_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_ls_spare(&self) -> BB_LDO_LS_SPARE_R {
    BB_LDO_LS_SPARE_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 3:5 - rmap_bb_ldo_ls_trim_hv\\[2:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim(&self) -> BB_LDO_LS_TRIM_R {
    BB_LDO_LS_TRIM_R::new(((self.bits >> 3) & 0x07) as u8)
  }
  #[doc = "Bits 6:7 - rmap_bb_ldo_xo_spare_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_xo_spare(&self) -> BB_LDO_XO_SPARE_R {
    BB_LDO_XO_SPARE_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bits 8:10 - rmap_bb_ldo_xo_trim_hv\\[2:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim(&self) -> BB_LDO_XO_TRIM_R {
    BB_LDO_XO_TRIM_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bits 11:15 - rmap_bb_xtal_spare_hv\\[4:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_spare(&self) -> BB_XTAL_SPARE_R {
    BB_XTAL_SPARE_R::new(((self.bits >> 11) & 0x1f) as u8)
  }
  #[doc = "Bits 16:23 - rmap_bb_xtal_trim_hv\\[7:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_trim(&self) -> BB_XTAL_TRIM_R {
    BB_XTAL_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:27 - rmap_bg_1v_trim_hv\\[3:0\\]"]
  #[inline(always)]
  pub fn bg_1v_trim(&self) -> BG_1V_TRIM_R {
    BG_1V_TRIM_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - rmap_bg_ibias_5u_trim_hv\\[3:0\\]"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim(&self) -> BG_IBIAS_5U_TRIM_R {
    BG_IBIAS_5U_TRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - rmap_bb_ldo_ls_spare_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_ls_spare(&mut self) -> BB_LDO_LS_SPARE_W {
    BB_LDO_LS_SPARE_W { w: self }
  }
  #[doc = "Bits 3:5 - rmap_bb_ldo_ls_trim_hv\\[2:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_ls_trim(&mut self) -> BB_LDO_LS_TRIM_W {
    BB_LDO_LS_TRIM_W { w: self }
  }
  #[doc = "Bits 6:7 - rmap_bb_ldo_xo_spare_hv\\[1:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_xo_spare(&mut self) -> BB_LDO_XO_SPARE_W {
    BB_LDO_XO_SPARE_W { w: self }
  }
  #[doc = "Bits 8:10 - rmap_bb_ldo_xo_trim_hv\\[2:0\\]"]
  #[inline(always)]
  pub fn bb_ldo_xo_trim(&mut self) -> BB_LDO_XO_TRIM_W {
    BB_LDO_XO_TRIM_W { w: self }
  }
  #[doc = "Bits 11:15 - rmap_bb_xtal_spare_hv\\[4:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_spare(&mut self) -> BB_XTAL_SPARE_W {
    BB_XTAL_SPARE_W { w: self }
  }
  #[doc = "Bits 16:23 - rmap_bb_xtal_trim_hv\\[7:0\\]"]
  #[inline(always)]
  pub fn bb_xtal_trim(&mut self) -> BB_XTAL_TRIM_W {
    BB_XTAL_TRIM_W { w: self }
  }
  #[doc = "Bits 24:27 - rmap_bg_1v_trim_hv\\[3:0\\]"]
  #[inline(always)]
  pub fn bg_1v_trim(&mut self) -> BG_1V_TRIM_W {
    BG_1V_TRIM_W { w: self }
  }
  #[doc = "Bits 28:31 - rmap_bg_ibias_5u_trim_hv\\[3:0\\]"]
  #[inline(always)]
  pub fn bg_ibias_5u_trim(&mut self) -> BG_IBIAS_5U_TRIM_W {
    BG_IBIAS_5U_TRIM_W { w: self }
  }
}
