#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("include/cpp.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;

        fn add(a: i32, b: i32) -> i32;

    }
}

pub fn add(a: i32, b: i32) -> i32 {
    ffi::add(a, b)
}
