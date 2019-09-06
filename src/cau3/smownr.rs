#[doc = "Reader of register SMOWNR"]
pub type R = crate::R<u32, super::SMOWNR>;
#[doc = "Semaphore Locked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
  #[doc = "0: Semaphore not locked"]
  LOCK_0,
  #[doc = "1: Semaphore locked"]
  LOCK_1,
}
impl From<LOCK_A> for bool {
  #[inline(always)]
  fn from(variant: LOCK_A) -> Self {
    match variant {
      LOCK_A::LOCK_0 => false,
      LOCK_A::LOCK_1 => true,
    }
  }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LOCK_A {
    match self.bits {
      false => LOCK_A::LOCK_0,
      true => LOCK_A::LOCK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LOCK_0`"]
  #[inline(always)]
  pub fn is_lock_0(&self) -> bool {
    *self == LOCK_A::LOCK_0
  }
  #[doc = "Checks if the value of the field is `LOCK_1`"]
  #[inline(always)]
  pub fn is_lock_1(&self) -> bool {
    *self == LOCK_A::LOCK_1
  }
}
#[doc = "Semaphore Ownership\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOWNER_A {
  #[doc = "0: The host making the current read access is the semaphore owner"]
  NOWNER_0,
  #[doc = "1: The host making the current read access is NOT the semaphore owner"]
  NOWNER_1,
}
impl From<NOWNER_A> for bool {
  #[inline(always)]
  fn from(variant: NOWNER_A) -> Self {
    match variant {
      NOWNER_A::NOWNER_0 => false,
      NOWNER_A::NOWNER_1 => true,
    }
  }
}
#[doc = "Reader of field `NOWNER`"]
pub type NOWNER_R = crate::R<bool, NOWNER_A>;
impl NOWNER_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> NOWNER_A {
    match self.bits {
      false => NOWNER_A::NOWNER_0,
      true => NOWNER_A::NOWNER_1,
    }
  }
  #[doc = "Checks if the value of the field is `NOWNER_0`"]
  #[inline(always)]
  pub fn is_nowner_0(&self) -> bool {
    *self == NOWNER_A::NOWNER_0
  }
  #[doc = "Checks if the value of the field is `NOWNER_1`"]
  #[inline(always)]
  pub fn is_nowner_1(&self) -> bool {
    *self == NOWNER_A::NOWNER_1
  }
}
impl R {
  #[doc = "Bit 0 - Semaphore Locked"]
  #[inline(always)]
  pub fn lock(&self) -> LOCK_R {
    LOCK_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 31 - Semaphore Ownership"]
  #[inline(always)]
  pub fn nowner(&self) -> NOWNER_R {
    NOWNER_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
