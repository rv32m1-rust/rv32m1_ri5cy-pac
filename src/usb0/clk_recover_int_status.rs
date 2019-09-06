#[doc = "Reader of register CLK_RECOVER_INT_STATUS"]
pub type R = crate::R<u8, super::CLK_RECOVER_INT_STATUS>;
#[doc = "Writer for register CLK_RECOVER_INT_STATUS"]
pub type W = crate::W<u8, super::CLK_RECOVER_INT_STATUS>;
#[doc = "Register CLK_RECOVER_INT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_RECOVER_INT_STATUS {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "OVF_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_ERROR_A {
  #[doc = "0: No interrupt is reported"]
  OVF_ERROR_0,
  #[doc = "1: Unmasked interrupt has been generated"]
  OVF_ERROR_1,
}
impl From<OVF_ERROR_A> for bool {
  #[inline(always)]
  fn from(variant: OVF_ERROR_A) -> Self {
    match variant {
      OVF_ERROR_A::OVF_ERROR_0 => false,
      OVF_ERROR_A::OVF_ERROR_1 => true,
    }
  }
}
#[doc = "Reader of field `OVF_ERROR`"]
pub type OVF_ERROR_R = crate::R<bool, OVF_ERROR_A>;
impl OVF_ERROR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OVF_ERROR_A {
    match self.bits {
      false => OVF_ERROR_A::OVF_ERROR_0,
      true => OVF_ERROR_A::OVF_ERROR_1,
    }
  }
  #[doc = "Checks if the value of the field is `OVF_ERROR_0`"]
  #[inline(always)]
  pub fn is_ovf_error_0(&self) -> bool {
    *self == OVF_ERROR_A::OVF_ERROR_0
  }
  #[doc = "Checks if the value of the field is `OVF_ERROR_1`"]
  #[inline(always)]
  pub fn is_ovf_error_1(&self) -> bool {
    *self == OVF_ERROR_A::OVF_ERROR_1
  }
}
#[doc = "Write proxy for field `OVF_ERROR`"]
pub struct OVF_ERROR_W<'a> {
  w: &'a mut W,
}
impl<'a> OVF_ERROR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OVF_ERROR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No interrupt is reported"]
  #[inline(always)]
  pub fn ovf_error_0(self) -> &'a mut W {
    self.variant(OVF_ERROR_A::OVF_ERROR_0)
  }
  #[doc = "Unmasked interrupt has been generated"]
  #[inline(always)]
  pub fn ovf_error_1(self) -> &'a mut W {
    self.variant(OVF_ERROR_A::OVF_ERROR_1)
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
  #[doc = "Bit 4 - OVF_ERROR"]
  #[inline(always)]
  pub fn ovf_error(&self) -> OVF_ERROR_R {
    OVF_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - OVF_ERROR"]
  #[inline(always)]
  pub fn ovf_error(&mut self) -> OVF_ERROR_W {
    OVF_ERROR_W { w: self }
  }
}
