#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Comparator Flag Rising Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR_IE_A {
  #[doc = "0: CFR interrupt is disabled."]
  CFR_IE_0,
  #[doc = "1: CFR interrupt is enabled."]
  CFR_IE_1,
}
impl From<CFR_IE_A> for bool {
  #[inline(always)]
  fn from(variant: CFR_IE_A) -> Self {
    match variant {
      CFR_IE_A::CFR_IE_0 => false,
      CFR_IE_A::CFR_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `CFR_IE`"]
pub type CFR_IE_R = crate::R<bool, CFR_IE_A>;
impl CFR_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CFR_IE_A {
    match self.bits {
      false => CFR_IE_A::CFR_IE_0,
      true => CFR_IE_A::CFR_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `CFR_IE_0`"]
  #[inline(always)]
  pub fn is_cfr_ie_0(&self) -> bool {
    *self == CFR_IE_A::CFR_IE_0
  }
  #[doc = "Checks if the value of the field is `CFR_IE_1`"]
  #[inline(always)]
  pub fn is_cfr_ie_1(&self) -> bool {
    *self == CFR_IE_A::CFR_IE_1
  }
}
#[doc = "Write proxy for field `CFR_IE`"]
pub struct CFR_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> CFR_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CFR_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CFR interrupt is disabled."]
  #[inline(always)]
  pub fn cfr_ie_0(self) -> &'a mut W {
    self.variant(CFR_IE_A::CFR_IE_0)
  }
  #[doc = "CFR interrupt is enabled."]
  #[inline(always)]
  pub fn cfr_ie_1(self) -> &'a mut W {
    self.variant(CFR_IE_A::CFR_IE_1)
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
#[doc = "Comparator Flag Falling Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFF_IE_A {
  #[doc = "0: CFF interrupt is disabled."]
  CFF_IE_0,
  #[doc = "1: CFF interrupt is enabled."]
  CFF_IE_1,
}
impl From<CFF_IE_A> for bool {
  #[inline(always)]
  fn from(variant: CFF_IE_A) -> Self {
    match variant {
      CFF_IE_A::CFF_IE_0 => false,
      CFF_IE_A::CFF_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `CFF_IE`"]
pub type CFF_IE_R = crate::R<bool, CFF_IE_A>;
impl CFF_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CFF_IE_A {
    match self.bits {
      false => CFF_IE_A::CFF_IE_0,
      true => CFF_IE_A::CFF_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `CFF_IE_0`"]
  #[inline(always)]
  pub fn is_cff_ie_0(&self) -> bool {
    *self == CFF_IE_A::CFF_IE_0
  }
  #[doc = "Checks if the value of the field is `CFF_IE_1`"]
  #[inline(always)]
  pub fn is_cff_ie_1(&self) -> bool {
    *self == CFF_IE_A::CFF_IE_1
  }
}
#[doc = "Write proxy for field `CFF_IE`"]
pub struct CFF_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> CFF_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CFF_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CFF interrupt is disabled."]
  #[inline(always)]
  pub fn cff_ie_0(self) -> &'a mut W {
    self.variant(CFF_IE_A::CFF_IE_0)
  }
  #[doc = "CFF interrupt is enabled."]
  #[inline(always)]
  pub fn cff_ie_1(self) -> &'a mut W {
    self.variant(CFF_IE_A::CFF_IE_1)
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
  #[doc = "Bit 0 - Comparator Flag Rising Interrupt Enable"]
  #[inline(always)]
  pub fn cfr_ie(&self) -> CFR_IE_R {
    CFR_IE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Comparator Flag Falling Interrupt Enable"]
  #[inline(always)]
  pub fn cff_ie(&self) -> CFF_IE_R {
    CFF_IE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Comparator Flag Rising Interrupt Enable"]
  #[inline(always)]
  pub fn cfr_ie(&mut self) -> CFR_IE_W {
    CFR_IE_W { w: self }
  }
  #[doc = "Bit 1 - Comparator Flag Falling Interrupt Enable"]
  #[inline(always)]
  pub fn cff_ie(&mut self) -> CFF_IE_W {
    CFF_IE_W { w: self }
  }
}
