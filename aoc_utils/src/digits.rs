pub trait NumDigits {
    fn num_digits(&self) -> usize; //𝑓(𝑛,base)=logbase(𝑛)+1
}

impl NumDigits for usize {
    fn num_digits(&self) -> usize {
        (*self as f64).log10().floor() as usize + 1
    }
}

impl NumDigits for isize {
    fn num_digits(&self) -> usize {
        (*self as f64).log10().floor() as usize + 1
    }
}

impl NumDigits for u32 {
    fn num_digits(&self) -> usize {
        (*self as f64).log10().floor() as usize + 1
    }
}

impl NumDigits for i32 {
    fn num_digits(&self) -> usize {
        (*self as f64).log10().floor() as usize + 1
    }
}

impl NumDigits for u64 {
    fn num_digits(&self) -> usize {
        (*self as f64).log10().floor() as usize + 1
    }
}

impl NumDigits for i64 {
    fn num_digits(&self) -> usize {
        (*self as f64).log10().floor() as usize + 1
    }
}

pub trait ToDigits<T> {
    fn to_digits(&self) -> Vec<T>;
}

impl ToDigits<usize> for usize {
    fn to_digits(&self) -> Vec<Self> {
        let mut digits = Vec::new();
        let mut n = *self;
        while n > 9 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }
}

impl ToDigits<isize> for isize {
    fn to_digits(&self) -> Vec<Self> {
        let mut digits = Vec::new();
        let mut n = *self;
        while n > 9 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }
}

impl ToDigits<u32> for u32 {
    fn to_digits(&self) -> Vec<Self> {
        let mut digits = Vec::new();
        let mut n = *self;
        while n > 9 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }
}
impl ToDigits<i32> for i32 {
    fn to_digits(&self) -> Vec<Self> {
        let mut digits = Vec::new();
        let mut n = *self;
        while n > 9 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }
}
