#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Slow Clock Divide Ratio\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSLOW_A {
  #[doc = "1: Divide-by-2"]
  DIVSLOW_1,
  #[doc = "2: Divide-by-3"]
  DIVSLOW_2,
  #[doc = "3: Divide-by-4"]
  DIVSLOW_3,
  #[doc = "4: Divide-by-5"]
  DIVSLOW_4,
  #[doc = "5: Divide-by-6"]
  DIVSLOW_5,
  #[doc = "6: Divide-by-7"]
  DIVSLOW_6,
  #[doc = "7: Divide-by-8"]
  DIVSLOW_7,
  #[doc = "8: no description available"]
  DIVSLOW_8,
  #[doc = "9: no description available"]
  DIVSLOW_9,
  #[doc = "10: no description available"]
  DIVSLOW_10,
  #[doc = "11: no description available"]
  DIVSLOW_11,
  #[doc = "12: no description available"]
  DIVSLOW_12,
  #[doc = "13: no description available"]
  DIVSLOW_13,
  #[doc = "14: no description available"]
  DIVSLOW_14,
  #[doc = "15: no description available"]
  DIVSLOW_15,
}
impl From<DIVSLOW_A> for u8 {
  #[inline(always)]
  fn from(variant: DIVSLOW_A) -> Self {
    match variant {
      DIVSLOW_A::DIVSLOW_1 => 1,
      DIVSLOW_A::DIVSLOW_2 => 2,
      DIVSLOW_A::DIVSLOW_3 => 3,
      DIVSLOW_A::DIVSLOW_4 => 4,
      DIVSLOW_A::DIVSLOW_5 => 5,
      DIVSLOW_A::DIVSLOW_6 => 6,
      DIVSLOW_A::DIVSLOW_7 => 7,
      DIVSLOW_A::DIVSLOW_8 => 8,
      DIVSLOW_A::DIVSLOW_9 => 9,
      DIVSLOW_A::DIVSLOW_10 => 10,
      DIVSLOW_A::DIVSLOW_11 => 11,
      DIVSLOW_A::DIVSLOW_12 => 12,
      DIVSLOW_A::DIVSLOW_13 => 13,
      DIVSLOW_A::DIVSLOW_14 => 14,
      DIVSLOW_A::DIVSLOW_15 => 15,
    }
  }
}
#[doc = "Reader of field `DIVSLOW`"]
pub type DIVSLOW_R = crate::R<u8, DIVSLOW_A>;
impl DIVSLOW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DIVSLOW_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(DIVSLOW_A::DIVSLOW_1),
      2 => Val(DIVSLOW_A::DIVSLOW_2),
      3 => Val(DIVSLOW_A::DIVSLOW_3),
      4 => Val(DIVSLOW_A::DIVSLOW_4),
      5 => Val(DIVSLOW_A::DIVSLOW_5),
      6 => Val(DIVSLOW_A::DIVSLOW_6),
      7 => Val(DIVSLOW_A::DIVSLOW_7),
      8 => Val(DIVSLOW_A::DIVSLOW_8),
      9 => Val(DIVSLOW_A::DIVSLOW_9),
      10 => Val(DIVSLOW_A::DIVSLOW_10),
      11 => Val(DIVSLOW_A::DIVSLOW_11),
      12 => Val(DIVSLOW_A::DIVSLOW_12),
      13 => Val(DIVSLOW_A::DIVSLOW_13),
      14 => Val(DIVSLOW_A::DIVSLOW_14),
      15 => Val(DIVSLOW_A::DIVSLOW_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_1`"]
  #[inline(always)]
  pub fn is_divslow_1(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_1
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_2`"]
  #[inline(always)]
  pub fn is_divslow_2(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_2
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_3`"]
  #[inline(always)]
  pub fn is_divslow_3(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_3
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_4`"]
  #[inline(always)]
  pub fn is_divslow_4(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_4
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_5`"]
  #[inline(always)]
  pub fn is_divslow_5(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_5
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_6`"]
  #[inline(always)]
  pub fn is_divslow_6(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_6
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_7`"]
  #[inline(always)]
  pub fn is_divslow_7(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_7
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_8`"]
  #[inline(always)]
  pub fn is_divslow_8(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_8
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_9`"]
  #[inline(always)]
  pub fn is_divslow_9(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_9
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_10`"]
  #[inline(always)]
  pub fn is_divslow_10(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_10
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_11`"]
  #[inline(always)]
  pub fn is_divslow_11(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_11
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_12`"]
  #[inline(always)]
  pub fn is_divslow_12(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_12
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_13`"]
  #[inline(always)]
  pub fn is_divslow_13(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_13
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_14`"]
  #[inline(always)]
  pub fn is_divslow_14(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_14
  }
  #[doc = "Checks if the value of the field is `DIVSLOW_15`"]
  #[inline(always)]
  pub fn is_divslow_15(&self) -> bool {
    *self == DIVSLOW_A::DIVSLOW_15
  }
}
#[doc = "Bus Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBUS_A {
  #[doc = "0: Divide-by-1"]
  DIVBUS_0,
  #[doc = "1: Divide-by-2"]
  DIVBUS_1,
  #[doc = "2: Divide-by-3"]
  DIVBUS_2,
  #[doc = "3: Divide-by-4"]
  DIVBUS_3,
  #[doc = "4: Divide-by-5"]
  DIVBUS_4,
  #[doc = "5: Divide-by-6"]
  DIVBUS_5,
  #[doc = "6: Divide-by-7"]
  DIVBUS_6,
  #[doc = "7: Divide-by-8"]
  DIVBUS_7,
  #[doc = "8: Divide-by-9"]
  DIVBUS_8,
  #[doc = "9: Divide-by-10"]
  DIVBUS_9,
  #[doc = "10: Divide-by-11"]
  DIVBUS_10,
  #[doc = "11: Divide-by-12"]
  DIVBUS_11,
  #[doc = "12: Divide-by-13"]
  DIVBUS_12,
  #[doc = "13: Divide-by-14"]
  DIVBUS_13,
  #[doc = "14: Divide-by-15"]
  DIVBUS_14,
  #[doc = "15: Divide-by-16"]
  DIVBUS_15,
}
impl From<DIVBUS_A> for u8 {
  #[inline(always)]
  fn from(variant: DIVBUS_A) -> Self {
    match variant {
      DIVBUS_A::DIVBUS_0 => 0,
      DIVBUS_A::DIVBUS_1 => 1,
      DIVBUS_A::DIVBUS_2 => 2,
      DIVBUS_A::DIVBUS_3 => 3,
      DIVBUS_A::DIVBUS_4 => 4,
      DIVBUS_A::DIVBUS_5 => 5,
      DIVBUS_A::DIVBUS_6 => 6,
      DIVBUS_A::DIVBUS_7 => 7,
      DIVBUS_A::DIVBUS_8 => 8,
      DIVBUS_A::DIVBUS_9 => 9,
      DIVBUS_A::DIVBUS_10 => 10,
      DIVBUS_A::DIVBUS_11 => 11,
      DIVBUS_A::DIVBUS_12 => 12,
      DIVBUS_A::DIVBUS_13 => 13,
      DIVBUS_A::DIVBUS_14 => 14,
      DIVBUS_A::DIVBUS_15 => 15,
    }
  }
}
#[doc = "Reader of field `DIVBUS`"]
pub type DIVBUS_R = crate::R<u8, DIVBUS_A>;
impl DIVBUS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIVBUS_A {
    match self.bits {
      0 => DIVBUS_A::DIVBUS_0,
      1 => DIVBUS_A::DIVBUS_1,
      2 => DIVBUS_A::DIVBUS_2,
      3 => DIVBUS_A::DIVBUS_3,
      4 => DIVBUS_A::DIVBUS_4,
      5 => DIVBUS_A::DIVBUS_5,
      6 => DIVBUS_A::DIVBUS_6,
      7 => DIVBUS_A::DIVBUS_7,
      8 => DIVBUS_A::DIVBUS_8,
      9 => DIVBUS_A::DIVBUS_9,
      10 => DIVBUS_A::DIVBUS_10,
      11 => DIVBUS_A::DIVBUS_11,
      12 => DIVBUS_A::DIVBUS_12,
      13 => DIVBUS_A::DIVBUS_13,
      14 => DIVBUS_A::DIVBUS_14,
      15 => DIVBUS_A::DIVBUS_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `DIVBUS_0`"]
  #[inline(always)]
  pub fn is_divbus_0(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_0
  }
  #[doc = "Checks if the value of the field is `DIVBUS_1`"]
  #[inline(always)]
  pub fn is_divbus_1(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_1
  }
  #[doc = "Checks if the value of the field is `DIVBUS_2`"]
  #[inline(always)]
  pub fn is_divbus_2(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_2
  }
  #[doc = "Checks if the value of the field is `DIVBUS_3`"]
  #[inline(always)]
  pub fn is_divbus_3(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_3
  }
  #[doc = "Checks if the value of the field is `DIVBUS_4`"]
  #[inline(always)]
  pub fn is_divbus_4(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_4
  }
  #[doc = "Checks if the value of the field is `DIVBUS_5`"]
  #[inline(always)]
  pub fn is_divbus_5(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_5
  }
  #[doc = "Checks if the value of the field is `DIVBUS_6`"]
  #[inline(always)]
  pub fn is_divbus_6(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_6
  }
  #[doc = "Checks if the value of the field is `DIVBUS_7`"]
  #[inline(always)]
  pub fn is_divbus_7(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_7
  }
  #[doc = "Checks if the value of the field is `DIVBUS_8`"]
  #[inline(always)]
  pub fn is_divbus_8(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_8
  }
  #[doc = "Checks if the value of the field is `DIVBUS_9`"]
  #[inline(always)]
  pub fn is_divbus_9(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_9
  }
  #[doc = "Checks if the value of the field is `DIVBUS_10`"]
  #[inline(always)]
  pub fn is_divbus_10(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_10
  }
  #[doc = "Checks if the value of the field is `DIVBUS_11`"]
  #[inline(always)]
  pub fn is_divbus_11(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_11
  }
  #[doc = "Checks if the value of the field is `DIVBUS_12`"]
  #[inline(always)]
  pub fn is_divbus_12(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_12
  }
  #[doc = "Checks if the value of the field is `DIVBUS_13`"]
  #[inline(always)]
  pub fn is_divbus_13(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_13
  }
  #[doc = "Checks if the value of the field is `DIVBUS_14`"]
  #[inline(always)]
  pub fn is_divbus_14(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_14
  }
  #[doc = "Checks if the value of the field is `DIVBUS_15`"]
  #[inline(always)]
  pub fn is_divbus_15(&self) -> bool {
    *self == DIVBUS_A::DIVBUS_15
  }
}
#[doc = "External Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVEXT_A {
  #[doc = "0: Divide-by-1"]
  DIVEXT_0,
  #[doc = "1: Divide-by-2"]
  DIVEXT_1,
  #[doc = "2: Divide-by-3"]
  DIVEXT_2,
  #[doc = "3: Divide-by-4"]
  DIVEXT_3,
  #[doc = "4: Divide-by-5"]
  DIVEXT_4,
  #[doc = "5: Divide-by-6"]
  DIVEXT_5,
  #[doc = "6: Divide-by-7"]
  DIVEXT_6,
  #[doc = "7: Divide-by-8"]
  DIVEXT_7,
  #[doc = "8: Divide-by-9"]
  DIVEXT_8,
  #[doc = "9: Divide-by-10"]
  DIVEXT_9,
  #[doc = "10: Divide-by-11"]
  DIVEXT_10,
  #[doc = "11: Divide-by-12"]
  DIVEXT_11,
  #[doc = "12: Divide-by-13"]
  DIVEXT_12,
  #[doc = "13: Divide-by-14"]
  DIVEXT_13,
  #[doc = "14: Divide-by-15"]
  DIVEXT_14,
  #[doc = "15: Divide-by-16"]
  DIVEXT_15,
}
impl From<DIVEXT_A> for u8 {
  #[inline(always)]
  fn from(variant: DIVEXT_A) -> Self {
    match variant {
      DIVEXT_A::DIVEXT_0 => 0,
      DIVEXT_A::DIVEXT_1 => 1,
      DIVEXT_A::DIVEXT_2 => 2,
      DIVEXT_A::DIVEXT_3 => 3,
      DIVEXT_A::DIVEXT_4 => 4,
      DIVEXT_A::DIVEXT_5 => 5,
      DIVEXT_A::DIVEXT_6 => 6,
      DIVEXT_A::DIVEXT_7 => 7,
      DIVEXT_A::DIVEXT_8 => 8,
      DIVEXT_A::DIVEXT_9 => 9,
      DIVEXT_A::DIVEXT_10 => 10,
      DIVEXT_A::DIVEXT_11 => 11,
      DIVEXT_A::DIVEXT_12 => 12,
      DIVEXT_A::DIVEXT_13 => 13,
      DIVEXT_A::DIVEXT_14 => 14,
      DIVEXT_A::DIVEXT_15 => 15,
    }
  }
}
#[doc = "Reader of field `DIVEXT`"]
pub type DIVEXT_R = crate::R<u8, DIVEXT_A>;
impl DIVEXT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIVEXT_A {
    match self.bits {
      0 => DIVEXT_A::DIVEXT_0,
      1 => DIVEXT_A::DIVEXT_1,
      2 => DIVEXT_A::DIVEXT_2,
      3 => DIVEXT_A::DIVEXT_3,
      4 => DIVEXT_A::DIVEXT_4,
      5 => DIVEXT_A::DIVEXT_5,
      6 => DIVEXT_A::DIVEXT_6,
      7 => DIVEXT_A::DIVEXT_7,
      8 => DIVEXT_A::DIVEXT_8,
      9 => DIVEXT_A::DIVEXT_9,
      10 => DIVEXT_A::DIVEXT_10,
      11 => DIVEXT_A::DIVEXT_11,
      12 => DIVEXT_A::DIVEXT_12,
      13 => DIVEXT_A::DIVEXT_13,
      14 => DIVEXT_A::DIVEXT_14,
      15 => DIVEXT_A::DIVEXT_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `DIVEXT_0`"]
  #[inline(always)]
  pub fn is_divext_0(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_0
  }
  #[doc = "Checks if the value of the field is `DIVEXT_1`"]
  #[inline(always)]
  pub fn is_divext_1(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_1
  }
  #[doc = "Checks if the value of the field is `DIVEXT_2`"]
  #[inline(always)]
  pub fn is_divext_2(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_2
  }
  #[doc = "Checks if the value of the field is `DIVEXT_3`"]
  #[inline(always)]
  pub fn is_divext_3(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_3
  }
  #[doc = "Checks if the value of the field is `DIVEXT_4`"]
  #[inline(always)]
  pub fn is_divext_4(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_4
  }
  #[doc = "Checks if the value of the field is `DIVEXT_5`"]
  #[inline(always)]
  pub fn is_divext_5(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_5
  }
  #[doc = "Checks if the value of the field is `DIVEXT_6`"]
  #[inline(always)]
  pub fn is_divext_6(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_6
  }
  #[doc = "Checks if the value of the field is `DIVEXT_7`"]
  #[inline(always)]
  pub fn is_divext_7(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_7
  }
  #[doc = "Checks if the value of the field is `DIVEXT_8`"]
  #[inline(always)]
  pub fn is_divext_8(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_8
  }
  #[doc = "Checks if the value of the field is `DIVEXT_9`"]
  #[inline(always)]
  pub fn is_divext_9(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_9
  }
  #[doc = "Checks if the value of the field is `DIVEXT_10`"]
  #[inline(always)]
  pub fn is_divext_10(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_10
  }
  #[doc = "Checks if the value of the field is `DIVEXT_11`"]
  #[inline(always)]
  pub fn is_divext_11(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_11
  }
  #[doc = "Checks if the value of the field is `DIVEXT_12`"]
  #[inline(always)]
  pub fn is_divext_12(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_12
  }
  #[doc = "Checks if the value of the field is `DIVEXT_13`"]
  #[inline(always)]
  pub fn is_divext_13(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_13
  }
  #[doc = "Checks if the value of the field is `DIVEXT_14`"]
  #[inline(always)]
  pub fn is_divext_14(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_14
  }
  #[doc = "Checks if the value of the field is `DIVEXT_15`"]
  #[inline(always)]
  pub fn is_divext_15(&self) -> bool {
    *self == DIVEXT_A::DIVEXT_15
  }
}
#[doc = "Core Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVCORE_A {
  #[doc = "0: Divide-by-1"]
  DIVCORE_0,
  #[doc = "1: Divide-by-2"]
  DIVCORE_1,
  #[doc = "2: Divide-by-3"]
  DIVCORE_2,
  #[doc = "3: Divide-by-4"]
  DIVCORE_3,
  #[doc = "4: Divide-by-5"]
  DIVCORE_4,
  #[doc = "5: Divide-by-6"]
  DIVCORE_5,
  #[doc = "6: Divide-by-7"]
  DIVCORE_6,
  #[doc = "7: Divide-by-8"]
  DIVCORE_7,
  #[doc = "8: Divide-by-9"]
  DIVCORE_8,
  #[doc = "9: Divide-by-10"]
  DIVCORE_9,
  #[doc = "10: Divide-by-11"]
  DIVCORE_10,
  #[doc = "11: Divide-by-12"]
  DIVCORE_11,
  #[doc = "12: Divide-by-13"]
  DIVCORE_12,
  #[doc = "13: Divide-by-14"]
  DIVCORE_13,
  #[doc = "14: Divide-by-15"]
  DIVCORE_14,
  #[doc = "15: Divide-by-16"]
  DIVCORE_15,
}
impl From<DIVCORE_A> for u8 {
  #[inline(always)]
  fn from(variant: DIVCORE_A) -> Self {
    match variant {
      DIVCORE_A::DIVCORE_0 => 0,
      DIVCORE_A::DIVCORE_1 => 1,
      DIVCORE_A::DIVCORE_2 => 2,
      DIVCORE_A::DIVCORE_3 => 3,
      DIVCORE_A::DIVCORE_4 => 4,
      DIVCORE_A::DIVCORE_5 => 5,
      DIVCORE_A::DIVCORE_6 => 6,
      DIVCORE_A::DIVCORE_7 => 7,
      DIVCORE_A::DIVCORE_8 => 8,
      DIVCORE_A::DIVCORE_9 => 9,
      DIVCORE_A::DIVCORE_10 => 10,
      DIVCORE_A::DIVCORE_11 => 11,
      DIVCORE_A::DIVCORE_12 => 12,
      DIVCORE_A::DIVCORE_13 => 13,
      DIVCORE_A::DIVCORE_14 => 14,
      DIVCORE_A::DIVCORE_15 => 15,
    }
  }
}
#[doc = "Reader of field `DIVCORE`"]
pub type DIVCORE_R = crate::R<u8, DIVCORE_A>;
impl DIVCORE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIVCORE_A {
    match self.bits {
      0 => DIVCORE_A::DIVCORE_0,
      1 => DIVCORE_A::DIVCORE_1,
      2 => DIVCORE_A::DIVCORE_2,
      3 => DIVCORE_A::DIVCORE_3,
      4 => DIVCORE_A::DIVCORE_4,
      5 => DIVCORE_A::DIVCORE_5,
      6 => DIVCORE_A::DIVCORE_6,
      7 => DIVCORE_A::DIVCORE_7,
      8 => DIVCORE_A::DIVCORE_8,
      9 => DIVCORE_A::DIVCORE_9,
      10 => DIVCORE_A::DIVCORE_10,
      11 => DIVCORE_A::DIVCORE_11,
      12 => DIVCORE_A::DIVCORE_12,
      13 => DIVCORE_A::DIVCORE_13,
      14 => DIVCORE_A::DIVCORE_14,
      15 => DIVCORE_A::DIVCORE_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `DIVCORE_0`"]
  #[inline(always)]
  pub fn is_divcore_0(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_0
  }
  #[doc = "Checks if the value of the field is `DIVCORE_1`"]
  #[inline(always)]
  pub fn is_divcore_1(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_1
  }
  #[doc = "Checks if the value of the field is `DIVCORE_2`"]
  #[inline(always)]
  pub fn is_divcore_2(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_2
  }
  #[doc = "Checks if the value of the field is `DIVCORE_3`"]
  #[inline(always)]
  pub fn is_divcore_3(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_3
  }
  #[doc = "Checks if the value of the field is `DIVCORE_4`"]
  #[inline(always)]
  pub fn is_divcore_4(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_4
  }
  #[doc = "Checks if the value of the field is `DIVCORE_5`"]
  #[inline(always)]
  pub fn is_divcore_5(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_5
  }
  #[doc = "Checks if the value of the field is `DIVCORE_6`"]
  #[inline(always)]
  pub fn is_divcore_6(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_6
  }
  #[doc = "Checks if the value of the field is `DIVCORE_7`"]
  #[inline(always)]
  pub fn is_divcore_7(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_7
  }
  #[doc = "Checks if the value of the field is `DIVCORE_8`"]
  #[inline(always)]
  pub fn is_divcore_8(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_8
  }
  #[doc = "Checks if the value of the field is `DIVCORE_9`"]
  #[inline(always)]
  pub fn is_divcore_9(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_9
  }
  #[doc = "Checks if the value of the field is `DIVCORE_10`"]
  #[inline(always)]
  pub fn is_divcore_10(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_10
  }
  #[doc = "Checks if the value of the field is `DIVCORE_11`"]
  #[inline(always)]
  pub fn is_divcore_11(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_11
  }
  #[doc = "Checks if the value of the field is `DIVCORE_12`"]
  #[inline(always)]
  pub fn is_divcore_12(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_12
  }
  #[doc = "Checks if the value of the field is `DIVCORE_13`"]
  #[inline(always)]
  pub fn is_divcore_13(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_13
  }
  #[doc = "Checks if the value of the field is `DIVCORE_14`"]
  #[inline(always)]
  pub fn is_divcore_14(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_14
  }
  #[doc = "Checks if the value of the field is `DIVCORE_15`"]
  #[inline(always)]
  pub fn is_divcore_15(&self) -> bool {
    *self == DIVCORE_A::DIVCORE_15
  }
}
#[doc = "System Clock Source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCS_A {
  #[doc = "1: no description available"]
  SCS_1,
  #[doc = "2: no description available"]
  SCS_2,
  #[doc = "3: no description available"]
  SCS_3,
  #[doc = "4: no description available"]
  SCS_4,
  #[doc = "5: no description available"]
  SCS_5,
}
impl From<SCS_A> for u8 {
  #[inline(always)]
  fn from(variant: SCS_A) -> Self {
    match variant {
      SCS_A::SCS_1 => 1,
      SCS_A::SCS_2 => 2,
      SCS_A::SCS_3 => 3,
      SCS_A::SCS_4 => 4,
      SCS_A::SCS_5 => 5,
    }
  }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, SCS_A>;
impl SCS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, SCS_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(SCS_A::SCS_1),
      2 => Val(SCS_A::SCS_2),
      3 => Val(SCS_A::SCS_3),
      4 => Val(SCS_A::SCS_4),
      5 => Val(SCS_A::SCS_5),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `SCS_1`"]
  #[inline(always)]
  pub fn is_scs_1(&self) -> bool {
    *self == SCS_A::SCS_1
  }
  #[doc = "Checks if the value of the field is `SCS_2`"]
  #[inline(always)]
  pub fn is_scs_2(&self) -> bool {
    *self == SCS_A::SCS_2
  }
  #[doc = "Checks if the value of the field is `SCS_3`"]
  #[inline(always)]
  pub fn is_scs_3(&self) -> bool {
    *self == SCS_A::SCS_3
  }
  #[doc = "Checks if the value of the field is `SCS_4`"]
  #[inline(always)]
  pub fn is_scs_4(&self) -> bool {
    *self == SCS_A::SCS_4
  }
  #[doc = "Checks if the value of the field is `SCS_5`"]
  #[inline(always)]
  pub fn is_scs_5(&self) -> bool {
    *self == SCS_A::SCS_5
  }
}
impl R {
  #[doc = "Bits 0:3 - Slow Clock Divide Ratio"]
  #[inline(always)]
  pub fn divslow(&self) -> DIVSLOW_R {
    DIVSLOW_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 4:7 - Bus Clock Divide Ratio"]
  #[inline(always)]
  pub fn divbus(&self) -> DIVBUS_R {
    DIVBUS_R::new(((self.bits >> 4) & 0x0f) as u8)
  }
  #[doc = "Bits 8:11 - External Clock Divide Ratio"]
  #[inline(always)]
  pub fn divext(&self) -> DIVEXT_R {
    DIVEXT_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
  #[doc = "Bits 16:19 - Core Clock Divide Ratio"]
  #[inline(always)]
  pub fn divcore(&self) -> DIVCORE_R {
    DIVCORE_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - System Clock Source"]
  #[inline(always)]
  pub fn scs(&self) -> SCS_R {
    SCS_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
