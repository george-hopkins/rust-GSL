//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use c_vec::CSlice;
use ffi;

pub struct FftComplexWaveTable {
    w: *mut sys::gsl_fft_complex_wavetable,
    f: CSlice<usize>,
}

impl FftComplexWaveTable {
    /// This function prepares a trigonometric lookup table for a complex FFT of length n. The function returns a pointer to the newly allocated
    /// gsl_fft_complex_wavetable if no errors were detected, and a null pointer in the case of error. The length n is factorized into a product
    /// of subtransforms, and the factors and their trigonometric coefficients are stored in the wavetable. The trigonometric coefficients are
    /// computed using direct calls to sin and cos, for accuracy. Recursion relations could be used to compute the lookup table faster, but if
    /// an application performs many FFTs of the same length then this computation is a one-off overhead which does not affect the final throughput.
    ///
    /// The wavetable structure can be used repeatedly for any transform of the same length. The table is not modified by calls to any of the other
    /// FFT functions. The same wavetable can be used for both forward and backward (or inverse) transforms of a given length.
    pub fn new(n: usize) -> Option<FftComplexWaveTable> {
        let tmp = unsafe { sys::gsl_fft_complex_wavetable_alloc(n) };

        if tmp.is_null() {
            None
        } else {
            unsafe {
                Some(FftComplexWaveTable {
                    w: tmp,
                    f: CSlice::new((*tmp).factor.as_mut_ptr(), 64),
                })
            }
        }
    }

    pub fn factor<'r>(&'r mut self) -> &'r mut [usize] {
        self.f.as_mut()
    }
}

impl Drop for FftComplexWaveTable {
    fn drop(&mut self) {
        unsafe { sys::gsl_fft_complex_wavetable_free(self.w) };
        self.w = ::std::ptr::null_mut();
    }
}

impl ffi::FFI<sys::gsl_fft_complex_wavetable> for FftComplexWaveTable {
    fn wrap(w: *mut sys::gsl_fft_complex_wavetable) -> Self {
        Self {
            w: w,
            f: unsafe { CSlice::new((*w).factor.as_mut_ptr(), 64) },
        }
    }

    fn soft_wrap(w: *mut sys::gsl_fft_complex_wavetable) -> Self {
        Self::wrap(w)
    }

    fn unwrap_shared(&self) -> *const sys::gsl_fft_complex_wavetable {
        self.w as *const _
    }

    fn unwrap_unique(&mut self) -> *mut sys::gsl_fft_complex_wavetable {
        self.w
    }
}

pub struct FftComplexWorkspace {
    w: *mut sys::gsl_fft_complex_workspace,
}

impl FftComplexWorkspace {
    /// This function allocates a workspace for a complex transform of length n.
    pub fn new(n: usize) -> Option<FftComplexWorkspace> {
        let tmp = unsafe { sys::gsl_fft_complex_workspace_alloc(n) };

        if tmp.is_null() {
            None
        } else {
            Some(FftComplexWorkspace { w: tmp })
        }
    }
}

impl Drop for FftComplexWorkspace {
    fn drop(&mut self) {
        unsafe { sys::gsl_fft_complex_workspace_free(self.w) };
        self.w = ::std::ptr::null_mut();
    }
}

impl ffi::FFI<sys::gsl_fft_complex_workspace> for FftComplexWorkspace {
    fn wrap(w: *mut sys::gsl_fft_complex_workspace) -> Self {
        Self { w: w }
    }

    fn soft_wrap(w: *mut sys::gsl_fft_complex_workspace) -> Self {
        Self::wrap(w)
    }

    fn unwrap_shared(&self) -> *const sys::gsl_fft_complex_workspace {
        self.w as *const _
    }

    fn unwrap_unique(&mut self) -> *mut sys::gsl_fft_complex_workspace {
        self.w
    }
}
