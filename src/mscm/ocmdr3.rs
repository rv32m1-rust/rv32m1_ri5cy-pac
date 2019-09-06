#[doc = "Reader of register OCMDR3"]
pub type R = crate::R<u32, super::OCMDR3>;
#[doc = "Writer for register OCMDR3"]
pub type W = crate::W<u32, super::OCMDR3>;
#[doc = "Register OCMDR3 `reset()`'s with value 0xd704_7000"]
impl crate::ResetValue for super::OCMDR3 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xd704_7000
  }
}
#[doc = "Reader of field `OCMPU`"]
pub type OCMPU_R = crate::R<bool, bool>;
#[doc = "OCMT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMT_A {
  #[doc = "3: OCMEMn is a ROM."]
  OCMT_3,
  #[doc = "4: OCMEMn is a Program Flash."]
  OCMT_4,
  #[doc = "6: OCMEMn is an EEE."]
  OCMT_6,
}
impl From<OCMT_A> for u8 {
  #[inline(always)]
  fn from(variant: OCMT_A) -> Self {
    match variant {
      OCMT_A::OCMT_3 => 3,
      OCMT_A::OCMT_4 => 4,
      OCMT_A::OCMT_6 => 6,
    }
  }
}
#[doc = "Reader of field `OCMT`"]
pub type OCMT_R = crate::R<u8, OCMT_A>;
impl OCMT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, OCMT_A> {
    use crate::Variant::*;
    match self.bits {
      3 => Val(OCMT_A::OCMT_3),
      4 => Val(OCMT_A::OCMT_4),
      6 => Val(OCMT_A::OCMT_6),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `OCMT_3`"]
  #[inline(always)]
  pub fn is_ocmt_3(&self) -> bool {
    *self == OCMT_A::OCMT_3
  }
  #[doc = "Checks if the value of the field is `OCMT_4`"]
  #[inline(always)]
  pub fn is_ocmt_4(&self) -> bool {
    *self == OCMT_A::OCMT_4
  }
  #[doc = "Checks if the value of the field is `OCMT_6`"]
  #[inline(always)]
  pub fn is_ocmt_6(&self) -> bool {
    *self == OCMT_A::OCMT_6
  }
}
#[doc = "RO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_A {
  #[doc = "0: Writes to the OCMDRn\\[11:0\\] are allowed"]
  RO_0,
  #[doc = "1: Writes to the OCMDRn\\[11:0\\] are ignored"]
  RO_1,
}
impl From<RO_A> for bool {
  #[inline(always)]
  fn from(variant: RO_A) -> Self {
    match variant {
      RO_A::RO_0 => false,
      RO_A::RO_1 => true,
    }
  }
}
#[doc = "Reader of field `RO`"]
pub type RO_R = crate::R<bool, RO_A>;
impl RO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RO_A {
    match self.bits {
      false => RO_A::RO_0,
      true => RO_A::RO_1,
    }
  }
  #[doc = "Checks if the value of the field is `RO_0`"]
  #[inline(always)]
  pub fn is_ro_0(&self) -> bool {
    *self == RO_A::RO_0
  }
  #[doc = "Checks if the value of the field is `RO_1`"]
  #[inline(always)]
  pub fn is_ro_1(&self) -> bool {
    *self == RO_A::RO_1
  }
}
#[doc = "Write proxy for field `RO`"]
pub struct RO_W<'a> {
  w: &'a mut W,
}
impl<'a> RO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RO_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the OCMDRn\\[11:0\\] are allowed"]
  #[inline(always)]
  pub fn ro_0(self) -> &'a mut W {
    self.variant(RO_A::RO_0)
  }
  #[doc = "Writes to the OCMDRn\\[11:0\\] are ignored"]
  #[inline(always)]
  pub fn ro_1(self) -> &'a mut W {
    self.variant(RO_A::RO_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "OCMW\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMW_A {
  #[doc = "2: OCMEMn 32-bits wide"]
  OCMW_2,
  #[doc = "3: OCMEMn 64-bits wide"]
  OCMW_3,
  #[doc = "4: OCMEMn 128-bits wide"]
  OCMW_4,
  #[doc = "5: OCMEMn 256-bits wide"]
  OCMW_5,
}
impl From<OCMW_A> for u8 {
  #[inline(always)]
  fn from(variant: OCMW_A) -> Self {
    match variant {
      OCMW_A::OCMW_2 => 2,
      OCMW_A::OCMW_3 => 3,
      OCMW_A::OCMW_4 => 4,
      OCMW_A::OCMW_5 => 5,
    }
  }
}
#[doc = "Reader of field `OCMW`"]
pub type OCMW_R = crate::R<u8, OCMW_A>;
impl OCMW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, OCMW_A> {
    use crate::Variant::*;
    match self.bits {
      2 => Val(OCMW_A::OCMW_2),
      3 => Val(OCMW_A::OCMW_3),
      4 => Val(OCMW_A::OCMW_4),
      5 => Val(OCMW_A::OCMW_5),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `OCMW_2`"]
  #[inline(always)]
  pub fn is_ocmw_2(&self) -> bool {
    *self == OCMW_A::OCMW_2
  }
  #[doc = "Checks if the value of the field is `OCMW_3`"]
  #[inline(always)]
  pub fn is_ocmw_3(&self) -> bool {
    *self == OCMW_A::OCMW_3
  }
  #[doc = "Checks if the value of the field is `OCMW_4`"]
  #[inline(always)]
  pub fn is_ocmw_4(&self) -> bool {
    *self == OCMW_A::OCMW_4
  }
  #[doc = "Checks if the value of the field is `OCMW_5`"]
  #[inline(always)]
  pub fn is_ocmw_5(&self) -> bool {
    *self == OCMW_A::OCMW_5
  }
}
#[doc = "OCMSZ\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMSZ_A {
  #[doc = "0: no OCMEMn"]
  OCMSZ_0,
  #[doc = "1: 1KB OCMEMn"]
  OCMSZ_1,
  #[doc = "2: 2KB OCMEMn"]
  OCMSZ_2,
  #[doc = "3: 4KB OCMEMn"]
  OCMSZ_3,
  #[doc = "4: 8KB OCMEMn"]
  OCMSZ_4,
  #[doc = "5: 16KB OCMEMn"]
  OCMSZ_5,
  #[doc = "6: 32KB OCMEMn"]
  OCMSZ_6,
  #[doc = "7: 64KB OCMEMn"]
  OCMSZ_7,
  #[doc = "8: 128KB OCMEMn"]
  OCMSZ_8,
  #[doc = "9: 256KB OCMEMn"]
  OCMSZ_9,
  #[doc = "10: 512KB OCMEMn"]
  OCMSZ_10,
  #[doc = "11: 1MB OCMEMn"]
  OCMSZ_11,
  #[doc = "12: 2MB OCMEMn"]
  OCMSZ_12,
  #[doc = "13: 4MB OCMEMn"]
  OCMSZ_13,
  #[doc = "14: 8MB OCMEMn"]
  OCMSZ_14,
  #[doc = "15: 16MB OCMEMn"]
  OCMSZ_15,
}
impl From<OCMSZ_A> for u8 {
  #[inline(always)]
  fn from(variant: OCMSZ_A) -> Self {
    match variant {
      OCMSZ_A::OCMSZ_0 => 0,
      OCMSZ_A::OCMSZ_1 => 1,
      OCMSZ_A::OCMSZ_2 => 2,
      OCMSZ_A::OCMSZ_3 => 3,
      OCMSZ_A::OCMSZ_4 => 4,
      OCMSZ_A::OCMSZ_5 => 5,
      OCMSZ_A::OCMSZ_6 => 6,
      OCMSZ_A::OCMSZ_7 => 7,
      OCMSZ_A::OCMSZ_8 => 8,
      OCMSZ_A::OCMSZ_9 => 9,
      OCMSZ_A::OCMSZ_10 => 10,
      OCMSZ_A::OCMSZ_11 => 11,
      OCMSZ_A::OCMSZ_12 => 12,
      OCMSZ_A::OCMSZ_13 => 13,
      OCMSZ_A::OCMSZ_14 => 14,
      OCMSZ_A::OCMSZ_15 => 15,
    }
  }
}
#[doc = "Reader of field `OCMSZ`"]
pub type OCMSZ_R = crate::R<u8, OCMSZ_A>;
impl OCMSZ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OCMSZ_A {
    match self.bits {
      0 => OCMSZ_A::OCMSZ_0,
      1 => OCMSZ_A::OCMSZ_1,
      2 => OCMSZ_A::OCMSZ_2,
      3 => OCMSZ_A::OCMSZ_3,
      4 => OCMSZ_A::OCMSZ_4,
      5 => OCMSZ_A::OCMSZ_5,
      6 => OCMSZ_A::OCMSZ_6,
      7 => OCMSZ_A::OCMSZ_7,
      8 => OCMSZ_A::OCMSZ_8,
      9 => OCMSZ_A::OCMSZ_9,
      10 => OCMSZ_A::OCMSZ_10,
      11 => OCMSZ_A::OCMSZ_11,
      12 => OCMSZ_A::OCMSZ_12,
      13 => OCMSZ_A::OCMSZ_13,
      14 => OCMSZ_A::OCMSZ_14,
      15 => OCMSZ_A::OCMSZ_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `OCMSZ_0`"]
  #[inline(always)]
  pub fn is_ocmsz_0(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_0
  }
  #[doc = "Checks if the value of the field is `OCMSZ_1`"]
  #[inline(always)]
  pub fn is_ocmsz_1(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_1
  }
  #[doc = "Checks if the value of the field is `OCMSZ_2`"]
  #[inline(always)]
  pub fn is_ocmsz_2(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_2
  }
  #[doc = "Checks if the value of the field is `OCMSZ_3`"]
  #[inline(always)]
  pub fn is_ocmsz_3(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_3
  }
  #[doc = "Checks if the value of the field is `OCMSZ_4`"]
  #[inline(always)]
  pub fn is_ocmsz_4(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_4
  }
  #[doc = "Checks if the value of the field is `OCMSZ_5`"]
  #[inline(always)]
  pub fn is_ocmsz_5(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_5
  }
  #[doc = "Checks if the value of the field is `OCMSZ_6`"]
  #[inline(always)]
  pub fn is_ocmsz_6(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_6
  }
  #[doc = "Checks if the value of the field is `OCMSZ_7`"]
  #[inline(always)]
  pub fn is_ocmsz_7(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_7
  }
  #[doc = "Checks if the value of the field is `OCMSZ_8`"]
  #[inline(always)]
  pub fn is_ocmsz_8(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_8
  }
  #[doc = "Checks if the value of the field is `OCMSZ_9`"]
  #[inline(always)]
  pub fn is_ocmsz_9(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_9
  }
  #[doc = "Checks if the value of the field is `OCMSZ_10`"]
  #[inline(always)]
  pub fn is_ocmsz_10(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_10
  }
  #[doc = "Checks if the value of the field is `OCMSZ_11`"]
  #[inline(always)]
  pub fn is_ocmsz_11(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_11
  }
  #[doc = "Checks if the value of the field is `OCMSZ_12`"]
  #[inline(always)]
  pub fn is_ocmsz_12(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_12
  }
  #[doc = "Checks if the value of the field is `OCMSZ_13`"]
  #[inline(always)]
  pub fn is_ocmsz_13(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_13
  }
  #[doc = "Checks if the value of the field is `OCMSZ_14`"]
  #[inline(always)]
  pub fn is_ocmsz_14(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_14
  }
  #[doc = "Checks if the value of the field is `OCMSZ_15`"]
  #[inline(always)]
  pub fn is_ocmsz_15(&self) -> bool {
    *self == OCMSZ_A::OCMSZ_15
  }
}
#[doc = "OCMSZH\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMSZH_A {
  #[doc = "0: OCMEMn is a power-of-2 capacity."]
  OCMSZH_0,
  #[doc = "1: OCMEMn is not a power-of-2, with a capacity is 0.75 * OCMSZ."]
  OCMSZH_1,
}
impl From<OCMSZH_A> for bool {
  #[inline(always)]
  fn from(variant: OCMSZH_A) -> Self {
    match variant {
      OCMSZH_A::OCMSZH_0 => false,
      OCMSZH_A::OCMSZH_1 => true,
    }
  }
}
#[doc = "Reader of field `OCMSZH`"]
pub type OCMSZH_R = crate::R<bool, OCMSZH_A>;
impl OCMSZH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OCMSZH_A {
    match self.bits {
      false => OCMSZH_A::OCMSZH_0,
      true => OCMSZH_A::OCMSZH_1,
    }
  }
  #[doc = "Checks if the value of the field is `OCMSZH_0`"]
  #[inline(always)]
  pub fn is_ocmszh_0(&self) -> bool {
    *self == OCMSZH_A::OCMSZH_0
  }
  #[doc = "Checks if the value of the field is `OCMSZH_1`"]
  #[inline(always)]
  pub fn is_ocmszh_1(&self) -> bool {
    *self == OCMSZH_A::OCMSZH_1
  }
}
#[doc = "V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
  #[doc = "0: OCMEMn is not present."]
  V_0,
  #[doc = "1: OCMEMn is present."]
  V_1,
}
impl From<V_A> for bool {
  #[inline(always)]
  fn from(variant: V_A) -> Self {
    match variant {
      V_A::V_0 => false,
      V_A::V_1 => true,
    }
  }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> V_A {
    match self.bits {
      false => V_A::V_0,
      true => V_A::V_1,
    }
  }
  #[doc = "Checks if the value of the field is `V_0`"]
  #[inline(always)]
  pub fn is_v_0(&self) -> bool {
    *self == V_A::V_0
  }
  #[doc = "Checks if the value of the field is `V_1`"]
  #[inline(always)]
  pub fn is_v_1(&self) -> bool {
    *self == V_A::V_1
  }
}
impl R {
  #[doc = "Bit 12 - OCMPU"]
  #[inline(always)]
  pub fn ocmpu(&self) -> OCMPU_R {
    OCMPU_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bits 13:15 - OCMT"]
  #[inline(always)]
  pub fn ocmt(&self) -> OCMT_R {
    OCMT_R::new(((self.bits >> 13) & 0x07) as u8)
  }
  #[doc = "Bit 16 - RO"]
  #[inline(always)]
  pub fn ro(&self) -> RO_R {
    RO_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bits 17:19 - OCMW"]
  #[inline(always)]
  pub fn ocmw(&self) -> OCMW_R {
    OCMW_R::new(((self.bits >> 17) & 0x07) as u8)
  }
  #[doc = "Bits 24:27 - OCMSZ"]
  #[inline(always)]
  pub fn ocmsz(&self) -> OCMSZ_R {
    OCMSZ_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bit 28 - OCMSZH"]
  #[inline(always)]
  pub fn ocmszh(&self) -> OCMSZH_R {
    OCMSZH_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 31 - V"]
  #[inline(always)]
  pub fn v(&self) -> V_R {
    V_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 16 - RO"]
  #[inline(always)]
  pub fn ro(&mut self) -> RO_W {
    RO_W { w: self }
  }
}
