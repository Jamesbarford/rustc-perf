#[doc = "Register `M1AR` reader"]
pub struct R(crate::R<M1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M1AR` writer"]
pub struct W(crate::W<M1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M1AR_SPEC>;
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
impl From<crate::W<M1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M1AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M1A` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1A_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M1A` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M1AR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&mut self) -> M1A_W<0> {
        M1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1ar](index.html) module"]
pub struct M1AR_SPEC;
impl crate::RegisterSpec for M1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m1ar::R](R) reader structure"]
impl crate::Readable for M1AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m1ar::W](W) writer structure"]
impl crate::Writable for M1AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M1AR to value 0"]
impl crate::Resettable for M1AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
