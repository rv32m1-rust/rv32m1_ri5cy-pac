#[doc = "Reader of register FCNFG"]
pub type R = crate::R<u8, super::FCNFG>;
#[doc = "Writer for register FCNFG"]
pub type W = crate::W<u8, super::FCNFG>;
#[doc = "Register FCNFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FCNFG {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x02
  }
}
#[doc = "RAM Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRDY_A {
  #[doc = "0: Programming acceleration RAM is not available"]
  RAMRDY_0,
  #[doc = "1: Programming acceleration RAM is available"]
  RAMRDY_1,
}
impl From<RAMRDY_A> for bool {
  #[inline(always)]
  fn from(variant: RAMRDY_A) -> Self {
    match variant {
      RAMRDY_A::RAMRDY_0 => false,
      RAMRDY_A::RAMRDY_1 => true,
    }
  }
}
#[doc = "Reader of field `RAMRDY`"]
pub type RAMRDY_R = crate::R<bool, RAMRDY_A>;
impl RAMRDY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RAMRDY_A {
    match self.bits {
      false => RAMRDY_A::RAMRDY_0,
      true => RAMRDY_A::RAMRDY_1,
    }
  }
  #[doc = "Checks if the value of the field is `RAMRDY_0`"]
  #[inline(always)]
  pub fn is_ramrdy_0(&self) -> bool {
    *self == RAMRDY_A::RAMRDY_0
  }
  #[doc = "Checks if the value of the field is `RAMRDY_1`"]
  #[inline(always)]
  pub fn is_ramrdy_1(&self) -> bool {
    *self == RAMRDY_A::RAMRDY_1
  }
}
#[doc = "CRC Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCRDY_A {
  #[doc = "0: Programming acceleration RAM is not available for CRC operations"]
  CRCRDY_0,
  #[doc = "1: Programming acceleration RAM is available for CRC operations"]
  CRCRDY_1,
}
impl From<CRCRDY_A> for bool {
  #[inline(always)]
  fn from(variant: CRCRDY_A) -> Self {
    match variant {
      CRCRDY_A::CRCRDY_0 => false,
      CRCRDY_A::CRCRDY_1 => true,
    }
  }
}
#[doc = "Reader of field `CRCRDY`"]
pub type CRCRDY_R = crate::R<bool, CRCRDY_A>;
impl CRCRDY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CRCRDY_A {
    match self.bits {
      false => CRCRDY_A::CRCRDY_0,
      true => CRCRDY_A::CRCRDY_1,
    }
  }
  #[doc = "Checks if the value of the field is `CRCRDY_0`"]
  #[inline(always)]
  pub fn is_crcrdy_0(&self) -> bool {
    *self == CRCRDY_A::CRCRDY_0
  }
  #[doc = "Checks if the value of the field is `CRCRDY_1`"]
  #[inline(always)]
  pub fn is_crcrdy_1(&self) -> bool {
    *self == CRCRDY_A::CRCRDY_1
  }
}
#[doc = "Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAP_A {
  #[doc = "0: Program flash 0 block is located at relative address 0x0000"]
  SWAP_0,
  #[doc = "1: Program flash 1 block is located at relative address 0x0000"]
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
#[doc = "Erase Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSSUSP_A {
  #[doc = "0: No suspend requested"]
  ERSSUSP_0,
  #[doc = "1: Suspend the current Erase Flash Sector command execution"]
  ERSSUSP_1,
}
impl From<ERSSUSP_A> for bool {
  #[inline(always)]
  fn from(variant: ERSSUSP_A) -> Self {
    match variant {
      ERSSUSP_A::ERSSUSP_0 => false,
      ERSSUSP_A::ERSSUSP_1 => true,
    }
  }
}
#[doc = "Reader of field `ERSSUSP`"]
pub type ERSSUSP_R = crate::R<bool, ERSSUSP_A>;
impl ERSSUSP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ERSSUSP_A {
    match self.bits {
      false => ERSSUSP_A::ERSSUSP_0,
      true => ERSSUSP_A::ERSSUSP_1,
    }
  }
  #[doc = "Checks if the value of the field is `ERSSUSP_0`"]
  #[inline(always)]
  pub fn is_erssusp_0(&self) -> bool {
    *self == ERSSUSP_A::ERSSUSP_0
  }
  #[doc = "Checks if the value of the field is `ERSSUSP_1`"]
  #[inline(always)]
  pub fn is_erssusp_1(&self) -> bool {
    *self == ERSSUSP_A::ERSSUSP_1
  }
}
#[doc = "Write proxy for field `ERSSUSP`"]
pub struct ERSSUSP_W<'a> {
  w: &'a mut W,
}
impl<'a> ERSSUSP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ERSSUSP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No suspend requested"]
  #[inline(always)]
  pub fn erssusp_0(self) -> &'a mut W {
    self.variant(ERSSUSP_A::ERSSUSP_0)
  }
  #[doc = "Suspend the current Erase Flash Sector command execution"]
  #[inline(always)]
  pub fn erssusp_1(self) -> &'a mut W {
    self.variant(ERSSUSP_A::ERSSUSP_1)
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
#[doc = "Erase All Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQ_A {
  #[doc = "0: No request or request complete"]
  ERSAREQ_0,
  #[doc = "1: Request to: (1) run the Erase All Blocks command, (2) verify the erased state, (3) program the security byte in the Flash Configuration Field to the unsecure state, and (4) release MCU security by setting the FSEC\\[SEC\\] field to the unsecure state."]
  ERSAREQ_1,
}
impl From<ERSAREQ_A> for bool {
  #[inline(always)]
  fn from(variant: ERSAREQ_A) -> Self {
    match variant {
      ERSAREQ_A::ERSAREQ_0 => false,
      ERSAREQ_A::ERSAREQ_1 => true,
    }
  }
}
#[doc = "Reader of field `ERSAREQ`"]
pub type ERSAREQ_R = crate::R<bool, ERSAREQ_A>;
impl ERSAREQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ERSAREQ_A {
    match self.bits {
      false => ERSAREQ_A::ERSAREQ_0,
      true => ERSAREQ_A::ERSAREQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `ERSAREQ_0`"]
  #[inline(always)]
  pub fn is_ersareq_0(&self) -> bool {
    *self == ERSAREQ_A::ERSAREQ_0
  }
  #[doc = "Checks if the value of the field is `ERSAREQ_1`"]
  #[inline(always)]
  pub fn is_ersareq_1(&self) -> bool {
    *self == ERSAREQ_A::ERSAREQ_1
  }
}
#[doc = "Read Collision Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLLIE_A {
  #[doc = "0: Read collision error interrupt disabled"]
  RDCOLLIE_0,
  #[doc = "1: Read collision error interrupt enabled. An interrupt request is generated whenever a flash read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
  RDCOLLIE_1,
}
impl From<RDCOLLIE_A> for bool {
  #[inline(always)]
  fn from(variant: RDCOLLIE_A) -> Self {
    match variant {
      RDCOLLIE_A::RDCOLLIE_0 => false,
      RDCOLLIE_A::RDCOLLIE_1 => true,
    }
  }
}
#[doc = "Reader of field `RDCOLLIE`"]
pub type RDCOLLIE_R = crate::R<bool, RDCOLLIE_A>;
impl RDCOLLIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDCOLLIE_A {
    match self.bits {
      false => RDCOLLIE_A::RDCOLLIE_0,
      true => RDCOLLIE_A::RDCOLLIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDCOLLIE_0`"]
  #[inline(always)]
  pub fn is_rdcollie_0(&self) -> bool {
    *self == RDCOLLIE_A::RDCOLLIE_0
  }
  #[doc = "Checks if the value of the field is `RDCOLLIE_1`"]
  #[inline(always)]
  pub fn is_rdcollie_1(&self) -> bool {
    *self == RDCOLLIE_A::RDCOLLIE_1
  }
}
#[doc = "Write proxy for field `RDCOLLIE`"]
pub struct RDCOLLIE_W<'a> {
  w: &'a mut W,
}
impl<'a> RDCOLLIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RDCOLLIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Read collision error interrupt disabled"]
  #[inline(always)]
  pub fn rdcollie_0(self) -> &'a mut W {
    self.variant(RDCOLLIE_A::RDCOLLIE_0)
  }
  #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever a flash read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
  #[inline(always)]
  pub fn rdcollie_1(self) -> &'a mut W {
    self.variant(RDCOLLIE_A::RDCOLLIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
  #[doc = "0: Command complete interrupt disabled"]
  CCIE_0,
  #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
  CCIE_1,
}
impl From<CCIE_A> for bool {
  #[inline(always)]
  fn from(variant: CCIE_A) -> Self {
    match variant {
      CCIE_A::CCIE_0 => false,
      CCIE_A::CCIE_1 => true,
    }
  }
}
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, CCIE_A>;
impl CCIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CCIE_A {
    match self.bits {
      false => CCIE_A::CCIE_0,
      true => CCIE_A::CCIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `CCIE_0`"]
  #[inline(always)]
  pub fn is_ccie_0(&self) -> bool {
    *self == CCIE_A::CCIE_0
  }
  #[doc = "Checks if the value of the field is `CCIE_1`"]
  #[inline(always)]
  pub fn is_ccie_1(&self) -> bool {
    *self == CCIE_A::CCIE_1
  }
}
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
  w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CCIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Command complete interrupt disabled"]
  #[inline(always)]
  pub fn ccie_0(self) -> &'a mut W {
    self.variant(CCIE_A::CCIE_0)
  }
  #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
  #[inline(always)]
  pub fn ccie_1(self) -> &'a mut W {
    self.variant(CCIE_A::CCIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 1 - RAM Ready"]
  #[inline(always)]
  pub fn ramrdy(&self) -> RAMRDY_R {
    RAMRDY_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - CRC Ready"]
  #[inline(always)]
  pub fn crcrdy(&self) -> CRCRDY_R {
    CRCRDY_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Swap"]
  #[inline(always)]
  pub fn swap(&self) -> SWAP_R {
    SWAP_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Erase Suspend"]
  #[inline(always)]
  pub fn erssusp(&self) -> ERSSUSP_R {
    ERSSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Erase All Request"]
  #[inline(always)]
  pub fn ersareq(&self) -> ERSAREQ_R {
    ERSAREQ_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
  #[inline(always)]
  pub fn rdcollie(&self) -> RDCOLLIE_R {
    RDCOLLIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Command Complete Interrupt Enable"]
  #[inline(always)]
  pub fn ccie(&self) -> CCIE_R {
    CCIE_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - Erase Suspend"]
  #[inline(always)]
  pub fn erssusp(&mut self) -> ERSSUSP_W {
    ERSSUSP_W { w: self }
  }
  #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
  #[inline(always)]
  pub fn rdcollie(&mut self) -> RDCOLLIE_W {
    RDCOLLIE_W { w: self }
  }
  #[doc = "Bit 7 - Command Complete Interrupt Enable"]
  #[inline(always)]
  pub fn ccie(&mut self) -> CCIE_W {
    CCIE_W { w: self }
  }
}
