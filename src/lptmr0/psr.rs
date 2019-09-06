#[doc = "Reader of register PSR"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Writer for register PSR"]
pub type W = crate::W<u32, super::PSR>;
#[doc = "Register PSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Prescaler Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS_A {
  #[doc = "0: Prescaler/glitch filter clock 0 selected."]
  PCS_0,
  #[doc = "1: Prescaler/glitch filter clock 1 selected."]
  PCS_1,
  #[doc = "2: Prescaler/glitch filter clock 2 selected."]
  PCS_2,
  #[doc = "3: Prescaler/glitch filter clock 3 selected."]
  PCS_3,
}
impl From<PCS_A> for u8 {
  #[inline(always)]
  fn from(variant: PCS_A) -> Self {
    match variant {
      PCS_A::PCS_0 => 0,
      PCS_A::PCS_1 => 1,
      PCS_A::PCS_2 => 2,
      PCS_A::PCS_3 => 3,
    }
  }
}
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, PCS_A>;
impl PCS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PCS_A {
    match self.bits {
      0 => PCS_A::PCS_0,
      1 => PCS_A::PCS_1,
      2 => PCS_A::PCS_2,
      3 => PCS_A::PCS_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PCS_0`"]
  #[inline(always)]
  pub fn is_pcs_0(&self) -> bool {
    *self == PCS_A::PCS_0
  }
  #[doc = "Checks if the value of the field is `PCS_1`"]
  #[inline(always)]
  pub fn is_pcs_1(&self) -> bool {
    *self == PCS_A::PCS_1
  }
  #[doc = "Checks if the value of the field is `PCS_2`"]
  #[inline(always)]
  pub fn is_pcs_2(&self) -> bool {
    *self == PCS_A::PCS_2
  }
  #[doc = "Checks if the value of the field is `PCS_3`"]
  #[inline(always)]
  pub fn is_pcs_3(&self) -> bool {
    *self == PCS_A::PCS_3
  }
}
#[doc = "Write proxy for field `PCS`"]
pub struct PCS_W<'a> {
  w: &'a mut W,
}
impl<'a> PCS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PCS_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Prescaler/glitch filter clock 0 selected."]
  #[inline(always)]
  pub fn pcs_0(self) -> &'a mut W {
    self.variant(PCS_A::PCS_0)
  }
  #[doc = "Prescaler/glitch filter clock 1 selected."]
  #[inline(always)]
  pub fn pcs_1(self) -> &'a mut W {
    self.variant(PCS_A::PCS_1)
  }
  #[doc = "Prescaler/glitch filter clock 2 selected."]
  #[inline(always)]
  pub fn pcs_2(self) -> &'a mut W {
    self.variant(PCS_A::PCS_2)
  }
  #[doc = "Prescaler/glitch filter clock 3 selected."]
  #[inline(always)]
  pub fn pcs_3(self) -> &'a mut W {
    self.variant(PCS_A::PCS_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Prescaler Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBYP_A {
  #[doc = "0: Prescaler/glitch filter is enabled."]
  PBYP_0,
  #[doc = "1: Prescaler/glitch filter is bypassed."]
  PBYP_1,
}
impl From<PBYP_A> for bool {
  #[inline(always)]
  fn from(variant: PBYP_A) -> Self {
    match variant {
      PBYP_A::PBYP_0 => false,
      PBYP_A::PBYP_1 => true,
    }
  }
}
#[doc = "Reader of field `PBYP`"]
pub type PBYP_R = crate::R<bool, PBYP_A>;
impl PBYP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PBYP_A {
    match self.bits {
      false => PBYP_A::PBYP_0,
      true => PBYP_A::PBYP_1,
    }
  }
  #[doc = "Checks if the value of the field is `PBYP_0`"]
  #[inline(always)]
  pub fn is_pbyp_0(&self) -> bool {
    *self == PBYP_A::PBYP_0
  }
  #[doc = "Checks if the value of the field is `PBYP_1`"]
  #[inline(always)]
  pub fn is_pbyp_1(&self) -> bool {
    *self == PBYP_A::PBYP_1
  }
}
#[doc = "Write proxy for field `PBYP`"]
pub struct PBYP_W<'a> {
  w: &'a mut W,
}
impl<'a> PBYP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PBYP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Prescaler/glitch filter is enabled."]
  #[inline(always)]
  pub fn pbyp_0(self) -> &'a mut W {
    self.variant(PBYP_A::PBYP_0)
  }
  #[doc = "Prescaler/glitch filter is bypassed."]
  #[inline(always)]
  pub fn pbyp_1(self) -> &'a mut W {
    self.variant(PBYP_A::PBYP_1)
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
#[doc = "Prescale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALE_A {
  #[doc = "0: Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
  PRESCALE_0,
  #[doc = "1: Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
  PRESCALE_1,
  #[doc = "2: Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
  PRESCALE_2,
  #[doc = "3: Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
  PRESCALE_3,
  #[doc = "4: Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
  PRESCALE_4,
  #[doc = "5: Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
  PRESCALE_5,
  #[doc = "6: Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
  PRESCALE_6,
  #[doc = "7: Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
  PRESCALE_7,
  #[doc = "8: Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
  PRESCALE_8,
  #[doc = "9: Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
  PRESCALE_9,
  #[doc = "10: Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
  PRESCALE_10,
  #[doc = "11: Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
  PRESCALE_11,
  #[doc = "12: Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
  PRESCALE_12,
  #[doc = "13: Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
  PRESCALE_13,
  #[doc = "14: Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
  PRESCALE_14,
  #[doc = "15: Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
  PRESCALE_15,
}
impl From<PRESCALE_A> for u8 {
  #[inline(always)]
  fn from(variant: PRESCALE_A) -> Self {
    match variant {
      PRESCALE_A::PRESCALE_0 => 0,
      PRESCALE_A::PRESCALE_1 => 1,
      PRESCALE_A::PRESCALE_2 => 2,
      PRESCALE_A::PRESCALE_3 => 3,
      PRESCALE_A::PRESCALE_4 => 4,
      PRESCALE_A::PRESCALE_5 => 5,
      PRESCALE_A::PRESCALE_6 => 6,
      PRESCALE_A::PRESCALE_7 => 7,
      PRESCALE_A::PRESCALE_8 => 8,
      PRESCALE_A::PRESCALE_9 => 9,
      PRESCALE_A::PRESCALE_10 => 10,
      PRESCALE_A::PRESCALE_11 => 11,
      PRESCALE_A::PRESCALE_12 => 12,
      PRESCALE_A::PRESCALE_13 => 13,
      PRESCALE_A::PRESCALE_14 => 14,
      PRESCALE_A::PRESCALE_15 => 15,
    }
  }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PRESCALE_A {
    match self.bits {
      0 => PRESCALE_A::PRESCALE_0,
      1 => PRESCALE_A::PRESCALE_1,
      2 => PRESCALE_A::PRESCALE_2,
      3 => PRESCALE_A::PRESCALE_3,
      4 => PRESCALE_A::PRESCALE_4,
      5 => PRESCALE_A::PRESCALE_5,
      6 => PRESCALE_A::PRESCALE_6,
      7 => PRESCALE_A::PRESCALE_7,
      8 => PRESCALE_A::PRESCALE_8,
      9 => PRESCALE_A::PRESCALE_9,
      10 => PRESCALE_A::PRESCALE_10,
      11 => PRESCALE_A::PRESCALE_11,
      12 => PRESCALE_A::PRESCALE_12,
      13 => PRESCALE_A::PRESCALE_13,
      14 => PRESCALE_A::PRESCALE_14,
      15 => PRESCALE_A::PRESCALE_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PRESCALE_0`"]
  #[inline(always)]
  pub fn is_prescale_0(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_0
  }
  #[doc = "Checks if the value of the field is `PRESCALE_1`"]
  #[inline(always)]
  pub fn is_prescale_1(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_1
  }
  #[doc = "Checks if the value of the field is `PRESCALE_2`"]
  #[inline(always)]
  pub fn is_prescale_2(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_2
  }
  #[doc = "Checks if the value of the field is `PRESCALE_3`"]
  #[inline(always)]
  pub fn is_prescale_3(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_3
  }
  #[doc = "Checks if the value of the field is `PRESCALE_4`"]
  #[inline(always)]
  pub fn is_prescale_4(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_4
  }
  #[doc = "Checks if the value of the field is `PRESCALE_5`"]
  #[inline(always)]
  pub fn is_prescale_5(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_5
  }
  #[doc = "Checks if the value of the field is `PRESCALE_6`"]
  #[inline(always)]
  pub fn is_prescale_6(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_6
  }
  #[doc = "Checks if the value of the field is `PRESCALE_7`"]
  #[inline(always)]
  pub fn is_prescale_7(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_7
  }
  #[doc = "Checks if the value of the field is `PRESCALE_8`"]
  #[inline(always)]
  pub fn is_prescale_8(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_8
  }
  #[doc = "Checks if the value of the field is `PRESCALE_9`"]
  #[inline(always)]
  pub fn is_prescale_9(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_9
  }
  #[doc = "Checks if the value of the field is `PRESCALE_10`"]
  #[inline(always)]
  pub fn is_prescale_10(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_10
  }
  #[doc = "Checks if the value of the field is `PRESCALE_11`"]
  #[inline(always)]
  pub fn is_prescale_11(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_11
  }
  #[doc = "Checks if the value of the field is `PRESCALE_12`"]
  #[inline(always)]
  pub fn is_prescale_12(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_12
  }
  #[doc = "Checks if the value of the field is `PRESCALE_13`"]
  #[inline(always)]
  pub fn is_prescale_13(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_13
  }
  #[doc = "Checks if the value of the field is `PRESCALE_14`"]
  #[inline(always)]
  pub fn is_prescale_14(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_14
  }
  #[doc = "Checks if the value of the field is `PRESCALE_15`"]
  #[inline(always)]
  pub fn is_prescale_15(&self) -> bool {
    *self == PRESCALE_A::PRESCALE_15
  }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
  w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
  #[inline(always)]
  pub fn prescale_0(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_0)
  }
  #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
  #[inline(always)]
  pub fn prescale_1(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_1)
  }
  #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
  #[inline(always)]
  pub fn prescale_2(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_2)
  }
  #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
  #[inline(always)]
  pub fn prescale_3(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_3)
  }
  #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
  #[inline(always)]
  pub fn prescale_4(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_4)
  }
  #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
  #[inline(always)]
  pub fn prescale_5(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_5)
  }
  #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
  #[inline(always)]
  pub fn prescale_6(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_6)
  }
  #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
  #[inline(always)]
  pub fn prescale_7(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_7)
  }
  #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
  #[inline(always)]
  pub fn prescale_8(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_8)
  }
  #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
  #[inline(always)]
  pub fn prescale_9(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_9)
  }
  #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
  #[inline(always)]
  pub fn prescale_10(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_10)
  }
  #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
  #[inline(always)]
  pub fn prescale_11(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_11)
  }
  #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
  #[inline(always)]
  pub fn prescale_12(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_12)
  }
  #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
  #[inline(always)]
  pub fn prescale_13(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_13)
  }
  #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
  #[inline(always)]
  pub fn prescale_14(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_14)
  }
  #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
  #[inline(always)]
  pub fn prescale_15(self) -> &'a mut W {
    self.variant(PRESCALE_A::PRESCALE_15)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Prescaler Clock Select"]
  #[inline(always)]
  pub fn pcs(&self) -> PCS_R {
    PCS_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bit 2 - Prescaler Bypass"]
  #[inline(always)]
  pub fn pbyp(&self) -> PBYP_R {
    PBYP_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bits 3:6 - Prescale Value"]
  #[inline(always)]
  pub fn prescale(&self) -> PRESCALE_R {
    PRESCALE_R::new(((self.bits >> 3) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Prescaler Clock Select"]
  #[inline(always)]
  pub fn pcs(&mut self) -> PCS_W {
    PCS_W { w: self }
  }
  #[doc = "Bit 2 - Prescaler Bypass"]
  #[inline(always)]
  pub fn pbyp(&mut self) -> PBYP_W {
    PBYP_W { w: self }
  }
  #[doc = "Bits 3:6 - Prescale Value"]
  #[inline(always)]
  pub fn prescale(&mut self) -> PRESCALE_W {
    PRESCALE_W { w: self }
  }
}
