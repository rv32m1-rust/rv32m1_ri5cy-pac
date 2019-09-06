#[doc = "Reader of register VEND_SPEC2"]
pub type R = crate::R<u32, super::VEND_SPEC2>;
#[doc = "Writer for register VEND_SPEC2"]
pub type W = crate::W<u32, super::VEND_SPEC2>;
#[doc = "Register VEND_SPEC2 `reset()`'s with value 0x1006"]
impl crate::ResetValue for super::VEND_SPEC2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x1006
  }
}
#[doc = "Card Interrupt Detection Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_D3_TEST_A {
  #[doc = "0: Check the card interrupt only when DATA3 is high."]
  CARD_INT_D3_TEST_0,
  #[doc = "1: Check the card interrupt by ignoring the status of DATA3."]
  CARD_INT_D3_TEST_1,
}
impl From<CARD_INT_D3_TEST_A> for bool {
  #[inline(always)]
  fn from(variant: CARD_INT_D3_TEST_A) -> Self {
    match variant {
      CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0 => false,
      CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1 => true,
    }
  }
}
#[doc = "Reader of field `CARD_INT_D3_TEST`"]
pub type CARD_INT_D3_TEST_R = crate::R<bool, CARD_INT_D3_TEST_A>;
impl CARD_INT_D3_TEST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CARD_INT_D3_TEST_A {
    match self.bits {
      false => CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0,
      true => CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1,
    }
  }
  #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_0`"]
  #[inline(always)]
  pub fn is_card_int_d3_test_0(&self) -> bool {
    *self == CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0
  }
  #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_1`"]
  #[inline(always)]
  pub fn is_card_int_d3_test_1(&self) -> bool {
    *self == CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1
  }
}
#[doc = "Write proxy for field `CARD_INT_D3_TEST`"]
pub struct CARD_INT_D3_TEST_W<'a> {
  w: &'a mut W,
}
impl<'a> CARD_INT_D3_TEST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CARD_INT_D3_TEST_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Check the card interrupt only when DATA3 is high."]
  #[inline(always)]
  pub fn card_int_d3_test_0(self) -> &'a mut W {
    self.variant(CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0)
  }
  #[doc = "Check the card interrupt by ignoring the status of DATA3."]
  #[inline(always)]
  pub fn card_int_d3_test_1(self) -> &'a mut W {
    self.variant(CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1)
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
#[doc = "Argument2 register enable for ACMD23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD23_ARGU2_EN_A {
  #[doc = "0: Disable"]
  ACMD23_ARGU2_EN_0,
  #[doc = "1: Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
  ACMD23_ARGU2_EN_1,
}
impl From<ACMD23_ARGU2_EN_A> for bool {
  #[inline(always)]
  fn from(variant: ACMD23_ARGU2_EN_A) -> Self {
    match variant {
      ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0 => false,
      ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `ACMD23_ARGU2_EN`"]
pub type ACMD23_ARGU2_EN_R = crate::R<bool, ACMD23_ARGU2_EN_A>;
impl ACMD23_ARGU2_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ACMD23_ARGU2_EN_A {
    match self.bits {
      false => ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0,
      true => ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_0`"]
  #[inline(always)]
  pub fn is_acmd23_argu2_en_0(&self) -> bool {
    *self == ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0
  }
  #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_1`"]
  #[inline(always)]
  pub fn is_acmd23_argu2_en_1(&self) -> bool {
    *self == ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1
  }
}
#[doc = "Write proxy for field `ACMD23_ARGU2_EN`"]
pub struct ACMD23_ARGU2_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> ACMD23_ARGU2_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ACMD23_ARGU2_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable"]
  #[inline(always)]
  pub fn acmd23_argu2_en_0(self) -> &'a mut W {
    self.variant(ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0)
  }
  #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
  #[inline(always)]
  pub fn acmd23_argu2_en_1(self) -> &'a mut W {
    self.variant(ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "Reader of field `AHB_RST`"]
pub type AHB_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_RST`"]
pub struct AHB_RST_W<'a> {
  w: &'a mut W,
}
impl<'a> AHB_RST_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
    self.w
  }
}
impl R {
  #[doc = "Bit 3 - Card Interrupt Detection Test"]
  #[inline(always)]
  pub fn card_int_d3_test(&self) -> CARD_INT_D3_TEST_R {
    CARD_INT_D3_TEST_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
  #[inline(always)]
  pub fn acmd23_argu2_en(&self) -> ACMD23_ARGU2_EN_R {
    ACMD23_ARGU2_EN_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 14 - AHB BUS reset"]
  #[inline(always)]
  pub fn ahb_rst(&self) -> AHB_RST_R {
    AHB_RST_R::new(((self.bits >> 14) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 3 - Card Interrupt Detection Test"]
  #[inline(always)]
  pub fn card_int_d3_test(&mut self) -> CARD_INT_D3_TEST_W {
    CARD_INT_D3_TEST_W { w: self }
  }
  #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
  #[inline(always)]
  pub fn acmd23_argu2_en(&mut self) -> ACMD23_ARGU2_EN_W {
    ACMD23_ARGU2_EN_W { w: self }
  }
  #[doc = "Bit 14 - AHB BUS reset"]
  #[inline(always)]
  pub fn ahb_rst(&mut self) -> AHB_RST_W {
    AHB_RST_W { w: self }
  }
}
