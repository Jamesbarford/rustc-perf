#[doc = "Register `PRER` reader"]
pub struct R(crate::R<PRER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRER` writer"]
pub struct W(crate::W<PRER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREDIV_A` reader - Asynchronous prescaler factor"]
pub type PREDIV_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREDIV_A` writer - Asynchronous prescaler factor"]
pub type PREDIV_A_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRER_SPEC, u8, u8, 7, O>;
#[doc = "Field `PREDIV_S` reader - Synchronous prescaler factor"]
pub type PREDIV_S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PREDIV_S` writer - Synchronous prescaler factor"]
pub type PREDIV_S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRER_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&self) -> PREDIV_A_R {
        PREDIV_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&self) -> PREDIV_S_R {
        PREDIV_S_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&mut self) -> PREDIV_A_W<16> {
        PREDIV_A_W::new(self)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&mut self) -> PREDIV_S_W<0> {
        PREDIV_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer](index.html) module"]
pub struct PRER_SPEC;
impl crate::RegisterSpec for PRER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prer::R](R) reader structure"]
impl crate::Readable for PRER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prer::W](W) writer structure"]
impl crate::Writable for PRER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRER to value 0x007f_00ff"]
impl crate::Resettable for PRER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x007f_00ff
    }
}
