/* automatically generated by rust-bindgen 0.55.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_BlockHeader_t {
    pub version: u32,
    pub hashPrevBlock: [u32; 8usize],
    pub hashMerkleRoot: [u32; 8usize],
    pub timeSeconds: u32,
    pub workBits: u32,
    pub nonce: u32,
}
#[test]
fn bindgen_test_layout_PacketCrypt_BlockHeader_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_BlockHeader_t>(),
        80usize,
        concat!("Size of: ", stringify!(PacketCrypt_BlockHeader_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_BlockHeader_t>(),
        4usize,
        concat!("Alignment of ", stringify!(PacketCrypt_BlockHeader_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).version as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).hashPrevBlock as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(hashPrevBlock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).hashMerkleRoot as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(hashMerkleRoot)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).timeSeconds as *const _ as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(timeSeconds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).workBits as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(workBits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).nonce as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(nonce)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_AnnounceHdr_t {
    pub version: u8,
    pub softNonce: [u8; 3usize],
    pub hardNonce: u32,
    pub workBits: u32,
    pub parentBlockHeight: u32,
    pub contentType: u32,
    pub contentLength: u32,
    pub contentHash: [u8; 32usize],
    pub signingKey: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_PacketCrypt_AnnounceHdr_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_AnnounceHdr_t>(),
        88usize,
        concat!("Size of: ", stringify!(PacketCrypt_AnnounceHdr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_AnnounceHdr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(PacketCrypt_AnnounceHdr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).version as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).softNonce as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(softNonce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).hardNonce as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(hardNonce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).workBits as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(workBits)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).parentBlockHeight as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(parentBlockHeight)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).contentType as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(contentType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).contentLength as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(contentLength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).contentHash as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(contentHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).signingKey as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(signingKey)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PacketCrypt_Announce_t {
    pub hdr: PacketCrypt_AnnounceHdr_t,
    pub proof: [u64; 117usize],
}
#[test]
fn bindgen_test_layout_PacketCrypt_Announce_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_Announce_t>(),
        1024usize,
        concat!("Size of: ", stringify!(PacketCrypt_Announce_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_Announce_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_Announce_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Announce_t>())).hdr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Announce_t),
            "::",
            stringify!(hdr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Announce_t>())).proof as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Announce_t),
            "::",
            stringify!(proof)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PacketCrypt_HeaderAndProof_t {
    pub blockHeader: PacketCrypt_BlockHeader_t,
    pub _pad: u32,
    pub nonce2: u32,
    pub announcements: [PacketCrypt_Announce_t; 4usize],
    pub proof: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_PacketCrypt_HeaderAndProof_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_HeaderAndProof_t>(),
        4192usize,
        concat!("Size of: ", stringify!(PacketCrypt_HeaderAndProof_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_HeaderAndProof_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_HeaderAndProof_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).blockHeader as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(blockHeader)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>()))._pad as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(_pad)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).nonce2 as *const _ as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(nonce2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).announcements as *const _
                as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(announcements)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).proof as *const _ as usize
        },
        4184usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(proof)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_Coinbase_t {
    pub magic: u32,
    pub annLeastWorkTarget: u32,
    pub merkleRoot: [u8; 32usize],
    pub numAnns: u64,
}
#[test]
fn bindgen_test_layout_PacketCrypt_Coinbase_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_Coinbase_t>(),
        48usize,
        concat!("Size of: ", stringify!(PacketCrypt_Coinbase_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_Coinbase_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_Coinbase_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).magic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(magic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).annLeastWorkTarget as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(annLeastWorkTarget)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).merkleRoot as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(merkleRoot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).numAnns as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(numAnns)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_ValidateCtx_s {
    _unused: [u8; 0],
}
pub type PacketCrypt_ValidateCtx_t = PacketCrypt_ValidateCtx_s;
extern "C" {
    pub fn ValidateCtx_create() -> *mut PacketCrypt_ValidateCtx_t;
}
extern "C" {
    pub fn ValidateCtx_destroy(ctx: *mut PacketCrypt_ValidateCtx_t);
}
extern "C" {
    pub fn Validate_checkAnn_outToString(
        code: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Validate_checkAnn(
        annHashOut: *mut u8,
        pcAnn: *const PacketCrypt_Announce_t,
        parentBlockHash: *const u8,
        vctx: *mut PacketCrypt_ValidateCtx_t,
    ) -> ::std::os::raw::c_int;
}
pub const Validate_checkBlock_Res_Validate_checkBlock_OK: Validate_checkBlock_Res = 0;
pub const Validate_checkBlock_Res_Validate_checkBlock_SHARE_OK: Validate_checkBlock_Res = 256;
pub const Validate_checkBlock_Res_Validate_checkBlock_ANN_INVALID_: Validate_checkBlock_Res = 512;
pub const Validate_checkBlock_Res_Validate_checkBlock_ANN_INSUF_POW_: Validate_checkBlock_Res = 768;
pub const Validate_checkBlock_Res_Validate_checkBlock_ANN_SIG_INVALID_: Validate_checkBlock_Res =
    1024;
pub const Validate_checkBlock_Res_Validate_checkBlock_ANN_CONTENT_INVALID_:
    Validate_checkBlock_Res = 1280;
pub const Validate_checkBlock_Res_Validate_checkBlock_PCP_INVAL: Validate_checkBlock_Res = 1536;
pub const Validate_checkBlock_Res_Validate_checkBlock_PCP_MISMATCH: Validate_checkBlock_Res = 1792;
pub const Validate_checkBlock_Res_Validate_checkBlock_INSUF_POW: Validate_checkBlock_Res = 2048;
pub const Validate_checkBlock_Res_Validate_checkBlock_BAD_COINBASE: Validate_checkBlock_Res = 2304;
pub type Validate_checkBlock_Res = ::std::os::raw::c_uint;
extern "C" {
    pub fn Validate_checkBlock_outToString(
        code: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Validate_checkBlock(
        hap: *const PacketCrypt_HeaderAndProof_t,
        hapLen: u32,
        blockHeight: u32,
        shareTarget: u32,
        coinbaseCommitment: *const PacketCrypt_Coinbase_t,
        blockHashes: *const u8,
        workHashOut: *mut u8,
        vctx: *mut PacketCrypt_ValidateCtx_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnnMiner_s {
    _unused: [u8; 0],
}
pub type AnnMiner_t = AnnMiner_s;
pub type AnnMiner_Callback = ::std::option::Option<
    unsafe extern "C" fn(callback_ctx: *mut ::std::os::raw::c_void, ann_buf: *mut u8),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnnMiner_Request_s {
    pub workTarget: u32,
    pub parentBlockHeight: u32,
    pub parentBlockHash: [u8; 32usize],
    pub signingKey: [u8; 32usize],
    pub contentType: u32,
    pub contentLen: u32,
}
#[test]
fn bindgen_test_layout_AnnMiner_Request_s() {
    assert_eq!(
        ::std::mem::size_of::<AnnMiner_Request_s>(),
        80usize,
        concat!("Size of: ", stringify!(AnnMiner_Request_s))
    );
    assert_eq!(
        ::std::mem::align_of::<AnnMiner_Request_s>(),
        4usize,
        concat!("Alignment of ", stringify!(AnnMiner_Request_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).workTarget as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(workTarget)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AnnMiner_Request_s>())).parentBlockHeight as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(parentBlockHeight)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AnnMiner_Request_s>())).parentBlockHash as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(parentBlockHash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).signingKey as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(signingKey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).contentType as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(contentType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).contentLen as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(contentLen)
        )
    );
}
pub type AnnMiner_Request_t = AnnMiner_Request_s;
extern "C" {
    pub fn AnnMiner_create(
        minerId: u32,
        threads: ::std::os::raw::c_int,
        callback_ctx: *mut ::std::os::raw::c_void,
        ann_found: AnnMiner_Callback,
    ) -> *mut AnnMiner_t;
}
extern "C" {
    pub fn AnnMiner_start(
        ctx: *mut AnnMiner_t,
        req: *mut AnnMiner_Request_t,
        version: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn AnnMiner_stop(miner: *mut AnnMiner_t);
}
extern "C" {
    pub fn AnnMiner_free(miner: *mut AnnMiner_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProofTree_Entry_t {
    pub hash: [u8; 32usize],
    pub start: u64,
    pub end: u64,
}
#[test]
fn bindgen_test_layout_ProofTree_Entry_t() {
    assert_eq!(
        ::std::mem::size_of::<ProofTree_Entry_t>(),
        48usize,
        concat!("Size of: ", stringify!(ProofTree_Entry_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ProofTree_Entry_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ProofTree_Entry_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProofTree_Entry_t>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ProofTree_Entry_t),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProofTree_Entry_t>())).start as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ProofTree_Entry_t),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProofTree_Entry_t>())).end as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ProofTree_Entry_t),
            "::",
            stringify!(end)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProofTree_s {
    _unused: [u8; 0],
}
pub type ProofTree_t = ProofTree_s;
extern "C" {
    pub fn ProofTree_create(maxAnns: u32) -> *mut ProofTree_t;
}
extern "C" {
    pub fn ProofTree_destroy(arg1: *mut ProofTree_t);
}
extern "C" {
    pub fn ProofTree_hashPair(pt: *mut ProofTree_t, odx: u64, idx: u64);
}
extern "C" {
    pub fn ProofTree_complete(pt: *mut ProofTree_t, rootHashOut: *mut u8) -> u64;
}
extern "C" {
    pub fn ProofTree_putEntry(pt: *mut ProofTree_t, index: u32, entry: *const ProofTree_Entry_t);
}
extern "C" {
    pub fn ProofTree_prepare2(pt: *mut ProofTree_t, totalAnns: u64);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProofTree_Proof_s {
    pub size: u32,
    pub data: *mut u8,
}
#[test]
fn bindgen_test_layout_ProofTree_Proof_s() {
    assert_eq!(
        ::std::mem::size_of::<ProofTree_Proof_s>(),
        16usize,
        concat!("Size of: ", stringify!(ProofTree_Proof_s))
    );
    assert_eq!(
        ::std::mem::align_of::<ProofTree_Proof_s>(),
        8usize,
        concat!("Alignment of ", stringify!(ProofTree_Proof_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProofTree_Proof_s>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ProofTree_Proof_s),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ProofTree_Proof_s>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ProofTree_Proof_s),
            "::",
            stringify!(data)
        )
    );
}
pub type ProofTree_Proof_t = ProofTree_Proof_s;
extern "C" {
    pub fn ProofTree_mkProof(
        arg1: *mut ProofTree_t,
        annNumbers: *const u64,
    ) -> *mut ProofTree_Proof_t;
}
extern "C" {
    pub fn ProofTree_destroyProof(arg1: *mut ProofTree_Proof_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlockMine_Res_s {
    pub high_nonce: u32,
    pub low_nonce: u32,
    pub ann_mlocs: [u32; 4usize],
    pub ann_llocs: [u32; 4usize],
    pub job_num: u32,
}
#[test]
fn bindgen_test_layout_BlockMine_Res_s() {
    assert_eq!(
        ::std::mem::size_of::<BlockMine_Res_s>(),
        44usize,
        concat!("Size of: ", stringify!(BlockMine_Res_s))
    );
    assert_eq!(
        ::std::mem::align_of::<BlockMine_Res_s>(),
        4usize,
        concat!("Alignment of ", stringify!(BlockMine_Res_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Res_s>())).high_nonce as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Res_s),
            "::",
            stringify!(high_nonce)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Res_s>())).low_nonce as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Res_s),
            "::",
            stringify!(low_nonce)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Res_s>())).ann_mlocs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Res_s),
            "::",
            stringify!(ann_mlocs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Res_s>())).ann_llocs as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Res_s),
            "::",
            stringify!(ann_llocs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Res_s>())).job_num as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Res_s),
            "::",
            stringify!(job_num)
        )
    );
}
pub type BlockMine_Res_t = BlockMine_Res_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlockMine_s {
    pub maxAnns: u32,
}
#[test]
fn bindgen_test_layout_BlockMine_s() {
    assert_eq!(
        ::std::mem::size_of::<BlockMine_s>(),
        4usize,
        concat!("Size of: ", stringify!(BlockMine_s))
    );
    assert_eq!(
        ::std::mem::align_of::<BlockMine_s>(),
        4usize,
        concat!("Alignment of ", stringify!(BlockMine_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_s>())).maxAnns as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_s),
            "::",
            stringify!(maxAnns)
        )
    );
}
pub type BlockMine_t = BlockMine_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlockMine_Create_s {
    pub err: *const ::std::os::raw::c_char,
    pub stage: *const ::std::os::raw::c_char,
    pub miner: *mut BlockMine_t,
}
#[test]
fn bindgen_test_layout_BlockMine_Create_s() {
    assert_eq!(
        ::std::mem::size_of::<BlockMine_Create_s>(),
        24usize,
        concat!("Size of: ", stringify!(BlockMine_Create_s))
    );
    assert_eq!(
        ::std::mem::align_of::<BlockMine_Create_s>(),
        8usize,
        concat!("Alignment of ", stringify!(BlockMine_Create_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Create_s>())).err as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Create_s),
            "::",
            stringify!(err)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Create_s>())).stage as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Create_s),
            "::",
            stringify!(stage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BlockMine_Create_s>())).miner as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(BlockMine_Create_s),
            "::",
            stringify!(miner)
        )
    );
}
pub type BlockMine_Create_t = BlockMine_Create_s;
pub type BlockMine_Callback_t = ::std::option::Option<
    unsafe extern "C" fn(res: *mut BlockMine_Res_t, ctx: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn BlockMine_create(
        maxmem: u64,
        threads: ::std::os::raw::c_int,
        cb: BlockMine_Callback_t,
        cbc: *mut ::std::os::raw::c_void,
    ) -> BlockMine_Create_t;
}
extern "C" {
    pub fn BlockMine_destroy(bm: *mut BlockMine_t);
}
extern "C" {
    pub fn BlockMine_updateAnn(bm: *const BlockMine_t, mloc: u32, ann: *const u8);
}
extern "C" {
    pub fn BlockMine_getAnn(bm: *const BlockMine_t, mloc: u32, annOut: *mut u8);
}
extern "C" {
    pub fn BlockMine_getHashesPerSecond(bm: *const BlockMine_t) -> i64;
}
extern "C" {
    pub fn BlockMine_mine(
        bm: *mut BlockMine_t,
        header: *const u8,
        annCount: u32,
        annIndexes: *const u32,
        effectiveTarget: u32,
        jobNum: u32,
    );
}
extern "C" {
    pub fn BlockMine_stop(bm: *mut BlockMine_t);
}
extern "C" {
    pub fn BlockMine_fakeMine(
        bm: *mut BlockMine_t,
        resOut: *mut BlockMine_Res_t,
        header: *const u8,
        annCount: u32,
        annIndexes: *const u32,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UdpGro_Sockaddr {
    pub isIpv6: u16,
    pub port: u16,
    pub addr: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_UdpGro_Sockaddr() {
    assert_eq!(
        ::std::mem::size_of::<UdpGro_Sockaddr>(),
        20usize,
        concat!("Size of: ", stringify!(UdpGro_Sockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<UdpGro_Sockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(UdpGro_Sockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UdpGro_Sockaddr>())).isIpv6 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UdpGro_Sockaddr),
            "::",
            stringify!(isIpv6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UdpGro_Sockaddr>())).port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(UdpGro_Sockaddr),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UdpGro_Sockaddr>())).addr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(UdpGro_Sockaddr),
            "::",
            stringify!(addr)
        )
    );
}
extern "C" {
    pub fn UdpGso_supported() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn UdpGro_enable(
        fd: ::std::os::raw::c_int,
        pktSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn UdpGro_recvmsg(
        fd: ::std::os::raw::c_int,
        addrOut: *mut UdpGro_Sockaddr,
        buf: *mut u8,
        len: ::std::os::raw::c_int,
        pktSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn UdpGro_sendmsg(
        fd: ::std::os::raw::c_int,
        addr: *const UdpGro_Sockaddr,
        data: *const u8,
        length: ::std::os::raw::c_int,
        pktSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn UdpGro_setRecvBuf(
        fd: ::std::os::raw::c_int,
        bufSz: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExportMe {
    pub a: Validate_checkBlock_Res,
}
#[test]
fn bindgen_test_layout_ExportMe() {
    assert_eq!(
        ::std::mem::size_of::<ExportMe>(),
        4usize,
        concat!("Size of: ", stringify!(ExportMe))
    );
    assert_eq!(
        ::std::mem::align_of::<ExportMe>(),
        4usize,
        concat!("Alignment of ", stringify!(ExportMe))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExportMe>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ExportMe),
            "::",
            stringify!(a)
        )
    );
}
 
 