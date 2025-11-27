pub mod blob;
pub mod commit;
pub mod tree;
pub mod tag;
pub mod object;

pub enum RGitObjectTypes {
    Blob(BlobObject),
    Commit(CommitObject),
    Tree(TreeObject),
    Tag(TagObject)
}

pub use object::*;
pub use blob::*;
pub use commit::*;
pub use tree::*;
pub use tag::*;
