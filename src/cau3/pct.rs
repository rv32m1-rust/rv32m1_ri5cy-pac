#[doc = "Reader of register PCT"]
pub type R = crate::R<u32, super::PCT>;
#[doc = "Minor version number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Y_A {
  #[doc = "0: Minor version number"]
  Y_0,
}
impl From<Y_A> for u8 {
  #[inline(always)]
  fn from(variant: Y_A) -> Self {
    match variant {
      Y_A::Y_0 => 0,
    }
  }
}
#[doc = "Reader of field `Y`"]
pub type Y_R = crate::R<u8, Y_A>;
impl Y_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, Y_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(Y_A::Y_0),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `Y_0`"]
  #[inline(always)]
  pub fn is_y_0(&self) -> bool {
    *self == Y_A::Y_0
  }
}
#[doc = "Major version number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X_A {
  #[doc = "0: Major version number"]
  X_0,
}
impl From<X_A> for u8 {
  #[inline(always)]
  fn from(variant: X_A) -> Self {
    match variant {
      X_A::X_0 => 0,
    }
  }
}
#[doc = "Reader of field `X`"]
pub type X_R = crate::R<u8, X_A>;
impl X_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, X_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(X_A::X_0),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `X_0`"]
  #[inline(always)]
  pub fn is_x_0(&self) -> bool {
    *self == X_A::X_0
  }
}
#[doc = "Module ID number\n\nValue on reset: 4931936"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID_A {
  #[doc = "4931936: ID number for basic configuration"]
  ID_4931936,
  #[doc = "4931937: ID number for PKHA configuration"]
  ID_4931937,
}
impl From<ID_A> for u32 {
  #[inline(always)]
  fn from(variant: ID_A) -> Self {
    match variant {
      ID_A::ID_4931936 => 4931936,
      ID_A::ID_4931937 => 4931937,
    }
  }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, ID_A>;
impl ID_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u32, ID_A> {
    use crate::Variant::*;
    match self.bits {
      4931936 => Val(ID_A::ID_4931936),
      4931937 => Val(ID_A::ID_4931937),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `ID_4931936`"]
  #[inline(always)]
  pub fn is_id_4931936(&self) -> bool {
    *self == ID_A::ID_4931936
  }
  #[doc = "Checks if the value of the field is `ID_4931937`"]
  #[inline(always)]
  pub fn is_id_4931937(&self) -> bool {
    *self == ID_A::ID_4931937
  }
}
impl R {
  #[doc = "Bits 0:3 - Minor version number"]
  #[inline(always)]
  pub fn y(&self) -> Y_R {
    Y_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 4:7 - Major version number"]
  #[inline(always)]
  pub fn x(&self) -> X_R {
    X_R::new(((self.bits >> 4) & 0x0f) as u8)
  }
  #[doc = "Bits 8:31 - Module ID number"]
  #[inline(always)]
  pub fn id(&self) -> ID_R {
    ID_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
  }
}
