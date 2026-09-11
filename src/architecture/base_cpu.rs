pub trait BaseCPU {
    fn id() -> usize;

    fn halt() -> !;
}
