#[macro_export]
macro_rules! opaque {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            _private: [u8; 0]
        }
    };
}

#[cfg(test)]
mod test {
    opaque!(leveldb_t);
}