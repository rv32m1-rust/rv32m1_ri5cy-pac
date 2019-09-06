#[doc = "Reader of register CCR0"]
pub type R = crate::R<u32, super::CCR0>;
#[doc = "Writer for register CCR0"]
pub type W = crate::W<u32, super::CCR0>;
#[doc = "Register CCR0 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CCR0 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x02
  }
}
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_EN_A {
  #[doc = "0: Analog Comparator is disabled."]
  CMP_EN_0,
  #[doc = "1: Analog Comparator is enabled."]
  CMP_EN_1,
}
impl From<CMP_EN_A> for bool {
  #[inline(always)]
  fn from(variant: CMP_EN_A) -> Self {
    match variant {
      CMP_EN_A::CMP_EN_0 => false,
      CMP_EN_A::CMP_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `CMP_EN`"]
pub type CMP_EN_R = crate::R<bool, CMP_EN_A>;
impl CMP_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CMP_EN_A {
    match self.bits {
      false => CMP_EN_A::CMP_EN_0,
      true => CMP_EN_A::CMP_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CMP_EN_0`"]
  #[inline(always)]
  pub fn is_cmp_en_0(&self) -> bool {
    *self == CMP_EN_A::CMP_EN_0
  }
  #[doc = "Checks if the value of the field is `CMP_EN_1`"]
  #[inline(always)]
  pub fn is_cmp_en_1(&self) -> bool {
    *self == CMP_EN_A::CMP_EN_1
  }
}
#[doc = "Write proxy for field `CMP_EN`"]
pub struct CMP_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> CMP_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CMP_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Analog Comparator is disabled."]
  #[inline(always)]
  pub fn cmp_en_0(self) -> &'a mut W {
    self.variant(CMP_EN_A::CMP_EN_0)
  }
  #[doc = "Analog Comparator is enabled."]
  #[inline(always)]
  pub fn cmp_en_1(self) -> &'a mut W {
    self.variant(CMP_EN_A::CMP_EN_1)
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
#[doc = "Comparator Module STOP Mode Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_STOP_EN_A {
  #[doc = "0: Comparator is disabled in STOP modes regardless of CMP_EN."]
  CMP_STOP_EN_0,
  #[doc = "1: Comparator is enabled in STOP mode if CMP_EN is active"]
  CMP_STOP_EN_1,
}
impl From<CMP_STOP_EN_A> for bool {
  #[inline(always)]
  fn from(variant: CMP_STOP_EN_A) -> Self {
    match variant {
      CMP_STOP_EN_A::CMP_STOP_EN_0 => false,
      CMP_STOP_EN_A::CMP_STOP_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `CMP_STOP_EN`"]
pub type CMP_STOP_EN_R = crate::R<bool, CMP_STOP_EN_A>;
impl CMP_STOP_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CMP_STOP_EN_A {
    match self.bits {
      false => CMP_STOP_EN_A::CMP_STOP_EN_0,
      true => CMP_STOP_EN_A::CMP_STOP_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CMP_STOP_EN_0`"]
  #[inline(always)]
  pub fn is_cmp_stop_en_0(&self) -> bool {
    *self == CMP_STOP_EN_A::CMP_STOP_EN_0
  }
  #[doc = "Checks if the value of the field is `CMP_STOP_EN_1`"]
  #[inline(always)]
  pub fn is_cmp_stop_en_1(&self) -> bool {
    *self == CMP_STOP_EN_A::CMP_STOP_EN_1
  }
}
#[doc = "Write proxy for field `CMP_STOP_EN`"]
pub struct CMP_STOP_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> CMP_STOP_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CMP_STOP_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Comparator is disabled in STOP modes regardless of CMP_EN."]
  #[inline(always)]
  pub fn cmp_stop_en_0(self) -> &'a mut W {
    self.variant(CMP_STOP_EN_A::CMP_STOP_EN_0)
  }
  #[doc = "Comparator is enabled in STOP mode if CMP_EN is active"]
  #[inline(always)]
  pub fn cmp_stop_en_1(self) -> &'a mut W {
    self.variant(CMP_STOP_EN_A::CMP_STOP_EN_1)
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
impl R {
  #[doc = "Bit 0 - Comparator Module Enable"]
  #[inline(always)]
  pub fn cmp_en(&self) -> CMP_EN_R {
    CMP_EN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Comparator Module STOP Mode Enable"]
  #[inline(always)]
  pub fn cmp_stop_en(&self) -> CMP_STOP_EN_R {
    CMP_STOP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Comparator Module Enable"]
  #[inline(always)]
  pub fn cmp_en(&mut self) -> CMP_EN_W {
    CMP_EN_W { w: self }
  }
  #[doc = "Bit 1 - Comparator Module STOP Mode Enable"]
  #[inline(always)]
  pub fn cmp_stop_en(&mut self) -> CMP_STOP_EN_W {
    CMP_STOP_EN_W { w: self }
  }
}
