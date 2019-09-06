#[doc = "Reader of register FCFG2"]
pub type R = crate::R<u32, super::FCFG2>;
#[doc = "Reader of field `MAXADDR2`"]
pub type MAXADDR2_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAXADDR01`"]
pub type MAXADDR01_R = crate::R<u8, u8>;
#[doc = "SWAP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAP_A {
  #[doc = "0: Logical P-flash Block 0 is located at relative address 0x0000"]
  SWAP_0,
  #[doc = "1: Logical P-flash Block 1 is located at relative address 0x0000"]
  SWAP_1,
}
impl From<SWAP_A> for bool {
  #[inline(always)]
  fn from(variant: SWAP_A) -> Self {
    match variant {
      SWAP_A::SWAP_0 => false,
      SWAP_A::SWAP_1 => true,
    }
  }
}
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, SWAP_A>;
impl SWAP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWAP_A {
    match self.bits {
      false => SWAP_A::SWAP_0,
      true => SWAP_A::SWAP_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWAP_0`"]
  #[inline(always)]
  pub fn is_swap_0(&self) -> bool {
    *self == SWAP_A::SWAP_0
  }
  #[doc = "Checks if the value of the field is `SWAP_1`"]
  #[inline(always)]
  pub fn is_swap_1(&self) -> bool {
    *self == SWAP_A::SWAP_1
  }
}
impl R {
  #[doc = "Bits 16:21 - Max Address lock"]
  #[inline(always)]
  pub fn maxaddr2(&self) -> MAXADDR2_R {
    MAXADDR2_R::new(((self.bits >> 16) & 0x3f) as u8)
  }
  #[doc = "Bits 24:30 - Max Address lock"]
  #[inline(always)]
  pub fn maxaddr01(&self) -> MAXADDR01_R {
    MAXADDR01_R::new(((self.bits >> 24) & 0x7f) as u8)
  }
  #[doc = "Bit 31 - SWAP"]
  #[inline(always)]
  pub fn swap(&self) -> SWAP_R {
    SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
