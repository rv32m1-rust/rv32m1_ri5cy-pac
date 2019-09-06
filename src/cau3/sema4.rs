#[doc = "Reader of register SEMA4"]
pub type R = crate::R<u32, super::SEMA4>;
#[doc = "Writer for register SEMA4"]
pub type W = crate::W<u32, super::SEMA4>;
#[doc = "Register SEMA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMA4 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `DID`"]
pub type DID_R = crate::R<u8, u8>;
#[doc = "Privilege Attribute of Locked Semaphore Owner\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR_A {
  #[doc = "0: If semaphore is locked, then owner is operating in user mode"]
  PR_0,
  #[doc = "1: If semaphore is locked, then owner is operating in privileged mode"]
  PR_1,
}
impl From<PR_A> for bool {
  #[inline(always)]
  fn from(variant: PR_A) -> Self {
    match variant {
      PR_A::PR_0 => false,
      PR_A::PR_1 => true,
    }
  }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, PR_A>;
impl PR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PR_A {
    match self.bits {
      false => PR_A::PR_0,
      true => PR_A::PR_1,
    }
  }
  #[doc = "Checks if the value of the field is `PR_0`"]
  #[inline(always)]
  pub fn is_pr_0(&self) -> bool {
    *self == PR_A::PR_0
  }
  #[doc = "Checks if the value of the field is `PR_1`"]
  #[inline(always)]
  pub fn is_pr_1(&self) -> bool {
    *self == PR_A::PR_1
  }
}
#[doc = "Non Secure Attribute of the Locked Semaphore Owner\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NS_A {
  #[doc = "0: If semaphore is locked, owner is operating in secure mode"]
  NS_0,
  #[doc = "1: If semaphore is locked, owner is operating in nonsecure mode"]
  NS_1,
}
impl From<NS_A> for bool {
  #[inline(always)]
  fn from(variant: NS_A) -> Self {
    match variant {
      NS_A::NS_0 => false,
      NS_A::NS_1 => true,
    }
  }
}
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, NS_A>;
impl NS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> NS_A {
    match self.bits {
      false => NS_A::NS_0,
      true => NS_A::NS_1,
    }
  }
  #[doc = "Checks if the value of the field is `NS_0`"]
  #[inline(always)]
  pub fn is_ns_0(&self) -> bool {
    *self == NS_A::NS_0
  }
  #[doc = "Checks if the value of the field is `NS_1`"]
  #[inline(always)]
  pub fn is_ns_1(&self) -> bool {
    *self == NS_A::NS_1
  }
}
#[doc = "Reader of field `MSTRN`"]
pub type MSTRN_R = crate::R<u8, u8>;
#[doc = "Semaphore Lock and Release Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_AW {
  #[doc = "0: Semaphore release"]
  LK_0,
  #[doc = "1: Semaphore lock"]
  LK_1,
}
impl From<LK_AW> for bool {
  #[inline(always)]
  fn from(variant: LK_AW) -> Self {
    match variant {
      LK_AW::LK_0 => false,
      LK_AW::LK_1 => true,
    }
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Semaphore release"]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_AW::LK_0)
  }
  #[doc = "Semaphore lock"]
  #[inline(always)]
  pub fn lk_1(self) -> &'a mut W {
    self.variant(LK_AW::LK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:3 - Domain ID of Locked Semaphore Owner"]
  #[inline(always)]
  pub fn did(&self) -> DID_R {
    DID_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bit 6 - Privilege Attribute of Locked Semaphore Owner"]
  #[inline(always)]
  pub fn pr(&self) -> PR_R {
    PR_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Non Secure Attribute of the Locked Semaphore Owner"]
  #[inline(always)]
  pub fn ns(&self) -> NS_R {
    NS_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 8:13 - Master Number of Locked Semaphore Owner"]
  #[inline(always)]
  pub fn mstrn(&self) -> MSTRN_R {
    MSTRN_R::new(((self.bits >> 8) & 0x3f) as u8)
  }
}
impl W {
  #[doc = "Bit 31 - Semaphore Lock and Release Control"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
}
