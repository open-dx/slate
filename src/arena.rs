use spin::Mutex;
use spin::MutexGuard;

use once_cell::sync::OnceCell;

pub use bumpalo::Bump;
pub use bumpalo_herd::Herd;
pub use bumpalo_herd::Member;

static HERD: OnceCell<Herd> = OnceCell::new();

/// TODO
pub fn get<'alloc>() -> Member<'alloc> {
    HERD.get_or_init(|| Herd::new()).get()
}
