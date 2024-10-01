mod uint256;

use uint256::Uint256;

fn main() {
    let superblock_start_hash =
        Uint256::from_hex("0000000000020cb27c7ef164d21003d5d20cdca2f54dd9a9ca6d45f4d47f8aa3")
            .unwrap();
    dbg!(&superblock_start_hash);

    // consensus.nSuperblockStartHash = uint256S("0000000000020cb27c7ef164d21003d5d20cdca2f54dd9a9ca6d45f4d47f8aa3");

    // consensus.nSuperblockStartHash = uint256S("0000000000020cb27c7ef164d21003d5d20cdca2f54dd9a9ca6d45f4d47f8aa3");
    // consensus.BIP34Hash = uint256S("0x000001f35e70f7c5705f64c6c5cc3dea9449e74d5b5c7cf74dad1bcca14a8012");
    // consensus.DIP0003EnforcementHash = uint256S("000000000000002d1734087b4c5afc3133e4e1c3e1a89218f62bcd9bb3d17f81")
    // ;
    // consensus.powLimit = uint256S("00000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    // consensus.nMinimumChainWork = uint256S("0x00000000000000000000000000000000000000000000988117deadb0db9cd5b8"); /
    // consensus.defaultAssumeValid = uint256S("0x000000000000001889bd33ef019065e250d32bd46911f4003d3fdd8128b5358d");
}
