#[doc = "Reader of register CLK_RECOVER_INT_EN"]
pub type R = crate::R<u8, super::CLK_RECOVER_INT_EN>;
#[doc = "Writer for register CLK_RECOVER_INT_EN"]
pub type W = crate::W<u8, super::CLK_RECOVER_INT_EN>;
#[doc = "Register CLK_RECOVER_INT_EN `reset()`'s with value 0x10"]
impl crate::ResetValue for super::CLK_RECOVER_INT_EN {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x10
  }
}
#[doc = "OVF_ERROR_EN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_ERROR_EN_A {
  #[doc = "0: The interrupt will be masked"]
  OVF_ERROR_EN_0,
  #[doc = "1: The interrupt will be enabled (default)"]
  OVF_ERROR_EN_1,
}
impl From<OVF_ERROR_EN_A> for bool {
  #[inline(always)]
  fn from(variant: OVF_ERROR_EN_A) -> Self {
    match variant {
      OVF_ERROR_EN_A::OVF_ERROR_EN_0 => false,
      OVF_ERROR_EN_A::OVF_ERROR_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `OVF_ERROR_EN`"]
pub type OVF_ERROR_EN_R = crate::R<bool, OVF_ERROR_EN_A>;
impl OVF_ERROR_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OVF_ERROR_EN_A {
    match self.bits {
      false => OVF_ERROR_EN_A::OVF_ERROR_EN_0,
      true => OVF_ERROR_EN_A::OVF_ERROR_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `OVF_ERROR_EN_0`"]
  #[inline(always)]
  pub fn is_ovf_error_en_0(&self) -> bool {
    *self == OVF_ERROR_EN_A::OVF_ERROR_EN_0
  }
  #[doc = "Checks if the value of the field is `OVF_ERROR_EN_1`"]
  #[inline(always)]
  pub fn is_ovf_error_en_1(&self) -> bool {
    *self == OVF_ERROR_EN_A::OVF_ERROR_EN_1
  }
}
#[doc = "Write proxy for field `OVF_ERROR_EN`"]
pub struct OVF_ERROR_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> OVF_ERROR_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OVF_ERROR_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The interrupt will be masked"]
  #[inline(always)]
  pub fn ovf_error_en_0(self) -> &'a mut W {
    self.variant(OVF_ERROR_EN_A::OVF_ERROR_EN_0)
  }
  #[doc = "The interrupt will be enabled (default)"]
  #[inline(always)]
  pub fn ovf_error_en_1(self) -> &'a mut W {
    self.variant(OVF_ERROR_EN_A::OVF_ERROR_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
    self.w
  }
}
impl R {
  #[doc = "Bit 4 - OVF_ERROR_EN"]
  #[inline(always)]
  pub fn ovf_error_en(&self) -> OVF_ERROR_EN_R {
    OVF_ERROR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - OVF_ERROR_EN"]
  #[inline(always)]
  pub fn ovf_error_en(&mut self) -> OVF_ERROR_EN_W {
    OVF_ERROR_EN_W { w: self }
  }
}
