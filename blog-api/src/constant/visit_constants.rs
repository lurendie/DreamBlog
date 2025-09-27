#[derive(Debug)]
pub struct VisitBehavior {
    behavior: String,
    content: String,
    remark: String,
}

impl VisitBehavior {
    pub fn new(behavior: &str, content: &str, remark: &str) -> Self {
        Self {
            behavior: behavior.to_string(),
            content: content.to_string(),
            remark: remark.to_string(),
        }
    }

    pub fn get_behavior(&self) -> &str {
        &self.behavior
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn get_remark(&self) -> &str {
        &self.remark
    }

    pub fn set_remark(&mut self, remark: String) {
        self.remark = remark;
    }
}
#[derive(Debug, PartialEq)]
pub enum VisitBehaviorType {
    UNKNOWN,
    INDEX,
    ARCHIVE,
    MOMENT,
    FRIEND,
    ABOUT,
    BLOG,
    CATEGORY,
    TAG,
    SEARCH,
    ClickFriend,
    LikeMoment,
    CheckPassword,
}

impl From<&str> for VisitBehaviorType {
    fn from(behavior: &str) -> Self {
        match behavior {
            "/blogs" => Self::INDEX,
            "/archives" => Self::ARCHIVE,
            "/moments" => Self::MOMENT,
            "/friends" => Self::FRIEND,
            "/about" => Self::ABOUT,
            "/category" => Self::CATEGORY,
            "/tag" => Self::TAG,
            "/searchBlog" => Self::SEARCH,
            "/friend" => Self::ClickFriend,
            "/checkBlogPassword" => Self::CheckPassword,
             "/blog" => Self::BLOG,
            _ => {
                if behavior.contains("moment/like") {
                    Self::LikeMoment
                } else {
                    Self::UNKNOWN
                }
            }
        }
    }
}

impl From<VisitBehaviorType> for VisitBehavior {
    fn from(behavior: VisitBehaviorType) -> Self {
        match behavior {
            VisitBehaviorType::UNKNOWN => Self::new("UNKNOWN", "UNKNOWN", ""),
            VisitBehaviorType::INDEX => Self::new("访问页面", "首页", ""),
            VisitBehaviorType::ARCHIVE => Self::new("访问页面", "归档", ""),
            VisitBehaviorType::MOMENT => Self::new("访问页面", "动态", ""),
            VisitBehaviorType::FRIEND => Self::new("访问页面", "友链", ""),
            VisitBehaviorType::ABOUT => Self::new("访问页面", "关于我", ""),
            VisitBehaviorType::BLOG => Self::new("查看博客", "", ""),
            VisitBehaviorType::CATEGORY => Self::new("查看分类", "", ""),
            VisitBehaviorType::TAG => Self::new("查看标签", "", ""),
            VisitBehaviorType::SEARCH => Self::new("搜索博客", "", ""),
            VisitBehaviorType::ClickFriend => Self::new("点击友链", "", ""),
            VisitBehaviorType::LikeMoment => Self::new("点赞动态", "", ""),
            VisitBehaviorType::CheckPassword => Self::new("校验博客密码", "", ""),
        }
    }
}
