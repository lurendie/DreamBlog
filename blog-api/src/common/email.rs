use chrono::{DateTime, Local, NaiveDateTime};
use lettre::message::{header, Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

use crate::app::CONFIG;
use crate::error::EmailServerError;

#[derive(Debug, Clone)]
pub struct OwenrComment {
    pub post_title: String,    //文章标题
    pub post_url: String,      //文章地址
    pub nickname: String,      //评论人昵称
    pub content: String,       //评论内容
    pub time: DateTime<Local>, // 使用 chrono 的 DateTime
    pub ip: String,            //评论人IP
    pub email: String,         //评论人邮箱
    pub status: String,        //评论状态
    pub manage_url: String,    //管理地址
}
impl OwenrComment {
    pub fn new(
        post_title: String,
        post_url: String,
        nickname: String,
        content: String,
        time: DateTime<Local>,
        ip: String,
        email: String,
        status: String,
        manage_url: String,
    ) -> Self {
        OwenrComment {
            post_title,
            post_url,
            nickname,
            content,
            time,
            ip,
            email,
            status,
            manage_url,
        }
    }
}

const OWNER_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="zh_CN">
<head>
    <meta charset="UTF-8">
    <title>评论通知</title>
</head>
<body>
    <div style="width: 550px;height: auto;border-radius: 5px;margin: 0 auto;border: 1px solid #ffb0b0;box-shadow: 0px 0px 20px #888888;position: relative;">
        <div style="background-image: url(https://cdn.naccl.top/blog/img/mail.jpg);width: 550px;height: 250px;background-size: cover;background-repeat: no-repeat;border-radius: 5px 5px 0px 0px"></div>
        <div style="background-color: white;line-height: 180%;padding: 0 15px 12px;width: 520px;color: #555555;font-family: 'Century Gothic', 'Trebuchet MS', 'Hiragino Sans GB', 微软雅黑, 'Microsoft Yahei', Tahoma, Helvetica, Arial, 'SimSun', sans-serif;font-size: 12px;margin: 10px auto 0px">
            <h2 style="border-bottom: 1px solid #DDD;font-size: 14px;font-weight: normal;padding: 13px 0 10px 8px">
                您的文章<a style="text-decoration: none;color: #12ADDB" href="{post_url}" target="_blank">《{post_title}》</a>有了新的评论~</h2>
            <div style="padding: 0 12px 0 12px;margin-top: 18px">
                <p>时间：<span style="border-bottom: 1px dashed #ccc">{time}</span></p>
                <p><strong>{nickname}</strong>&nbsp;给您的评论：</p>
                <p style="background-color: #f5f5f5;border: 0px solid #DDD;padding: 10px 15px;margin: 18px 0">{content}</p>
                <p>其他信息：</p>
                <p style="background-color: #f5f5f5;border: 0px solid #DDD;padding: 10px 15px;margin: 18px 0"><span>IP：{ip}</span><br/><span>邮箱：{email}</span><br/><span>状态：{status}</span> [<a href="{manage_url}" target="_blank">管理评论</a>]</p>
            </div>
        </div>
        <a style="text-decoration: none;color: rgb(255, 255, 255);width: 40%;text-align: center;background-color: rgb(255, 114, 114);height: 40px;line-height: 40px;box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.3);display: block;margin: 0 auto 28px auto" href="{post_url}" target="_blank">查看回复的完整內容</a>
    </div>
</body>
</html>"#;

impl HtmlTemplate for OwenrComment {
    fn to_html(&self) -> String {
        let mut html = OWNER_HTML_TEMPLATE.to_string();
        // 使用 replace 方法逐个替换占位符（用户输入字段先做 HTML 转义，防注入）
        html = html.replace("{post_title}", &escape_html(&self.post_title));
        html = html.replace("{post_url}", &self.post_url);
        html = html.replace("{nickname}", &escape_html(&self.nickname));
        html = html.replace("{content}", &escape_html(&self.content));

        // 格式化日期
        let formatted_time = self.time.format("%Y-%m-%d %H:%M").to_string();
        html = html.replace("{time}", &formatted_time);

        html = html.replace("{ip}", &escape_html(&self.ip));
        html = html.replace("{email}", &escape_html(&self.email));
        html = html.replace("{status}", &escape_html(&self.status));
        html = html.replace("{manage_url}", &self.manage_url);
        html
    }
}

#[derive(Debug, Clone)]
pub struct GuestReply {
    pub post_title: String,      //文章标题
    pub post_url: String,        //文章地址
    pub parent_nickname: String, // 被回复者的昵称
    pub parent_content: String,  // 被回复的评论内容
    pub nickname: String,        // 回复者的昵称
    pub content: String,         // 回复的内容
    pub time: NaiveDateTime,     // 回复的时间
}

impl GuestReply {
    pub fn new(
        post_title: String,
        post_url: String,
        parent_nickname: String,
        parent_content: String,
        nickname: String,
        content: String,
        time: NaiveDateTime,
    ) -> Self {
        GuestReply {
            post_title,
            post_url,
            parent_nickname,
            parent_content,
            nickname,
            content,
            time,
        }
    }
}

const GUEST_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="zh_CN">

<head>
    <meta charset="UTF-8">
    <title>回复通知</title>
</head>

<body>
    <div
        style="width: 550px;height: auto;border-radius: 5px;margin: 0 auto;border: 1px solid #ffb0b0;box-shadow: 0px 0px 20px #888888;position: relative;padding-bottom: 5px">
        <div
            style="background-image: url(https://cdn.naccl.top/blog/img/mail.jpg);width: 550px;height: 300px;background-size: cover;background-repeat: no-repeat;border-radius: 5px 5px 0px 0px">
        </div>
        <div style="width: 200px;height: 40px;background-color: rgb(255, 114, 114);margin-top: -20px;margin-left: 20px;box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.3);color: rgb(255, 255, 255);text-align: center;line-height: 40px"
            >Dear: {parent_nickname}</div>
        <div
            style="background-color: white;line-height: 180%;padding: 0 15px 12px;width: 520px;margin: 30px auto;color: #555555;font-family: 'Century Gothic', 'Trebuchet MS', 'Hiragino Sans GB', 微软雅黑, 'Microsoft Yahei', Tahoma, Helvetica, Arial, 'SimSun', sans-serif;font-size: 12px;margin-bottom: 0px">
            <h2 style="border-bottom: 1px solid #ddd;font-size: 14px;font-weight: normal;padding: 13px 0 10px 8px">
                您在<a style="text-decoration: none;color: #12addb" href="{post_url}" target="_blank"
                    >《{post_title}》</a>的评论有了新的回复~
            </h2>
            <div style="padding: 0 12px 0 12px;margin-top: 18px">
                <p>时间：<span style="border-bottom: 1px dashed #ccc"
                        >{time}</span></p>
                <p>您的评论：</p>
                <p style="background-color: #f5f5f5;border: 0px solid #ddd;padding: 10px 15px;margin: 18px 0"
                    >{parent_content}</p>
                <p><strong>{nickname}</strong>&nbsp;给您的回复：</p>
                <p style="background-color: #f5f5f5;border: 0px solid #ddd;padding: 10px 15px;margin: 18px 0"
                    >{content}</p>
            </div>
        </div>
        <div
            style="color: #8c8c8c;font-family: 'Century Gothic', 'Trebuchet MS', 'Hiragino Sans GB', 微软雅黑, 'Microsoft Yahei', Tahoma, Helvetica, Arial, 'SimSun', sans-serif;font-size: 10px;width: 100%;text-align: center;word-wrap: break-word;margin-top: -30px">
            <p style="padding: 20px">萤火虫消失之后，那光的轨迹仍久久地印在我的脑际。那微弱浅淡的光点，仿佛迷失方向的魂灵，在漆黑厚重的夜幕中彷徨。——《挪威的森林》村上春树</p>
        </div>
        <a style="text-decoration: none;color: #fff;width: 40%;text-align: center;background-color: #ff7272;height: 40px;line-height: 35px;box-shadow: 3px 3px 3px rgba(0, 0, 0, 0.3);margin: -10px auto;display: block"
            href="{post_url}" target="_blank">查看回复的完整內容</a>
        <div
            style="color: #8c8c8c;font-family: 'Century Gothic', 'Trebuchet MS', 'Hiragino Sans GB', 微软雅黑, 'Microsoft Yahei', Tahoma, Helvetica, Arial, 'SimSun', sans-serif;font-size: 10px;width: 100%;text-align: center;margin-top: 30px">
            <p>本邮件为系统自动发送，回复TD退订~</p>
        </div>
    </div>
</body>

</html>"#;

impl HtmlTemplate for GuestReply {
    fn to_html(&self) -> String {
        let mut html = GUEST_HTML_TEMPLATE.to_string();
        html = html.replace("{post_title}", &escape_html(&self.post_title));
        html = html.replace("{post_url}", &self.post_url);
        html = html.replace("{parent_nickname}", &escape_html(&self.parent_nickname));
        html = html.replace("{parent_content}", &escape_html(&self.parent_content));
        html = html.replace("{nickname}", &escape_html(&self.nickname));
        html = html.replace("{content}", &escape_html(&self.content));
        // 格式化日期
        let formatted_time = self.time.format("%Y-%m-%d %H:%M").to_string();
        html = html.replace("{time}", &formatted_time);
        html
    }
}

/// 对用户输入做 HTML 转义，防止注入邮件模板
fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub trait HtmlTemplate {
    fn to_html(&self) -> String;
}

pub enum EmailType {
    GuestReply,
    OwenrComment,
}

impl EmailType {
    pub fn get_type(is_notice: bool, is_parent: bool) -> Result<Self, EmailServerError> {
        if matches!(is_notice, true) {
            //是否是顶级评论
            if is_parent {
                Ok(EmailType::OwenrComment)
            } else {
                Ok(EmailType::GuestReply)
            }
        } else {
            return Err(EmailServerError::NotSend);
        }
    }
}

pub struct EmailServer;
impl EmailServer {
    pub async fn send_email(
        send_body: impl HtmlTemplate,
        email_type: EmailType,
        recipient_email: &str,
    ) -> Result<(), EmailServerError> {
        log::info!("发送邮件给: {}", recipient_email);
        let email_config = CONFIG.get_email_config();
        let email_message_build = Message::builder()
            .from(email_config.username.as_str().parse()?)
            .to(recipient_email.parse()?);
        let email_message_build = match email_type {
            EmailType::GuestReply => email_message_build.subject("您在博客的评论有了新的回复~"),
            EmailType::OwenrComment => email_message_build.subject("您的博客文章收到新的评论~"),
        }; // 设置邮件主题
        let email_message = email_message_build.singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(send_body.to_html()), // 使用我们动态生成的HTML
        )?;
        let email_config = CONFIG.get_email_config();
        let smtp_username = email_config.username;
        let smtp_password = email_config.password;
        // 邮箱认证信息
        // 注意：QQ邮箱需要使用授权码而不是密码
        // 授权码需要在QQ邮箱设置中生成
        let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
        let mailer = match SmtpTransport::relay(&email_config.host) {
            Ok(smtp) => smtp.port(email_config.port).credentials(creds).build(),
            Err(e) => return Err(EmailServerError::Custom(e.to_string())),
        };
        match mailer.send(&email_message) {
            Ok(_) => Ok(()),
            Err(e) => return Err(EmailServerError::Custom(e.to_string())),
        }
    }
}
