#[doc = "Reader of register COM"]
pub type R = crate::R<u32, super::COM>;
#[doc = "Writer for register COM"]
pub type W = crate::W<u32, super::COM>;
#[doc = "Register COM `reset()`'s with value 0"]
impl crate::ResetValue for super::COM {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reset All Internal Logic\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALL_AW {
  #[doc = "0: Do Not Reset"]
  NO_RESET,
  #[doc = "1: Reset PKHA engine and registers"]
  RESET_ALL,
}
impl From<ALL_AW> for bool {
  #[inline(always)]
  fn from(variant: ALL_AW) -> Self {
    match variant {
      ALL_AW::NO_RESET => false,
      ALL_AW::RESET_ALL => true,
    }
  }
}
#[doc = "Write proxy for field `ALL`"]
pub struct ALL_W<'a> {
  w: &'a mut W,
}
impl<'a> ALL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ALL_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Do Not Reset"]
  #[inline(always)]
  pub fn no_reset(self) -> &'a mut W {
    self.variant(ALL_AW::NO_RESET)
  }
  #[doc = "Reset PKHA engine and registers"]
  #[inline(always)]
  pub fn reset_all(self) -> &'a mut W {
    self.variant(ALL_AW::RESET_ALL)
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
#[doc = "Reset PKHA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PK_AW {
  #[doc = "0: Do Not Reset"]
  NO_RESET,
  #[doc = "1: Reset Public Key Hardware Accelerator"]
  RESET_PKHA,
}
impl From<PK_AW> for bool {
  #[inline(always)]
  fn from(variant: PK_AW) -> Self {
    match variant {
      PK_AW::NO_RESET => false,
      PK_AW::RESET_PKHA => true,
    }
  }
}
#[doc = "Write proxy for field `PK`"]
pub struct PK_W<'a> {
  w: &'a mut W,
}
impl<'a> PK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PK_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Do Not Reset"]
  #[inline(always)]
  pub fn no_reset(self) -> &'a mut W {
    self.variant(PK_AW::NO_RESET)
  }
  #[doc = "Reset Public Key Hardware Accelerator"]
  #[inline(always)]
  pub fn reset_pkha(self) -> &'a mut W {
    self.variant(PK_AW::RESET_PKHA)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
impl R {}
impl W {
  #[doc = "Bit 0 - Reset All Internal Logic"]
  #[inline(always)]
  pub fn all(&mut self) -> ALL_W {
    ALL_W { w: self }
  }
  #[doc = "Bit 6 - Reset PKHA"]
  #[inline(always)]
  pub fn pk(&mut self) -> PK_W {
    PK_W { w: self }
  }
}
