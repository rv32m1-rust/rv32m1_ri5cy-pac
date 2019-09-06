#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR_A {
  #[doc = "0: A rising edge has not been detected on COUT."]
  CFR_0,
  #[doc = "1: A rising edge on COUT has occurred."]
  CFR_1,
}
impl From<CFR_A> for bool {
  #[inline(always)]
  fn from(variant: CFR_A) -> Self {
    match variant {
      CFR_A::CFR_0 => false,
      CFR_A::CFR_1 => true,
    }
  }
}
#[doc = "Reader of field `CFR`"]
pub type CFR_R = crate::R<bool, CFR_A>;
impl CFR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CFR_A {
    match self.bits {
      false => CFR_A::CFR_0,
      true => CFR_A::CFR_1,
    }
  }
  #[doc = "Checks if the value of the field is `CFR_0`"]
  #[inline(always)]
  pub fn is_cfr_0(&self) -> bool {
    *self == CFR_A::CFR_0
  }
  #[doc = "Checks if the value of the field is `CFR_1`"]
  #[inline(always)]
  pub fn is_cfr_1(&self) -> bool {
    *self == CFR_A::CFR_1
  }
}
#[doc = "Write proxy for field `CFR`"]
pub struct CFR_W<'a> {
  w: &'a mut W,
}
impl<'a> CFR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CFR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "A rising edge has not been detected on COUT."]
  #[inline(always)]
  pub fn cfr_0(self) -> &'a mut W {
    self.variant(CFR_A::CFR_0)
  }
  #[doc = "A rising edge on COUT has occurred."]
  #[inline(always)]
  pub fn cfr_1(self) -> &'a mut W {
    self.variant(CFR_A::CFR_1)
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
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFF_A {
  #[doc = "0: A falling edge has not been detected on COUT."]
  CFF_0,
  #[doc = "1: A falling edge on COUT has occurred."]
  CFF_1,
}
impl From<CFF_A> for bool {
  #[inline(always)]
  fn from(variant: CFF_A) -> Self {
    match variant {
      CFF_A::CFF_0 => false,
      CFF_A::CFF_1 => true,
    }
  }
}
#[doc = "Reader of field `CFF`"]
pub type CFF_R = crate::R<bool, CFF_A>;
impl CFF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CFF_A {
    match self.bits {
      false => CFF_A::CFF_0,
      true => CFF_A::CFF_1,
    }
  }
  #[doc = "Checks if the value of the field is `CFF_0`"]
  #[inline(always)]
  pub fn is_cff_0(&self) -> bool {
    *self == CFF_A::CFF_0
  }
  #[doc = "Checks if the value of the field is `CFF_1`"]
  #[inline(always)]
  pub fn is_cff_1(&self) -> bool {
    *self == CFF_A::CFF_1
  }
}
#[doc = "Write proxy for field `CFF`"]
pub struct CFF_W<'a> {
  w: &'a mut W,
}
impl<'a> CFF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CFF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "A falling edge has not been detected on COUT."]
  #[inline(always)]
  pub fn cff_0(self) -> &'a mut W {
    self.variant(CFF_A::CFF_0)
  }
  #[doc = "A falling edge on COUT has occurred."]
  #[inline(always)]
  pub fn cff_1(self) -> &'a mut W {
    self.variant(CFF_A::CFF_1)
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
#[doc = "Reader of field `COUT`"]
pub type COUT_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 0 - Analog Comparator Flag Rising"]
  #[inline(always)]
  pub fn cfr(&self) -> CFR_R {
    CFR_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Analog Comparator Flag Falling"]
  #[inline(always)]
  pub fn cff(&self) -> CFF_R {
    CFF_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Analog Comparator Output"]
  #[inline(always)]
  pub fn cout(&self) -> COUT_R {
    COUT_R::new(((self.bits >> 8) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Analog Comparator Flag Rising"]
  #[inline(always)]
  pub fn cfr(&mut self) -> CFR_W {
    CFR_W { w: self }
  }
  #[doc = "Bit 1 - Analog Comparator Flag Falling"]
  #[inline(always)]
  pub fn cff(&mut self) -> CFF_W {
    CFF_W { w: self }
  }
}
