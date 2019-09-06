#[doc = "Reader of register PCC_EWM"]
pub type R = crate::R<u32, super::PCC_EWM>;
#[doc = "Writer for register PCC_EWM"]
pub type W = crate::W<u32, super::PCC_EWM>;
#[doc = "Register PCC_EWM `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::PCC_EWM {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x8000_0000
  }
}
#[doc = "In use flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INUSE_A {
  #[doc = "0: Peripheral is not being used."]
  INUSE_0,
  #[doc = "1: Peripheral is being used. Software cannot modify the existing clocking configuration."]
  INUSE_1,
}
impl From<INUSE_A> for bool {
  #[inline(always)]
  fn from(variant: INUSE_A) -> Self {
    match variant {
      INUSE_A::INUSE_0 => false,
      INUSE_A::INUSE_1 => true,
    }
  }
}
#[doc = "Reader of field `INUSE`"]
pub type INUSE_R = crate::R<bool, INUSE_A>;
impl INUSE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> INUSE_A {
    match self.bits {
      false => INUSE_A::INUSE_0,
      true => INUSE_A::INUSE_1,
    }
  }
  #[doc = "Checks if the value of the field is `INUSE_0`"]
  #[inline(always)]
  pub fn is_inuse_0(&self) -> bool {
    *self == INUSE_A::INUSE_0
  }
  #[doc = "Checks if the value of the field is `INUSE_1`"]
  #[inline(always)]
  pub fn is_inuse_1(&self) -> bool {
    *self == INUSE_A::INUSE_1
  }
}
#[doc = "Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGC_A {
  #[doc = "0: Clock disabled"]
  CGC_0,
  #[doc = "1: Clock enabled. The current clock selection and divider options are locked."]
  CGC_1,
}
impl From<CGC_A> for bool {
  #[inline(always)]
  fn from(variant: CGC_A) -> Self {
    match variant {
      CGC_A::CGC_0 => false,
      CGC_A::CGC_1 => true,
    }
  }
}
#[doc = "Reader of field `CGC`"]
pub type CGC_R = crate::R<bool, CGC_A>;
impl CGC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CGC_A {
    match self.bits {
      false => CGC_A::CGC_0,
      true => CGC_A::CGC_1,
    }
  }
  #[doc = "Checks if the value of the field is `CGC_0`"]
  #[inline(always)]
  pub fn is_cgc_0(&self) -> bool {
    *self == CGC_A::CGC_0
  }
  #[doc = "Checks if the value of the field is `CGC_1`"]
  #[inline(always)]
  pub fn is_cgc_1(&self) -> bool {
    *self == CGC_A::CGC_1
  }
}
#[doc = "Write proxy for field `CGC`"]
pub struct CGC_W<'a> {
  w: &'a mut W,
}
impl<'a> CGC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CGC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn cgc_0(self) -> &'a mut W {
    self.variant(CGC_A::CGC_0)
  }
  #[doc = "Clock enabled. The current clock selection and divider options are locked."]
  #[inline(always)]
  pub fn cgc_1(self) -> &'a mut W {
    self.variant(CGC_A::CGC_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
#[doc = "Present\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR_A {
  #[doc = "0: Peripheral is not present."]
  PR_0,
  #[doc = "1: Peripheral is present."]
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
impl R {
  #[doc = "Bit 29 - In use flag"]
  #[inline(always)]
  pub fn inuse(&self) -> INUSE_R {
    INUSE_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Clock Gate Control"]
  #[inline(always)]
  pub fn cgc(&self) -> CGC_R {
    CGC_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Present"]
  #[inline(always)]
  pub fn pr(&self) -> PR_R {
    PR_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 30 - Clock Gate Control"]
  #[inline(always)]
  pub fn cgc(&mut self) -> CGC_W {
    CGC_W { w: self }
  }
}
