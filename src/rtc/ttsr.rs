#[doc = "Reader of register TTSR"]
pub type R = crate::R<u32, super::TTSR>;
#[doc = "Reader of field `TTS`"]
pub type TTS_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - Tamper Time Seconds"]
  #[inline(always)]
  pub fn tts(&self) -> TTS_R {
    TTS_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
