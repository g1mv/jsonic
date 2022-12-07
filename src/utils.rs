mod fnv {
    use std::hash::Hasher;

    pub struct FnvHasher(u64);

    impl Default for FnvHasher {
        #[inline(always)]
        fn default() -> FnvHasher {
            FnvHasher(0xcbf29ce484222325)
        }
    }

    impl Hasher for FnvHasher {
        #[inline(always)]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline(always)]
        fn write(&mut self, bytes: &[u8]) {
            let FnvHasher(mut hash) = *self;

            for byte in bytes.iter() {
                hash = hash ^ (*byte as u64);
                hash = hash.wrapping_mul(0x100000001b3);
            }

            *self = FnvHasher(hash);
        }
    }
}

mod simple {
    use std::hash::Hasher;

    pub struct SimpleHasher(u64);

    #[inline(always)]
    fn load_u64_le(buf: &[u8], len: usize) -> u64 {
        use std::ptr;
        debug_assert!(len <= buf.len());
        let mut data = 0u64;
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), &mut data as *mut _ as *mut u8, len);
        }
        data.to_le()
    }


    impl Default for SimpleHasher {
        #[inline(always)]
        fn default() -> SimpleHasher {
            SimpleHasher(0)
        }
    }

    impl Hasher for SimpleHasher {
        #[inline(always)]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline(always)]
        fn write(&mut self, bytes: &[u8]) {
            *self = SimpleHasher(load_u64_le(bytes, bytes.len()));
        }
    }
}