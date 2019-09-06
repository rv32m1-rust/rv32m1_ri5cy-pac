#[doc = "Reader of register MISC2"]
pub type R = crate::R<u32, super::MISC2>;
#[doc = "Writer for register MISC2"]
pub type W = crate::W<u32, super::MISC2>;
#[doc = "Register MISC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Systick clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICK_CLK_EN_A {
  #[doc = "0: Systick clock is disabled"]
  SYSTICK_CLK_EN_0,
  #[doc = "1: Systick clock is enabled"]
  SYSTICK_CLK_EN_1,
}
impl From<SYSTICK_CLK_EN_A> for bool {
  #[inline(always)]
  fn from(variant: SYSTICK_CLK_EN_A) -> Self {
    match variant {
      SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_0 => false,
      SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `systick_clk_en`"]
pub type SYSTICK_CLK_EN_R = crate::R<bool, SYSTICK_CLK_EN_A>;
impl SYSTICK_CLK_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SYSTICK_CLK_EN_A {
    match self.bits {
      false => SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_0,
      true => SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SYSTICK_CLK_EN_0`"]
  #[inline(always)]
  pub fn is_systick_clk_en_0(&self) -> bool {
    *self == SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_0
  }
  #[doc = "Checks if the value of the field is `SYSTICK_CLK_EN_1`"]
  #[inline(always)]
  pub fn is_systick_clk_en_1(&self) -> bool {
    *self == SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_1
  }
}
#[doc = "Write proxy for field `systick_clk_en`"]
pub struct SYSTICK_CLK_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> SYSTICK_CLK_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SYSTICK_CLK_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Systick clock is disabled"]
  #[inline(always)]
  pub fn systick_clk_en_0(self) -> &'a mut W {
    self.variant(SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_0)
  }
  #[doc = "Systick clock is enabled"]
  #[inline(always)]
  pub fn systick_clk_en_1(self) -> &'a mut W {
    self.variant(SYSTICK_CLK_EN_A::SYSTICK_CLK_EN_1)
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
impl R {
  #[doc = "Bit 0 - Systick clock enable"]
  #[inline(always)]
  pub fn systick_clk_en(&self) -> SYSTICK_CLK_EN_R {
    SYSTICK_CLK_EN_R::new((self.bits & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Systick clock enable"]
  #[inline(always)]
  pub fn systick_clk_en(&mut self) -> SYSTICK_CLK_EN_W {
    SYSTICK_CLK_EN_W { w: self }
  }
}
