#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASE` writer - Clear ERASE Interrupt Flag"]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `WRITE` writer - Clear WRITE Interrupt Flag"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CHOF` writer - Clear CHOF Interrupt Flag"]
pub type CHOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CMOF` writer - Clear CMOF Interrupt Flag"]
pub type CMOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `PWRUPF` writer - Clear PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `ICACHERR` writer - Clear ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `WDATAOV` writer - Clear WDATAOV Interrupt Flag"]
pub type WDATAOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LVEWRITE` writer - Clear LVEWRITE Interrupt Flag"]
pub type LVEWRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<0> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - Clear WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Clear CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> CHOF_W<2> {
        CHOF_W::new(self)
    }
    #[doc = "Bit 3 - Clear CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CMOF_W<3> {
        CMOF_W::new(self)
    }
    #[doc = "Bit 4 - Clear PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<4> {
        PWRUPF_W::new(self)
    }
    #[doc = "Bit 5 - Clear ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> ICACHERR_W<5> {
        ICACHERR_W::new(self)
    }
    #[doc = "Bit 6 - Clear WDATAOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdataov(&mut self) -> WDATAOV_W<6> {
        WDATAOV_W::new(self)
    }
    #[doc = "Bit 8 - Clear LVEWRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lvewrite(&mut self) -> LVEWRITE_W<8> {
        LVEWRITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
