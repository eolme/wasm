#![feature(never_type)]

use brotli::{
    enc::{
        backward_references::{BrotliEncoderMode, BrotliHasherParams},
        command::BrotliDistanceParams,
        encode::{BROTLI_DISTANCE_ALPHABET_SIZE, BROTLI_MAX_DISTANCE, BROTLI_MAX_DISTANCE_BITS},
        BrotliCompressCustomIoCustomDict, BrotliEncoderParams,
    },
    interface, CustomRead, CustomWrite, InputReferenceMut,
};
use wasm_bindgen::prelude::wasm_bindgen;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct StaticRead {}
impl CustomRead<Option<!>> for StaticRead {
    fn read(self: &mut Self, data: &mut [u8]) -> Result<usize, Option<!>> {
        Ok(data.len())
    }
}

struct StaticWrite {}
impl CustomWrite<Option<!>> for StaticWrite {
    fn write(self: &mut Self, data: &[u8]) -> Result<usize, Option<!>> {
        Ok(data.len())
    }
    fn flush(self: &mut Self) -> Result<(), Option<!>> {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn compress(input: &mut [u8]) -> usize {
    let len = input.len();
    let mut output: Vec<u8> = Vec::with_capacity(len);
    let mut params = BrotliEncoderParams {
        dist: BrotliDistanceParams {
            distance_postfix_bits: 0,
            num_direct_distance_codes: 0,
            alphabet_size: BROTLI_DISTANCE_ALPHABET_SIZE(0, 0, BROTLI_MAX_DISTANCE_BITS),
            max_distance: BROTLI_MAX_DISTANCE,
        },
        mode: BrotliEncoderMode::BROTLI_MODE_TEXT,
        log_meta_block: false,
        large_window: false,
        avoid_distance_prefix_search: false,
        quality: 6,
        q9_5: false,
        lgwin: 12i32,
        lgblock: 0i32,
        size_hint: len,
        disable_literal_context_modeling: 0i32,
        stride_detection_quality: 2,
        high_entropy_detection_quality: 0,
        cdf_adaptation_detection: 0,
        prior_bitmask_detection: 0,
        literal_adaptation: [(0, 0); 4],
        catable: true,
        use_dictionary: false,
        appendable: true,
        magic_number: true,
        favor_cpu_efficiency: false,
        hasher: BrotliHasherParams {
            type_: 6,
            block_bits: 9 - 1,
            bucket_bits: 15,
            hash_len: 5,
            num_last_distances_to_check: 16,
            literal_byte_score: 0,
        },
    };

    let mut nop_callback = |_data: &mut interface::PredictionModeContextMap<InputReferenceMut>,
                            _cmds: &mut [interface::StaticCommand],
                            _mb: interface::InputPair,
                            _m: &mut WeeAlloc| ();

    let result = BrotliCompressCustomIoCustomDict(
        &mut StaticRead {},
        &mut StaticWrite {},
        input,
        output.as_mut_slice(),
        &mut params,
        ALLOC,
        &mut nop_callback,
        &[],
        None,
    );

    return result.unwrap();
}
