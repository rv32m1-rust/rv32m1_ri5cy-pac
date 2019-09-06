#[doc = "Reader of register STA"]
pub type R = crate::R<u32, super::STA>;
#[doc = "Writer for register STA"]
pub type W = crate::W<u32, super::STA>;
#[doc = "Register STA `reset()`'s with value 0"]
impl crate::ResetValue for super::STA {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "PKHA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PB_A {
  #[doc = "0: PKHA Idle"]
  PKHA_IDLE,
  #[doc = "1: PKHA Busy."]
  PKHA_BUSY,
}
impl From<PB_A> for bool {
  #[inline(always)]
  fn from(variant: PB_A) -> Self {
    match variant {
      PB_A::PKHA_IDLE => false,
      PB_A::PKHA_BUSY => true,
    }
  }
}
#[doc = "Reader of field `PB`"]
pub type PB_R = crate::R<bool, PB_A>;
impl PB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PB_A {
    match self.bits {
      false => PB_A::PKHA_IDLE,
      true => PB_A::PKHA_BUSY,
    }
  }
  #[doc = "Checks if the value of the field is `PKHA_IDLE`"]
  #[inline(always)]
  pub fn is_pkha_idle(&self) -> bool {
    *self == PB_A::PKHA_IDLE
  }
  #[doc = "Checks if the value of the field is `PKHA_BUSY`"]
  #[inline(always)]
  pub fn is_pkha_busy(&self) -> bool {
    *self == PB_A::PKHA_BUSY
  }
}
#[doc = "Reader of field `DI`"]
pub type DI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DI`"]
pub struct DI_W<'a> {
  w: &'a mut W,
}
impl<'a> DI_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EI_A {
  #[doc = "0: Not Error."]
  NOT_ERROR_INT,
  #[doc = "1: Error Interrupt."]
  ERROR_INT,
}
impl From<EI_A> for bool {
  #[inline(always)]
  fn from(variant: EI_A) -> Self {
    match variant {
      EI_A::NOT_ERROR_INT => false,
      EI_A::ERROR_INT => true,
    }
  }
}
#[doc = "Reader of field `EI`"]
pub type EI_R = crate::R<bool, EI_A>;
impl EI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EI_A {
    match self.bits {
      false => EI_A::NOT_ERROR_INT,
      true => EI_A::ERROR_INT,
    }
  }
  #[doc = "Checks if the value of the field is `NOT_ERROR_INT`"]
  #[inline(always)]
  pub fn is_not_error_int(&self) -> bool {
    *self == EI_A::NOT_ERROR_INT
  }
  #[doc = "Checks if the value of the field is `ERROR_INT`"]
  #[inline(always)]
  pub fn is_error_int(&self) -> bool {
    *self == EI_A::ERROR_INT
  }
}
#[doc = "Reader of field `PKP`"]
pub type PKP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKO`"]
pub type PKO_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKZ`"]
pub type PKZ_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 6 - PKHA Busy"]
  #[inline(always)]
  pub fn pb(&self) -> PB_R {
    PB_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Done Interrupt"]
  #[inline(always)]
  pub fn di(&self) -> DI_R {
    DI_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 20 - Error Interrupt"]
  #[inline(always)]
  pub fn ei(&self) -> EI_R {
    EI_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Public Key is Prime"]
  #[inline(always)]
  pub fn pkp(&self) -> PKP_R {
    PKP_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Public Key Operation is One"]
  #[inline(always)]
  pub fn pko(&self) -> PKO_R {
    PKO_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Public Key Operation is Zero"]
  #[inline(always)]
  pub fn pkz(&self) -> PKZ_R {
    PKZ_R::new(((self.bits >> 30) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 16 - Done Interrupt"]
  #[inline(always)]
  pub fn di(&mut self) -> DI_W {
    DI_W { w: self }
  }
}
