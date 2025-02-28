use anyhow::Result;
use ton_types::{BuilderData, Cell, CellType, IBitstring, LevelMask, UInt256};

const EMPTY_CELL_HASH: [u8; 32] = [
    0x96, 0xa2, 0x96, 0xd2, 0x24, 0xf2, 0x85, 0xc6, 0x7b, 0xee, 0x93, 0xc3, 0x0f, 0x8a, 0x30, 0x91,
    0x57, 0xf0, 0xda, 0xa3, 0x5d, 0xc5, 0xb8, 0x7e, 0x41, 0x0b, 0x78, 0x63, 0x0a, 0x09, 0xcf, 0xc7,
];

pub fn is_empty_cell(code_hash: &UInt256) -> bool {
    code_hash.as_slice() == &EMPTY_CELL_HASH
}

pub fn prune_deep_cells(cell: &Cell, after_depth: u16) -> Result<Cell> {
    fn prune_depp_cells_impl(cell: &Cell, after_depth: u16, depth: u16) -> Result<Cell> {
        if depth > after_depth {
            return make_pruned_branch_cell(cell, 0);
        }

        let ref_count = cell.references_count();
        if ref_count == 0 {
            return Ok(cell.clone());
        }

        let mut builder = BuilderData::new();
        for i in 0..ref_count {
            let cell = prune_depp_cells_impl(&cell.reference(i)?, after_depth, depth + 1)?;
            builder.append_reference_cell(cell);
        }

        builder.append_raw(cell.data(), cell.bit_length())?;
        builder.into_cell()
    }

    if cell.repr_depth() <= after_depth {
        return Ok(cell.clone());
    }

    prune_depp_cells_impl(cell, after_depth, 0)
}

pub fn make_pruned_branch_cell(cell: &Cell, merkle_depth: u8) -> Result<Cell> {
    let mut result = BuilderData::new();

    let level_mask = cell.level_mask().mask();
    let level_mask = LevelMask::with_mask(level_mask | (1 << merkle_depth));

    result.set_type(CellType::PrunedBranch);
    result.set_level_mask(level_mask);
    result.append_u8(u8::from(CellType::PrunedBranch))?;
    result.append_u8(level_mask.mask())?;
    for hash in cell.hashes() {
        result.append_raw(hash.as_slice(), hash.as_slice().len() * 8)?;
    }
    for depth in cell.depths() {
        result.append_u16(depth)?;
    }
    result.into_cell()
}
