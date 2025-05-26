/// High-level commands returned by an `AiController`.
///
/// For Sprint 1 we only need `Stay` and `Move`.  
/// `Move` stores axial **dq, dr** – i.e. the delta from the unit’s current hex.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Stay,
    Move(i32 /*dq*/, i32 /*dr*/),
}
