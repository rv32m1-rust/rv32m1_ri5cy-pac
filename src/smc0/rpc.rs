#[doc = "Reader of register RPC"]
pub type R = crate::R<u32, super::RPC>;
#[doc = "Writer for register RPC"]
pub type W = crate::W<u32, super::RPC>;
#[doc = "Register RPC `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `FILTCFG`"]
pub type FILTCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTCFG`"]
pub struct FILTCFG_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTCFG_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
    self.w
  }
}
#[doc = "Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEN_A {
  #[doc = "0: Slow clock reset pin filter disabled."]
  FILTEN_0,
  #[doc = "1: Slow clock reset pin filter enabled in Run modes."]
  FILTEN_1,
}
impl From<FILTEN_A> for bool {
  #[inline(always)]
  fn from(variant: FILTEN_A) -> Self {
    match variant {
      FILTEN_A::FILTEN_0 => false,
      FILTEN_A::FILTEN_1 => true,
    }
  }
}
#[doc = "Reader of field `FILTEN`"]
pub type FILTEN_R = crate::R<bool, FILTEN_A>;
impl FILTEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTEN_A {
    match self.bits {
      false => FILTEN_A::FILTEN_0,
      true => FILTEN_A::FILTEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FILTEN_0`"]
  #[inline(always)]
  pub fn is_filten_0(&self) -> bool {
    *self == FILTEN_A::FILTEN_0
  }
  #[doc = "Checks if the value of the field is `FILTEN_1`"]
  #[inline(always)]
  pub fn is_filten_1(&self) -> bool {
    *self == FILTEN_A::FILTEN_1
  }
}
#[doc = "Write proxy for field `FILTEN`"]
pub struct FILTEN_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Slow clock reset pin filter disabled."]
  #[inline(always)]
  pub fn filten_0(self) -> &'a mut W {
    self.variant(FILTEN_A::FILTEN_0)
  }
  #[doc = "Slow clock reset pin filter enabled in Run modes."]
  #[inline(always)]
  pub fn filten_1(self) -> &'a mut W {
    self.variant(FILTEN_A::FILTEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
    self.w
  }
}
#[doc = "LPO Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOFEN_A {
  #[doc = "0: LPO clock reset pin filter disabled."]
  LPOFEN_0,
  #[doc = "1: LPO clock reset pin filter enabled in all modes."]
  LPOFEN_1,
}
impl From<LPOFEN_A> for bool {
  #[inline(always)]
  fn from(variant: LPOFEN_A) -> Self {
    match variant {
      LPOFEN_A::LPOFEN_0 => false,
      LPOFEN_A::LPOFEN_1 => true,
    }
  }
}
#[doc = "Reader of field `LPOFEN`"]
pub type LPOFEN_R = crate::R<bool, LPOFEN_A>;
impl LPOFEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPOFEN_A {
    match self.bits {
      false => LPOFEN_A::LPOFEN_0,
      true => LPOFEN_A::LPOFEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPOFEN_0`"]
  #[inline(always)]
  pub fn is_lpofen_0(&self) -> bool {
    *self == LPOFEN_A::LPOFEN_0
  }
  #[doc = "Checks if the value of the field is `LPOFEN_1`"]
  #[inline(always)]
  pub fn is_lpofen_1(&self) -> bool {
    *self == LPOFEN_A::LPOFEN_1
  }
}
#[doc = "Write proxy for field `LPOFEN`"]
pub struct LPOFEN_W<'a> {
  w: &'a mut W,
}
impl<'a> LPOFEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPOFEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPO clock reset pin filter disabled."]
  #[inline(always)]
  pub fn lpofen_0(self) -> &'a mut W {
    self.variant(LPOFEN_A::LPOFEN_0)
  }
  #[doc = "LPO clock reset pin filter enabled in all modes."]
  #[inline(always)]
  pub fn lpofen_1(self) -> &'a mut W {
    self.variant(LPOFEN_A::LPOFEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:4 - Reset Filter Configuration"]
  #[inline(always)]
  pub fn filtcfg(&self) -> FILTCFG_R {
    FILTCFG_R::new((self.bits & 0x1f) as u8)
  }
  #[doc = "Bit 8 - Filter Enable"]
  #[inline(always)]
  pub fn filten(&self) -> FILTEN_R {
    FILTEN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - LPO Filter Enable"]
  #[inline(always)]
  pub fn lpofen(&self) -> LPOFEN_R {
    LPOFEN_R::new(((self.bits >> 9) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:4 - Reset Filter Configuration"]
  #[inline(always)]
  pub fn filtcfg(&mut self) -> FILTCFG_W {
    FILTCFG_W { w: self }
  }
  #[doc = "Bit 8 - Filter Enable"]
  #[inline(always)]
  pub fn filten(&mut self) -> FILTEN_W {
    FILTEN_W { w: self }
  }
  #[doc = "Bit 9 - LPO Filter Enable"]
  #[inline(always)]
  pub fn lpofen(&mut self) -> LPOFEN_W {
    LPOFEN_W { w: self }
  }
}
