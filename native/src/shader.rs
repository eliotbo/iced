//! Load and draw raster graphics.
use crate::Hasher;

use std::hash::{Hash, Hasher as _};
use std::path::PathBuf;

/// A handle of some shader code.
#[derive(Debug, Clone)]
pub struct Handle {
    /// A unique identifier for the shader.
    pub id: u64,

    /// Either the path to the shader code or a reference to the shader code in memory.
    pub shader_content: ShaderContent,
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Handle {
    /// Returns the unique identifier of the [`Handle`].
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl From<ShaderContent> for Handle {
    fn from(shader_content: ShaderContent) -> Handle {
        let mut hasher = Hasher::default();

        match shader_content.clone() {
            ShaderContent::Path(path) => {
                path.hash(&mut hasher);
                Handle {
                    id: hasher.finish(),
                    shader_content: shader_content,
                }
            }
            ShaderContent::Memory(memory) => {
                memory.hash(&mut hasher);
                Handle {
                    id: hasher.finish(),
                    shader_content: shader_content,
                }
            }
        }
    }
}

impl Hash for Handle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Either a path to the shader code or the code itself.
#[derive(Clone, Hash)]
pub enum ShaderContent {
    /// Shader in a file
    Path(PathBuf),
    /// Shader in memory
    Memory(&'static str),
}

impl std::fmt::Debug for ShaderContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderContent::Path(path) => write!(f, "Path({:?})", path),
            ShaderContent::Memory(_) => write!(f, "shader in memory"),
        }
    }
}
