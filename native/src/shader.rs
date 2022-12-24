//! Load and draw raster graphics.
use crate::Hasher;

use std::hash::{Hash, Hasher as _};
use std::path::PathBuf;

/// A handle of some shader code.
#[derive(Debug, Clone)]
pub struct Handle {
    /// A unique identifier for the shader.
    pub id: u64,
    /// The path to the shader code.
    pub path: PathBuf,
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Handle {
    /// Creates an shader [`Handle`] pointing to a shader with the given path.
    pub fn from_path<T: Into<PathBuf> + Clone>(path: T) -> Handle {
        // Self::from_data(ShaderCode::Path(path.into()))

        let shader_path = path.clone().into();

        let mut hasher = Hasher::default();
        shader_path.hash(&mut hasher);

        Handle {
            id: hasher.finish(),
            path: path.into(),
        }
    }

    /// Returns the unique identifier of the [`Handle`].
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl<T> From<T> for Handle
where
    T: Into<PathBuf>,
{
    fn from(path: T) -> Handle {
        Handle::from_path(path.into())
    }
}

impl Hash for Handle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Either a path to the shader code or the code itself.
#[derive(Clone, Hash)]
pub enum ShaderPath {
    /// Shader in a file
    Path(PathBuf),
}

impl std::fmt::Debug for ShaderPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderPath::Path(path) => write!(f, "Path({:?})", path),
        }
    }
}
