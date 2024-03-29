#[doc = "Register `PPUPATD0` reader"]
pub struct R(crate::R<PPUPATD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUPATD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUPATD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUPATD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUPATD0` writer"]
pub struct W(crate::W<PPUPATD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUPATD0_SPEC>;
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
impl From<crate::W<PPUPATD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUPATD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP0` reader - Analog Comparator 0 access control bit"]
pub type ACMP0_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 access control bit"]
pub type ACMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 access control bit"]
pub type ACMP1_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 access control bit"]
pub type ACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 access control bit"]
pub type ADC0_R = crate::BitReader<bool>;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 access control bit"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CMU` reader - Clock Management Unit access control bit"]
pub type CMU_R = crate::BitReader<bool>;
#[doc = "Field `CMU` writer - Clock Management Unit access control bit"]
pub type CMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER access control bit"]
pub type CRYOTIMER_R = crate::BitReader<bool>;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER access control bit"]
pub type CRYOTIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CRYPTO0` reader - Advanced Encryption Standard Accelerator 0 access control bit"]
pub type CRYPTO0_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO0` writer - Advanced Encryption Standard Accelerator 0 access control bit"]
pub type CRYPTO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CRYPTO1` reader - Advanced Encryption Standard Accelerator 1 access control bit"]
pub type CRYPTO1_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO1` writer - Advanced Encryption Standard Accelerator 1 access control bit"]
pub type CRYPTO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module access control bit"]
pub type CSEN_R = crate::BitReader<bool>;
#[doc = "Field `CSEN` writer - Capacitive touch sense module access control bit"]
pub type CSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 access control bit"]
pub type VDAC0_R = crate::BitReader<bool>;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 access control bit"]
pub type VDAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `PRS` reader - Peripheral Reflex System access control bit"]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - Peripheral Reflex System access control bit"]
pub type PRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `EMU` reader - Energy Management Unit access control bit"]
pub type EMU_R = crate::BitReader<bool>;
#[doc = "Field `EMU` writer - Energy Management Unit access control bit"]
pub type EMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `FPUEH` reader - FPU Exception Handler access control bit"]
pub type FPUEH_R = crate::BitReader<bool>;
#[doc = "Field `FPUEH` writer - FPU Exception Handler access control bit"]
pub type FPUEH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `GPCRC` reader - General Purpose CRC access control bit"]
pub type GPCRC_R = crate::BitReader<bool>;
#[doc = "Field `GPCRC` writer - General Purpose CRC access control bit"]
pub type GPCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - General purpose Input/Output access control bit"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - General purpose Input/Output access control bit"]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `I2C0` reader - I2C 0 access control bit"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C 0 access control bit"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - I2C 1 access control bit"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - I2C 1 access control bit"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 access control bit"]
pub type IDAC0_R = crate::BitReader<bool>;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 access control bit"]
pub type IDAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `MSC` reader - Memory System Controller access control bit"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSC` writer - Memory System Controller access control bit"]
pub type MSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller access control bit"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller access control bit"]
pub type LDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface access control bit"]
pub type LESENSE_R = crate::BitReader<bool>;
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface access control bit"]
pub type LESENSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 access control bit"]
pub type LETIMER0_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 access control bit"]
pub type LETIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 access control bit"]
pub type LEUART0_R = crate::BitReader<bool>;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 access control bit"]
pub type LEUART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `PCNT0` reader - Pulse Counter 0 access control bit"]
pub type PCNT0_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0` writer - Pulse Counter 0 access control bit"]
pub type PCNT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `PCNT1` reader - Pulse Counter 1 access control bit"]
pub type PCNT1_R = crate::BitReader<bool>;
#[doc = "Field `PCNT1` writer - Pulse Counter 1 access control bit"]
pub type PCNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `PCNT2` reader - Pulse Counter 2 access control bit"]
pub type PCNT2_R = crate::BitReader<bool>;
#[doc = "Field `PCNT2` writer - Pulse Counter 2 access control bit"]
pub type PCNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&self) -> CMU_R {
        CMU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CRYOTIMER access control bit"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CRYOTIMER_R {
        CRYOTIMER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Advanced Encryption Standard Accelerator 0 access control bit"]
    #[inline(always)]
    pub fn crypto0(&self) -> CRYPTO0_R {
        CRYPTO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Advanced Encryption Standard Accelerator 1 access control bit"]
    #[inline(always)]
    pub fn crypto1(&self) -> CRYPTO1_R {
        CRYPTO1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&self) -> EMU_R {
        EMU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&self) -> FPUEH_R {
        FPUEH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&self) -> PCNT0_R {
        PCNT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&self) -> PCNT1_R {
        PCNT1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&self) -> PCNT2_R {
        PCNT2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<0> {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> ACMP1_W<1> {
        ACMP1_W::new(self)
    }
    #[doc = "Bit 2 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<2> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 5 - Clock Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmu(&mut self) -> CMU_W<5> {
        CMU_W::new(self)
    }
    #[doc = "Bit 7 - CRYOTIMER access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cryotimer(&mut self) -> CRYOTIMER_W<7> {
        CRYOTIMER_W::new(self)
    }
    #[doc = "Bit 8 - Advanced Encryption Standard Accelerator 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crypto0(&mut self) -> CRYPTO0_W<8> {
        CRYPTO0_W::new(self)
    }
    #[doc = "Bit 9 - Advanced Encryption Standard Accelerator 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crypto1(&mut self) -> CRYPTO1_W<9> {
        CRYPTO1_W::new(self)
    }
    #[doc = "Bit 10 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<10> {
        CSEN_W::new(self)
    }
    #[doc = "Bit 11 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn vdac0(&mut self) -> VDAC0_W<11> {
        VDAC0_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<12> {
        PRS_W::new(self)
    }
    #[doc = "Bit 13 - Energy Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn emu(&mut self) -> EMU_W<13> {
        EMU_W::new(self)
    }
    #[doc = "Bit 14 - FPU Exception Handler access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn fpueh(&mut self) -> FPUEH_W<14> {
        FPUEH_W::new(self)
    }
    #[doc = "Bit 16 - General Purpose CRC access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GPCRC_W<16> {
        GPCRC_W::new(self)
    }
    #[doc = "Bit 17 - General purpose Input/Output access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<17> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 18 - I2C 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<18> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 19 - I2C 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<19> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 20 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn idac0(&mut self) -> IDAC0_W<20> {
        IDAC0_W::new(self)
    }
    #[doc = "Bit 21 - Memory System Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<21> {
        MSC_W::new(self)
    }
    #[doc = "Bit 22 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<22> {
        LDMA_W::new(self)
    }
    #[doc = "Bit 23 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn lesense(&mut self) -> LESENSE_W<23> {
        LESENSE_W::new(self)
    }
    #[doc = "Bit 24 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> LETIMER0_W<24> {
        LETIMER0_W::new(self)
    }
    #[doc = "Bit 25 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<25> {
        LEUART0_W::new(self)
    }
    #[doc = "Bit 27 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0(&mut self) -> PCNT0_W<27> {
        PCNT0_W::new(self)
    }
    #[doc = "Bit 28 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1(&mut self) -> PCNT1_W<28> {
        PCNT1_W::new(self)
    }
    #[doc = "Bit 29 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2(&mut self) -> PCNT2_W<29> {
        PCNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU Privilege Access Type Descriptor 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd0](index.html) module"]
pub struct PPUPATD0_SPEC;
impl crate::RegisterSpec for PPUPATD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppupatd0::R](R) reader structure"]
impl crate::Readable for PPUPATD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppupatd0::W](W) writer structure"]
impl crate::Writable for PPUPATD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUPATD0 to value 0"]
impl crate::Resettable for PPUPATD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
