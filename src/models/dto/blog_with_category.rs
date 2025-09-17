use serde::{Deserialize, Serialize};

use crate::models::{blog::Blog, category::Category};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlogWithCategoryDto {
    
    blog: Blog,
    
    category: Category,
}
