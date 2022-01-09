pub mod windows;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState{
  AssetLoading,
  Run,
}