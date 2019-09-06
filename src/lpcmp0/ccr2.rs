#[doc = "Reader of register CCR2"]
pub type R = crate::R<u32, super::CCR2>;
#[doc = "Writer for register CCR2"]
pub type W = crate::W<u32, super::CCR2>;
#[doc = "Register CCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "CMP High Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_HPMD_A {
  #[doc = "0: Low speed comparison mode is selected.(when CMP_NPMD is 0)"]
  CMP_HPMD_0,
  #[doc = "1: High speed comparison mode is selected.(when CMP_NPMD is 0)"]
  CMP_HPMD_1,
}
impl From<CMP_HPMD_A> for bool {
  #[inline(always)]
  fn from(variant: CMP_HPMD_A) -> Self {
    match variant {
      CMP_HPMD_A::CMP_HPMD_0 => false,
      CMP_HPMD_A::CMP_HPMD_1 => true,
    }
  }
}
#[doc = "Reader of field `CMP_HPMD`"]
pub type CMP_HPMD_R = crate::R<bool, CMP_HPMD_A>;
impl CMP_HPMD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CMP_HPMD_A {
    match self.bits {
      false => CMP_HPMD_A::CMP_HPMD_0,
      true => CMP_HPMD_A::CMP_HPMD_1,
    }
  }
  #[doc = "Checks if the value of the field is `CMP_HPMD_0`"]
  #[inline(always)]
  pub fn is_cmp_hpmd_0(&self) -> bool {
    *self == CMP_HPMD_A::CMP_HPMD_0
  }
  #[doc = "Checks if the value of the field is `CMP_HPMD_1`"]
  #[inline(always)]
  pub fn is_cmp_hpmd_1(&self) -> bool {
    *self == CMP_HPMD_A::CMP_HPMD_1
  }
}
#[doc = "Write proxy for field `CMP_HPMD`"]
pub struct CMP_HPMD_W<'a> {
  w: &'a mut W,
}
impl<'a> CMP_HPMD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CMP_HPMD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Low speed comparison mode is selected.(when CMP_NPMD is 0)"]
  #[inline(always)]
  pub fn cmp_hpmd_0(self) -> &'a mut W {
    self.variant(CMP_HPMD_A::CMP_HPMD_0)
  }
  #[doc = "High speed comparison mode is selected.(when CMP_NPMD is 0)"]
  #[inline(always)]
  pub fn cmp_hpmd_1(self) -> &'a mut W {
    self.variant(CMP_HPMD_A::CMP_HPMD_1)
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
#[doc = "CMP Nano Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_NPMD_A {
  #[doc = "0: Nano Power Comparator is not enabled (mode is determined by CMP_HPMD)"]
  CMP_NPMD_0,
  #[doc = "1: Nano Power Comparator is enabled"]
  CMP_NPMD_1,
}
impl From<CMP_NPMD_A> for bool {
  #[inline(always)]
  fn from(variant: CMP_NPMD_A) -> Self {
    match variant {
      CMP_NPMD_A::CMP_NPMD_0 => false,
      CMP_NPMD_A::CMP_NPMD_1 => true,
    }
  }
}
#[doc = "Reader of field `CMP_NPMD`"]
pub type CMP_NPMD_R = crate::R<bool, CMP_NPMD_A>;
impl CMP_NPMD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CMP_NPMD_A {
    match self.bits {
      false => CMP_NPMD_A::CMP_NPMD_0,
      true => CMP_NPMD_A::CMP_NPMD_1,
    }
  }
  #[doc = "Checks if the value of the field is `CMP_NPMD_0`"]
  #[inline(always)]
  pub fn is_cmp_npmd_0(&self) -> bool {
    *self == CMP_NPMD_A::CMP_NPMD_0
  }
  #[doc = "Checks if the value of the field is `CMP_NPMD_1`"]
  #[inline(always)]
  pub fn is_cmp_npmd_1(&self) -> bool {
    *self == CMP_NPMD_A::CMP_NPMD_1
  }
}
#[doc = "Write proxy for field `CMP_NPMD`"]
pub struct CMP_NPMD_W<'a> {
  w: &'a mut W,
}
impl<'a> CMP_NPMD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CMP_NPMD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Nano Power Comparator is not enabled (mode is determined by CMP_HPMD)"]
  #[inline(always)]
  pub fn cmp_npmd_0(self) -> &'a mut W {
    self.variant(CMP_NPMD_A::CMP_NPMD_0)
  }
  #[doc = "Nano Power Comparator is enabled"]
  #[inline(always)]
  pub fn cmp_npmd_1(self) -> &'a mut W {
    self.variant(CMP_NPMD_A::CMP_NPMD_1)
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
#[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTCTR_A {
  #[doc = "0: The hard block output has level 0 hysteresis internally."]
  HYSTCTR_0,
  #[doc = "1: The hard block output has level 1 hysteresis internally."]
  HYSTCTR_1,
  #[doc = "2: The hard block output has level 2 hysteresis internally."]
  HYSTCTR_2,
  #[doc = "3: The hard block output has level 3 hysteresis internally."]
  HYSTCTR_3,
}
impl From<HYSTCTR_A> for u8 {
  #[inline(always)]
  fn from(variant: HYSTCTR_A) -> Self {
    match variant {
      HYSTCTR_A::HYSTCTR_0 => 0,
      HYSTCTR_A::HYSTCTR_1 => 1,
      HYSTCTR_A::HYSTCTR_2 => 2,
      HYSTCTR_A::HYSTCTR_3 => 3,
    }
  }
}
#[doc = "Reader of field `HYSTCTR`"]
pub type HYSTCTR_R = crate::R<u8, HYSTCTR_A>;
impl HYSTCTR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HYSTCTR_A {
    match self.bits {
      0 => HYSTCTR_A::HYSTCTR_0,
      1 => HYSTCTR_A::HYSTCTR_1,
      2 => HYSTCTR_A::HYSTCTR_2,
      3 => HYSTCTR_A::HYSTCTR_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `HYSTCTR_0`"]
  #[inline(always)]
  pub fn is_hystctr_0(&self) -> bool {
    *self == HYSTCTR_A::HYSTCTR_0
  }
  #[doc = "Checks if the value of the field is `HYSTCTR_1`"]
  #[inline(always)]
  pub fn is_hystctr_1(&self) -> bool {
    *self == HYSTCTR_A::HYSTCTR_1
  }
  #[doc = "Checks if the value of the field is `HYSTCTR_2`"]
  #[inline(always)]
  pub fn is_hystctr_2(&self) -> bool {
    *self == HYSTCTR_A::HYSTCTR_2
  }
  #[doc = "Checks if the value of the field is `HYSTCTR_3`"]
  #[inline(always)]
  pub fn is_hystctr_3(&self) -> bool {
    *self == HYSTCTR_A::HYSTCTR_3
  }
}
#[doc = "Write proxy for field `HYSTCTR`"]
pub struct HYSTCTR_W<'a> {
  w: &'a mut W,
}
impl<'a> HYSTCTR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HYSTCTR_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "The hard block output has level 0 hysteresis internally."]
  #[inline(always)]
  pub fn hystctr_0(self) -> &'a mut W {
    self.variant(HYSTCTR_A::HYSTCTR_0)
  }
  #[doc = "The hard block output has level 1 hysteresis internally."]
  #[inline(always)]
  pub fn hystctr_1(self) -> &'a mut W {
    self.variant(HYSTCTR_A::HYSTCTR_1)
  }
  #[doc = "The hard block output has level 2 hysteresis internally."]
  #[inline(always)]
  pub fn hystctr_2(self) -> &'a mut W {
    self.variant(HYSTCTR_A::HYSTCTR_2)
  }
  #[doc = "The hard block output has level 3 hysteresis internally."]
  #[inline(always)]
  pub fn hystctr_3(self) -> &'a mut W {
    self.variant(HYSTCTR_A::HYSTCTR_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Plus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSEL_A {
  #[doc = "0: Input 0"]
  PSEL_0,
  #[doc = "1: Input 1"]
  PSEL_1,
  #[doc = "2: Input 2"]
  PSEL_2,
  #[doc = "3: Input 3"]
  PSEL_3,
  #[doc = "4: Input 4"]
  PSEL_4,
  #[doc = "5: Input 5"]
  PSEL_5,
  #[doc = "6: Input 6"]
  PSEL_6,
  #[doc = "7: Internal DAC output"]
  PSEL_7,
}
impl From<PSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: PSEL_A) -> Self {
    match variant {
      PSEL_A::PSEL_0 => 0,
      PSEL_A::PSEL_1 => 1,
      PSEL_A::PSEL_2 => 2,
      PSEL_A::PSEL_3 => 3,
      PSEL_A::PSEL_4 => 4,
      PSEL_A::PSEL_5 => 5,
      PSEL_A::PSEL_6 => 6,
      PSEL_A::PSEL_7 => 7,
    }
  }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PSEL_A {
    match self.bits {
      0 => PSEL_A::PSEL_0,
      1 => PSEL_A::PSEL_1,
      2 => PSEL_A::PSEL_2,
      3 => PSEL_A::PSEL_3,
      4 => PSEL_A::PSEL_4,
      5 => PSEL_A::PSEL_5,
      6 => PSEL_A::PSEL_6,
      7 => PSEL_A::PSEL_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PSEL_0`"]
  #[inline(always)]
  pub fn is_psel_0(&self) -> bool {
    *self == PSEL_A::PSEL_0
  }
  #[doc = "Checks if the value of the field is `PSEL_1`"]
  #[inline(always)]
  pub fn is_psel_1(&self) -> bool {
    *self == PSEL_A::PSEL_1
  }
  #[doc = "Checks if the value of the field is `PSEL_2`"]
  #[inline(always)]
  pub fn is_psel_2(&self) -> bool {
    *self == PSEL_A::PSEL_2
  }
  #[doc = "Checks if the value of the field is `PSEL_3`"]
  #[inline(always)]
  pub fn is_psel_3(&self) -> bool {
    *self == PSEL_A::PSEL_3
  }
  #[doc = "Checks if the value of the field is `PSEL_4`"]
  #[inline(always)]
  pub fn is_psel_4(&self) -> bool {
    *self == PSEL_A::PSEL_4
  }
  #[doc = "Checks if the value of the field is `PSEL_5`"]
  #[inline(always)]
  pub fn is_psel_5(&self) -> bool {
    *self == PSEL_A::PSEL_5
  }
  #[doc = "Checks if the value of the field is `PSEL_6`"]
  #[inline(always)]
  pub fn is_psel_6(&self) -> bool {
    *self == PSEL_A::PSEL_6
  }
  #[doc = "Checks if the value of the field is `PSEL_7`"]
  #[inline(always)]
  pub fn is_psel_7(&self) -> bool {
    *self == PSEL_A::PSEL_7
  }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PSEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Input 0"]
  #[inline(always)]
  pub fn psel_0(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_0)
  }
  #[doc = "Input 1"]
  #[inline(always)]
  pub fn psel_1(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_1)
  }
  #[doc = "Input 2"]
  #[inline(always)]
  pub fn psel_2(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_2)
  }
  #[doc = "Input 3"]
  #[inline(always)]
  pub fn psel_3(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_3)
  }
  #[doc = "Input 4"]
  #[inline(always)]
  pub fn psel_4(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_4)
  }
  #[doc = "Input 5"]
  #[inline(always)]
  pub fn psel_5(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_5)
  }
  #[doc = "Input 6"]
  #[inline(always)]
  pub fn psel_6(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_6)
  }
  #[doc = "Internal DAC output"]
  #[inline(always)]
  pub fn psel_7(self) -> &'a mut W {
    self.variant(PSEL_A::PSEL_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
#[doc = "Minus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSEL_A {
  #[doc = "0: Input 0"]
  MSEL_0,
  #[doc = "1: Input 1"]
  MSEL_1,
  #[doc = "2: Input 2"]
  MSEL_2,
  #[doc = "3: Input 3"]
  MSEL_3,
  #[doc = "4: Input 4"]
  MSEL_4,
  #[doc = "5: Input 5"]
  MSEL_5,
  #[doc = "6: Input 6"]
  MSEL_6,
  #[doc = "7: Internal DAC output"]
  MSEL_7,
}
impl From<MSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: MSEL_A) -> Self {
    match variant {
      MSEL_A::MSEL_0 => 0,
      MSEL_A::MSEL_1 => 1,
      MSEL_A::MSEL_2 => 2,
      MSEL_A::MSEL_3 => 3,
      MSEL_A::MSEL_4 => 4,
      MSEL_A::MSEL_5 => 5,
      MSEL_A::MSEL_6 => 6,
      MSEL_A::MSEL_7 => 7,
    }
  }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, MSEL_A>;
impl MSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MSEL_A {
    match self.bits {
      0 => MSEL_A::MSEL_0,
      1 => MSEL_A::MSEL_1,
      2 => MSEL_A::MSEL_2,
      3 => MSEL_A::MSEL_3,
      4 => MSEL_A::MSEL_4,
      5 => MSEL_A::MSEL_5,
      6 => MSEL_A::MSEL_6,
      7 => MSEL_A::MSEL_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `MSEL_0`"]
  #[inline(always)]
  pub fn is_msel_0(&self) -> bool {
    *self == MSEL_A::MSEL_0
  }
  #[doc = "Checks if the value of the field is `MSEL_1`"]
  #[inline(always)]
  pub fn is_msel_1(&self) -> bool {
    *self == MSEL_A::MSEL_1
  }
  #[doc = "Checks if the value of the field is `MSEL_2`"]
  #[inline(always)]
  pub fn is_msel_2(&self) -> bool {
    *self == MSEL_A::MSEL_2
  }
  #[doc = "Checks if the value of the field is `MSEL_3`"]
  #[inline(always)]
  pub fn is_msel_3(&self) -> bool {
    *self == MSEL_A::MSEL_3
  }
  #[doc = "Checks if the value of the field is `MSEL_4`"]
  #[inline(always)]
  pub fn is_msel_4(&self) -> bool {
    *self == MSEL_A::MSEL_4
  }
  #[doc = "Checks if the value of the field is `MSEL_5`"]
  #[inline(always)]
  pub fn is_msel_5(&self) -> bool {
    *self == MSEL_A::MSEL_5
  }
  #[doc = "Checks if the value of the field is `MSEL_6`"]
  #[inline(always)]
  pub fn is_msel_6(&self) -> bool {
    *self == MSEL_A::MSEL_6
  }
  #[doc = "Checks if the value of the field is `MSEL_7`"]
  #[inline(always)]
  pub fn is_msel_7(&self) -> bool {
    *self == MSEL_A::MSEL_7
  }
}
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MSEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Input 0"]
  #[inline(always)]
  pub fn msel_0(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_0)
  }
  #[doc = "Input 1"]
  #[inline(always)]
  pub fn msel_1(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_1)
  }
  #[doc = "Input 2"]
  #[inline(always)]
  pub fn msel_2(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_2)
  }
  #[doc = "Input 3"]
  #[inline(always)]
  pub fn msel_3(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_3)
  }
  #[doc = "Input 4"]
  #[inline(always)]
  pub fn msel_4(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_4)
  }
  #[doc = "Input 5"]
  #[inline(always)]
  pub fn msel_5(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_5)
  }
  #[doc = "Input 6"]
  #[inline(always)]
  pub fn msel_6(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_6)
  }
  #[doc = "Internal DAC output"]
  #[inline(always)]
  pub fn msel_7(self) -> &'a mut W {
    self.variant(MSEL_A::MSEL_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - CMP High Power Mode Select"]
  #[inline(always)]
  pub fn cmp_hpmd(&self) -> CMP_HPMD_R {
    CMP_HPMD_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - CMP Nano Power Mode Select"]
  #[inline(always)]
  pub fn cmp_npmd(&self) -> CMP_NPMD_R {
    CMP_NPMD_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bits 4:5 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
  #[inline(always)]
  pub fn hystctr(&self) -> HYSTCTR_R {
    HYSTCTR_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 16:18 - Plus Input MUX Control"]
  #[inline(always)]
  pub fn psel(&self) -> PSEL_R {
    PSEL_R::new(((self.bits >> 16) & 0x07) as u8)
  }
  #[doc = "Bits 20:22 - Minus Input MUX Control"]
  #[inline(always)]
  pub fn msel(&self) -> MSEL_R {
    MSEL_R::new(((self.bits >> 20) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - CMP High Power Mode Select"]
  #[inline(always)]
  pub fn cmp_hpmd(&mut self) -> CMP_HPMD_W {
    CMP_HPMD_W { w: self }
  }
  #[doc = "Bit 1 - CMP Nano Power Mode Select"]
  #[inline(always)]
  pub fn cmp_npmd(&mut self) -> CMP_NPMD_W {
    CMP_NPMD_W { w: self }
  }
  #[doc = "Bits 4:5 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
  #[inline(always)]
  pub fn hystctr(&mut self) -> HYSTCTR_W {
    HYSTCTR_W { w: self }
  }
  #[doc = "Bits 16:18 - Plus Input MUX Control"]
  #[inline(always)]
  pub fn psel(&mut self) -> PSEL_W {
    PSEL_W { w: self }
  }
  #[doc = "Bits 20:22 - Minus Input MUX Control"]
  #[inline(always)]
  pub fn msel(&mut self) -> MSEL_W {
    MSEL_W { w: self }
  }
}
