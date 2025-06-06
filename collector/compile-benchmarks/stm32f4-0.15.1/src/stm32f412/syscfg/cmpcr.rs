#[doc = "Register `CMPCR` reader"]
pub struct R(crate::R<CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY` reader - READY"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `CMP_PD` reader - Compensation cell power-down"]
pub type CMP_PD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpcr](index.html) module"]
pub struct CMPCR_SPEC;
impl crate::RegisterSpec for CMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpcr::R](R) reader structure"]
impl crate::Readable for CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
