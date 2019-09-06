#[doc = "Reader of register DERR_W1_0"]
pub type R = crate::R<u32, super::DERR_W1_0>;
#[doc = "Reader of field `EDID`"]
pub type EDID_R = crate::R<u8, u8>;
#[doc = "Error attributes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EATR_A {
  #[doc = "0: Secure user mode, instruction fetch access."]
  EATR_0,
  #[doc = "1: Secure user mode, data access."]
  EATR_1,
  #[doc = "2: Secure privileged mode, instruction fetch access."]
  EATR_2,
  #[doc = "3: Secure privileged mode, data access."]
  EATR_3,
  #[doc = "4: Nonsecure user mode, instruction fetch access."]
  EATR_4,
  #[doc = "5: Nonsecure user mode, data access."]
  EATR_5,
  #[doc = "6: Nonsecure privileged mode, instruction fetch access."]
  EATR_6,
  #[doc = "7: Nonsecure privileged mode, data access."]
  EATR_7,
}
impl From<EATR_A> for u8 {
  #[inline(always)]
  fn from(variant: EATR_A) -> Self {
    match variant {
      EATR_A::EATR_0 => 0,
      EATR_A::EATR_1 => 1,
      EATR_A::EATR_2 => 2,
      EATR_A::EATR_3 => 3,
      EATR_A::EATR_4 => 4,
      EATR_A::EATR_5 => 5,
      EATR_A::EATR_6 => 6,
      EATR_A::EATR_7 => 7,
    }
  }
}
#[doc = "Reader of field `EATR`"]
pub type EATR_R = crate::R<u8, EATR_A>;
impl EATR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EATR_A {
    match self.bits {
      0 => EATR_A::EATR_0,
      1 => EATR_A::EATR_1,
      2 => EATR_A::EATR_2,
      3 => EATR_A::EATR_3,
      4 => EATR_A::EATR_4,
      5 => EATR_A::EATR_5,
      6 => EATR_A::EATR_6,
      7 => EATR_A::EATR_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `EATR_0`"]
  #[inline(always)]
  pub fn is_eatr_0(&self) -> bool {
    *self == EATR_A::EATR_0
  }
  #[doc = "Checks if the value of the field is `EATR_1`"]
  #[inline(always)]
  pub fn is_eatr_1(&self) -> bool {
    *self == EATR_A::EATR_1
  }
  #[doc = "Checks if the value of the field is `EATR_2`"]
  #[inline(always)]
  pub fn is_eatr_2(&self) -> bool {
    *self == EATR_A::EATR_2
  }
  #[doc = "Checks if the value of the field is `EATR_3`"]
  #[inline(always)]
  pub fn is_eatr_3(&self) -> bool {
    *self == EATR_A::EATR_3
  }
  #[doc = "Checks if the value of the field is `EATR_4`"]
  #[inline(always)]
  pub fn is_eatr_4(&self) -> bool {
    *self == EATR_A::EATR_4
  }
  #[doc = "Checks if the value of the field is `EATR_5`"]
  #[inline(always)]
  pub fn is_eatr_5(&self) -> bool {
    *self == EATR_A::EATR_5
  }
  #[doc = "Checks if the value of the field is `EATR_6`"]
  #[inline(always)]
  pub fn is_eatr_6(&self) -> bool {
    *self == EATR_A::EATR_6
  }
  #[doc = "Checks if the value of the field is `EATR_7`"]
  #[inline(always)]
  pub fn is_eatr_7(&self) -> bool {
    *self == EATR_A::EATR_7
  }
}
#[doc = "Error read/write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERW_A {
  #[doc = "0: Read access"]
  ERW_0,
  #[doc = "1: Write access"]
  ERW_1,
}
impl From<ERW_A> for bool {
  #[inline(always)]
  fn from(variant: ERW_A) -> Self {
    match variant {
      ERW_A::ERW_0 => false,
      ERW_A::ERW_1 => true,
    }
  }
}
#[doc = "Reader of field `ERW`"]
pub type ERW_R = crate::R<bool, ERW_A>;
impl ERW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ERW_A {
    match self.bits {
      false => ERW_A::ERW_0,
      true => ERW_A::ERW_1,
    }
  }
  #[doc = "Checks if the value of the field is `ERW_0`"]
  #[inline(always)]
  pub fn is_erw_0(&self) -> bool {
    *self == ERW_A::ERW_0
  }
  #[doc = "Checks if the value of the field is `ERW_1`"]
  #[inline(always)]
  pub fn is_erw_1(&self) -> bool {
    *self == ERW_A::ERW_1
  }
}
#[doc = "Reader of field `EPORT`"]
pub type EPORT_R = crate::R<u8, u8>;
#[doc = "Error state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EST_A {
  #[doc = "0: No access violation has been detected."]
  EST_0,
  #[doc = "1: No access violation has been detected."]
  EST_1,
  #[doc = "2: A single access violation has been detected."]
  EST_2,
  #[doc = "3: Multiple access violations for this domain have been detected by this submodule instance. Only the address and attribute information for the first error have been captured in DERR_W0_i and DERR_W1_i."]
  EST_3,
}
impl From<EST_A> for u8 {
  #[inline(always)]
  fn from(variant: EST_A) -> Self {
    match variant {
      EST_A::EST_0 => 0,
      EST_A::EST_1 => 1,
      EST_A::EST_2 => 2,
      EST_A::EST_3 => 3,
    }
  }
}
#[doc = "Reader of field `EST`"]
pub type EST_R = crate::R<u8, EST_A>;
impl EST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EST_A {
    match self.bits {
      0 => EST_A::EST_0,
      1 => EST_A::EST_1,
      2 => EST_A::EST_2,
      3 => EST_A::EST_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `EST_0`"]
  #[inline(always)]
  pub fn is_est_0(&self) -> bool {
    *self == EST_A::EST_0
  }
  #[doc = "Checks if the value of the field is `EST_1`"]
  #[inline(always)]
  pub fn is_est_1(&self) -> bool {
    *self == EST_A::EST_1
  }
  #[doc = "Checks if the value of the field is `EST_2`"]
  #[inline(always)]
  pub fn is_est_2(&self) -> bool {
    *self == EST_A::EST_2
  }
  #[doc = "Checks if the value of the field is `EST_3`"]
  #[inline(always)]
  pub fn is_est_3(&self) -> bool {
    *self == EST_A::EST_3
  }
}
impl R {
  #[doc = "Bits 0:3 - Error domain identifier"]
  #[inline(always)]
  pub fn edid(&self) -> EDID_R {
    EDID_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 8:10 - Error attributes"]
  #[inline(always)]
  pub fn eatr(&self) -> EATR_R {
    EATR_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bit 11 - Error read/write"]
  #[inline(always)]
  pub fn erw(&self) -> ERW_R {
    ERW_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bits 24:26 - Error port"]
  #[inline(always)]
  pub fn eport(&self) -> EPORT_R {
    EPORT_R::new(((self.bits >> 24) & 0x07) as u8)
  }
  #[doc = "Bits 30:31 - Error state"]
  #[inline(always)]
  pub fn est(&self) -> EST_R {
    EST_R::new(((self.bits >> 30) & 0x03) as u8)
  }
}
