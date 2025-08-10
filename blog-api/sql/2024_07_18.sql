/*
 Navicat Premium Data Transfer

 Source Server         : 本机
 Source Server Type    : MySQL
 Source Server Version : 80031
 Source Host           : 127.0.0.1:3306
 Source Schema         : zero_api

 Target Server Type    : MySQL
 Target Server Version : 80031
 File Encoding         : 65001

 Date: 18/07/2024 18:55:57
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for about
-- ----------------------------
DROP TABLE IF EXISTS `about`;
CREATE TABLE `about`  (
  `id` bigint NOT NULL,
  `name_en` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL,
  `name_zh` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL,
  `value` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of about
-- ----------------------------
INSERT INTO `about` VALUES (1, 'title', '标题', '关于帅气的 Naccl');
INSERT INTO `about` VALUES (2, 'musicId', '网易云歌曲ID', '423015580');
INSERT INTO `about` VALUES (3, 'content', '正文Markdown', '');
INSERT INTO `about` VALUES (4, 'commentEnabled', '评论开关', 'true');

-- ----------------------------
-- Table structure for blog
-- ----------------------------
DROP TABLE IF EXISTS `blog`;
CREATE TABLE `blog`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '文章标题',
  `first_picture` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '文章首图，用于随机文章展示',
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '文章正文',
  `description` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '描述',
  `is_published` tinyint(1) NOT NULL COMMENT '公开或私密',
  `is_recommend` tinyint(1) NOT NULL COMMENT '推荐开关',
  `is_appreciation` tinyint(1) NOT NULL COMMENT '赞赏开关',
  `is_comment_enabled` tinyint(1) NOT NULL COMMENT '评论开关',
  `is_top` tinyint(1) NOT NULL COMMENT '置顶开关',
  `create_time` datetime NOT NULL COMMENT '创建时间',
  `update_time` datetime NOT NULL ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `views` int NOT NULL COMMENT '浏览次数',
  `words` int NOT NULL COMMENT '文章字数',
  `read_time` int NOT NULL COMMENT '阅读时长(分钟)',
  `category_id` bigint NOT NULL COMMENT '文章分类',
  `password` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '密码保护',
  `user_id` bigint NULL DEFAULT NULL COMMENT '文章作者',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `type_id`(`category_id`) USING BTREE,
  INDEX `user_id`(`user_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 15 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of blog
-- ----------------------------
INSERT INTO `blog` VALUES (2, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '%%隐藏文字%%', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 1, 1, 1, 0, '2023-09-16 02:09:24', '2024-07-18 18:25:26', 0, 10000, 50, 1, '', 1);
INSERT INTO `blog` VALUES (3, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2023-09-16 01:09:24', '2024-07-18 18:25:27', 0, 10000, 50, 1, NULL, 1);
INSERT INTO `blog` VALUES (4, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 1, 0, 1, 0, '2023-09-15 02:09:24', '2024-07-18 18:25:27', 0, 10000, 50, 1, NULL, 1);
INSERT INTO `blog` VALUES (5, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2023-09-01 02:09:24', '2024-07-18 18:25:27', 0, 10000, 50, 1, NULL, 1);
INSERT INTO `blog` VALUES (6, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2023-08-01 02:09:24', '2024-07-18 18:25:28', 0, 10000, 50, 2, NULL, 1);
INSERT INTO `blog` VALUES (7, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2023-07-27 02:09:24', '2024-07-18 18:25:29', 0, 10000, 50, 2, NULL, 1);
INSERT INTO `blog` VALUES (8, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2023-07-19 02:09:24', '2024-07-18 18:25:30', 0, 10000, 50, 1, NULL, 1);
INSERT INTO `blog` VALUES (9, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2023-07-19 02:09:24', '2024-07-18 18:25:30', 0, 10000, 50, 1, NULL, 1);
INSERT INTO `blog` VALUES (10, '测试', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '## 测试 ## 测试评论', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 1, 0, '2024-02-19 02:09:24', '2024-07-18 18:25:30', 0, 10000, 50, 1, NULL, 1);
INSERT INTO `blog` VALUES (14, '测试文章2', 'https://bu.dusays.com/2024/05/12/6640a9171101d.jpg', '# 测试文章2\n> 测试文章2\n1. 测试1\n2. 测试2\n3. 测速', '![测试图片](https://bu.dusays.com/2024/05/12/6640a91a56664.jpg)', 1, 0, 0, 0, 0, '2024-07-14 23:49:38', '2024-07-18 18:25:31', 0, 100, 1, 1, '', NULL);

-- ----------------------------
-- Table structure for blog_tag
-- ----------------------------
DROP TABLE IF EXISTS `blog_tag`;
CREATE TABLE `blog_tag`  (
  `blog_id` bigint NOT NULL,
  `tag_id` bigint NOT NULL
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of blog_tag
-- ----------------------------
INSERT INTO `blog_tag` VALUES (4, 1);
INSERT INTO `blog_tag` VALUES (5, 1);
INSERT INTO `blog_tag` VALUES (6, 1);
INSERT INTO `blog_tag` VALUES (7, 1);
INSERT INTO `blog_tag` VALUES (8, 1);
INSERT INTO `blog_tag` VALUES (3, 1);
INSERT INTO `blog_tag` VALUES (2, 1);
INSERT INTO `blog_tag` VALUES (2, 4);
INSERT INTO `blog_tag` VALUES (14, 1);
INSERT INTO `blog_tag` VALUES (14, 2);
INSERT INTO `blog_tag` VALUES (14, 3);
INSERT INTO `blog_tag` VALUES (14, 4);

-- ----------------------------
-- Table structure for category
-- ----------------------------
DROP TABLE IF EXISTS `category`;
CREATE TABLE `category`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `category_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 5 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of category
-- ----------------------------
INSERT INTO `category` VALUES (1, '测试');
INSERT INTO `category` VALUES (2, '测试2');
INSERT INTO `category` VALUES (3, '测试3');
INSERT INTO `category` VALUES (4, '测试4');

-- ----------------------------
-- Table structure for city_visitor
-- ----------------------------
DROP TABLE IF EXISTS `city_visitor`;
CREATE TABLE `city_visitor`  (
  `city` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '城市名称',
  `uv` int NOT NULL COMMENT '独立访客数量',
  PRIMARY KEY (`city`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of city_visitor
-- ----------------------------

-- ----------------------------
-- Table structure for comment
-- ----------------------------
DROP TABLE IF EXISTS `comment`;
CREATE TABLE `comment`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `nickname` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '昵称',
  `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '邮箱',
  `content` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '评论内容',
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '头像(图片路径)',
  `create_time` datetime NULL DEFAULT NULL COMMENT '评论时间',
  `ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '评论者ip地址',
  `is_published` tinyint(1) NOT NULL COMMENT '公开或回收站',
  `is_admin_comment` tinyint(1) NOT NULL COMMENT '博主回复',
  `page` tinyint NOT NULL COMMENT '0普通文章，1关于我页面，2友链页面',
  `is_notice` tinyint(1) NOT NULL COMMENT '接收邮件提醒',
  `blog_id` bigint NULL DEFAULT NULL COMMENT '所属的文章',
  `parent_comment_id` bigint NOT NULL COMMENT '父评论id，-1为根评论',
  `website` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '个人网站',
  `qq` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '如果评论昵称为QQ号，则将昵称和头像置为QQ昵称和QQ头像，并将此字段置为QQ号备份',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 8 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of comment
-- ----------------------------
INSERT INTO `comment` VALUES (1, '评论1', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 1, 0, 0, 1, 10, -1, NULL, NULL);
INSERT INTO `comment` VALUES (2, '评论2', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 1, 0, 0, 1, 10, 1, NULL, NULL);
INSERT INTO `comment` VALUES (3, '评论3', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 1, 0, 0, 1, 10, 2, NULL, NULL);
INSERT INTO `comment` VALUES (4, '评论4', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 1, 0, 0, 1, 10, 3, NULL, NULL);
INSERT INTO `comment` VALUES (5, '评论5', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 1, 0, 0, 1, 10, -1, NULL, NULL);
INSERT INTO `comment` VALUES (6, '评论6', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 1, 0, 0, 1, 10, 5, NULL, NULL);
INSERT INTO `comment` VALUES (7, '评论7', '1', '1', '/img/comment-avatar/3.jpg', '2023-09-19 22:06:25', '1', 0, 0, 0, 1, 10, -1, NULL, NULL);

-- ----------------------------
-- Table structure for exception_log
-- ----------------------------
DROP TABLE IF EXISTS `exception_log`;
CREATE TABLE `exception_log`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `uri` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求接口',
  `method` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求方式',
  `param` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '请求参数',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作描述',
  `error` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL COMMENT '异常信息',
  `ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip',
  `ip_source` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip来源',
  `os` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作系统',
  `browser` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '浏览器',
  `create_time` datetime NOT NULL COMMENT '操作时间',
  `user_agent` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'user-agent用户代理',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of exception_log
-- ----------------------------
INSERT INTO `exception_log` VALUES (1, '/blog', 'GET', '{\"id\":5}', '查看博客', 'top.naccl.exception.NotFoundException: 该博客不存在\r\n	at top.naccl.service.impl.BlogServiceImpl.getBlogByIdAndIsPublished(BlogServiceImpl.java:356)\r\n	at top.naccl.service.impl.BlogServiceImpl$$FastClassBySpringCGLIB$$5890a2ac.invoke(<generated>)\r\n	at org.springframework.cglib.proxy.MethodProxy.invoke(MethodProxy.java:218)\r\n	at org.springframework.aop.framework.CglibAopProxy$DynamicAdvisedInterceptor.intercept(CglibAopProxy.java:687)\r\n	at top.naccl.service.impl.BlogServiceImpl$$EnhancerBySpringCGLIB$$e5da2649.getBlogByIdAndIsPublished(<generated>)\r\n	at top.naccl.controller.BlogController.getBlog(BlogController.java:63)\r\n	at top.naccl.controller.BlogController$$FastClassBySpringCGLIB$$942814c2.invoke(<generated>)\r\n	at org.springframework.cglib.proxy.MethodProxy.invoke(MethodProxy.java:218)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.invokeJoinpoint(CglibAopProxy.java:771)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:163)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.aspectj.MethodInvocationProceedingJoinPoint.proceed(MethodInvocationProceedingJoinPoint.java:88)\r\n	at top.naccl.aspect.VisitLogAspect.logAround(VisitLogAspect.java:67)\r\n	at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Native Method)\r\n	at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)\r\n	at java.base/jdk.internal.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)\r\n	at java.base/java.lang.reflect.Method.invoke(Method.java:566)\r\n	at org.springframework.aop.aspectj.AbstractAspectJAdvice.invokeAdviceMethodWithGivenArgs(AbstractAspectJAdvice.java:644)\r\n	at org.springframework.aop.aspectj.AbstractAspectJAdvice.invokeAdviceMethod(AbstractAspectJAdvice.java:633)\r\n	at org.springframework.aop.aspectj.AspectJAroundAdvice.invoke(AspectJAroundAdvice.java:70)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:175)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.aspectj.AspectJAfterThrowingAdvice.invoke(AspectJAfterThrowingAdvice.java:62)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:186)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.interceptor.ExposeInvocationInterceptor.invoke(ExposeInvocationInterceptor.java:95)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:186)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.framework.CglibAopProxy$DynamicAdvisedInterceptor.intercept(CglibAopProxy.java:691)\r\n	at top.naccl.controller.BlogController$$EnhancerBySpringCGLIB$$8b74c7c5.getBlog(<generated>)\r\n	at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Native Method)\r\n	at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)\r\n	at java.base/jdk.internal.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)\r\n	at java.base/java.lang.reflect.Method.invoke(Method.java:566)\r\n	at org.springframework.web.method.support.InvocableHandlerMethod.doInvoke(InvocableHandlerMethod.java:190)\r\n	at org.springframework.web.method.support.InvocableHandlerMethod.invokeForRequest(InvocableHandlerMethod.java:138)\r\n	at org.springframework.web.servlet.mvc.method.annotation.ServletInvocableHandlerMethod.invokeAndHandle(ServletInvocableHandlerMethod.java:105)\r\n	at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.invokeHandlerMethod(RequestMappingHandlerAdapter.java:879)\r\n	at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.handleInternal(RequestMappingHandlerAdapter.java:793)\r\n	at org.springframework.web.servlet.mvc.method.AbstractHandlerMethodAdapter.handle(AbstractHandlerMethodAdapter.java:87)\r\n	at org.springframework.web.servlet.DispatcherServlet.doDispatch(DispatcherServlet.java:1040)\r\n	at org.springframework.web.servlet.DispatcherServlet.doService(DispatcherServlet.java:943)\r\n	at org.springframework.web.servlet.FrameworkServlet.processRequest(FrameworkServlet.java:1006)\r\n	at org.springframework.web.servlet.FrameworkServlet.doGet(FrameworkServlet.java:898)\r\n	at javax.servlet.http.HttpServlet.service(HttpServlet.java:634)\r\n	at org.springframework.web.servlet.FrameworkServlet.service(FrameworkServlet.java:883)\r\n	at javax.servlet.http.HttpServlet.service(HttpServlet.java:741)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:231)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.apache.tomcat.websocket.server.WsFilter.doFilter(WsFilter.java:53)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:320)\r\n	at org.springframework.security.web.access.intercept.FilterSecurityInterceptor.invoke(FilterSecurityInterceptor.java:126)\r\n	at org.springframework.security.web.access.intercept.FilterSecurityInterceptor.doFilter(FilterSecurityInterceptor.java:90)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.access.ExceptionTranslationFilter.doFilter(ExceptionTranslationFilter.java:118)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.session.SessionManagementFilter.doFilter(SessionManagementFilter.java:137)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.authentication.AnonymousAuthenticationFilter.doFilter(AnonymousAuthenticationFilter.java:111)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.servletapi.SecurityContextHolderAwareRequestFilter.doFilter(SecurityContextHolderAwareRequestFilter.java:158)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.savedrequest.RequestCacheAwareFilter.doFilter(RequestCacheAwareFilter.java:63)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at top.naccl.config.JwtFilter.doFilter(JwtFilter.java:35)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.authentication.AbstractAuthenticationProcessingFilter.doFilter(AbstractAuthenticationProcessingFilter.java:200)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.authentication.logout.LogoutFilter.doFilter(LogoutFilter.java:116)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.web.filter.CorsFilter.doFilterInternal(CorsFilter.java:92)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.header.HeaderWriterFilter.doHeadersAfter(HeaderWriterFilter.java:92)\r\n	at org.springframework.security.web.header.HeaderWriterFilter.doFilterInternal(HeaderWriterFilter.java:77)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.context.SecurityContextPersistenceFilter.doFilter(SecurityContextPersistenceFilter.java:105)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.context.request.async.WebAsyncManagerIntegrationFilter.doFilterInternal(WebAsyncManagerIntegrationFilter.java:56)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.FilterChainProxy.doFilterInternal(FilterChainProxy.java:215)\r\n	at org.springframework.security.web.FilterChainProxy.doFilter(FilterChainProxy.java:178)\r\n	at org.springframework.web.filter.DelegatingFilterProxy.invokeDelegate(DelegatingFilterProxy.java:358)\r\n	at org.springframework.web.filter.DelegatingFilterProxy.doFilter(DelegatingFilterProxy.java:271)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.web.filter.RequestContextFilter.doFilterInternal(RequestContextFilter.java:100)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.web.filter.FormContentFilter.doFilterInternal(FormContentFilter.java:93)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.web.filter.CharacterEncodingFilter.doFilterInternal(CharacterEncodingFilter.java:201)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.apache.catalina.core.StandardWrapperValve.invoke(StandardWrapperValve.java:202)\r\n	at org.apache.catalina.core.StandardContextValve.invoke(StandardContextValve.java:96)\r\n	at org.apache.catalina.authenticator.AuthenticatorBase.invoke(AuthenticatorBase.java:541)\r\n	at org.apache.catalina.core.StandardHostValve.invoke(StandardHostValve.java:139)\r\n	at org.apache.catalina.valves.ErrorReportValve.invoke(ErrorReportValve.java:92)\r\n	at org.apache.catalina.core.StandardEngineValve.invoke(StandardEngineValve.java:74)\r\n	at org.apache.catalina.connector.CoyoteAdapter.service(CoyoteAdapter.java:343)\r\n	at org.apache.coyote.http11.Http11Processor.service(Http11Processor.java:373)\r\n	at org.apache.coyote.AbstractProcessorLight.process(AbstractProcessorLight.java:65)\r\n	at org.apache.coyote.AbstractProtocol$ConnectionHandler.process(AbstractProtocol.java:868)\r\n	at org.apache.tomcat.util.net.NioEndpoint$SocketProcessor.doRun(NioEndpoint.java:1590)\r\n	at org.apache.tomcat.util.net.SocketProcessorBase.run(SocketProcessorBase.java:49)\r\n	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1128)\r\n	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:628)\r\n	at org.apache.tomcat.util.threads.TaskThread$WrappingRunnable.run(TaskThread.java:61)\r\n	at java.base/java.lang.Thread.run(Thread.java:834)\r\n', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', '2023-08-20 23:56:52', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `exception_log` VALUES (2, '/blog', 'GET', '{\"id\":6}', '查看博客', 'top.naccl.exception.NotFoundException: 该博客不存在\r\n	at top.naccl.service.impl.BlogServiceImpl.getBlogByIdAndIsPublished(BlogServiceImpl.java:356)\r\n	at top.naccl.service.impl.BlogServiceImpl$$FastClassBySpringCGLIB$$5890a2ac.invoke(<generated>)\r\n	at org.springframework.cglib.proxy.MethodProxy.invoke(MethodProxy.java:218)\r\n	at org.springframework.aop.framework.CglibAopProxy$DynamicAdvisedInterceptor.intercept(CglibAopProxy.java:687)\r\n	at top.naccl.service.impl.BlogServiceImpl$$EnhancerBySpringCGLIB$$c6598e6f.getBlogByIdAndIsPublished(<generated>)\r\n	at top.naccl.controller.BlogController.getBlog(BlogController.java:63)\r\n	at top.naccl.controller.BlogController$$FastClassBySpringCGLIB$$942814c2.invoke(<generated>)\r\n	at org.springframework.cglib.proxy.MethodProxy.invoke(MethodProxy.java:218)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.invokeJoinpoint(CglibAopProxy.java:771)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:163)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.aspectj.MethodInvocationProceedingJoinPoint.proceed(MethodInvocationProceedingJoinPoint.java:88)\r\n	at top.naccl.aspect.VisitLogAspect.logAround(VisitLogAspect.java:67)\r\n	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)\r\n	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)\r\n	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)\r\n	at java.lang.reflect.Method.invoke(Method.java:498)\r\n	at org.springframework.aop.aspectj.AbstractAspectJAdvice.invokeAdviceMethodWithGivenArgs(AbstractAspectJAdvice.java:644)\r\n	at org.springframework.aop.aspectj.AbstractAspectJAdvice.invokeAdviceMethod(AbstractAspectJAdvice.java:633)\r\n	at org.springframework.aop.aspectj.AspectJAroundAdvice.invoke(AspectJAroundAdvice.java:70)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:175)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.aspectj.AspectJAfterThrowingAdvice.invoke(AspectJAfterThrowingAdvice.java:62)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:186)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.interceptor.ExposeInvocationInterceptor.invoke(ExposeInvocationInterceptor.java:95)\r\n	at org.springframework.aop.framework.ReflectiveMethodInvocation.proceed(ReflectiveMethodInvocation.java:186)\r\n	at org.springframework.aop.framework.CglibAopProxy$CglibMethodInvocation.proceed(CglibAopProxy.java:749)\r\n	at org.springframework.aop.framework.CglibAopProxy$DynamicAdvisedInterceptor.intercept(CglibAopProxy.java:691)\r\n	at top.naccl.controller.BlogController$$EnhancerBySpringCGLIB$$8a24e948.getBlog(<generated>)\r\n	at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)\r\n	at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)\r\n	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)\r\n	at java.lang.reflect.Method.invoke(Method.java:498)\r\n	at org.springframework.web.method.support.InvocableHandlerMethod.doInvoke(InvocableHandlerMethod.java:190)\r\n	at org.springframework.web.method.support.InvocableHandlerMethod.invokeForRequest(InvocableHandlerMethod.java:138)\r\n	at org.springframework.web.servlet.mvc.method.annotation.ServletInvocableHandlerMethod.invokeAndHandle(ServletInvocableHandlerMethod.java:105)\r\n	at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.invokeHandlerMethod(RequestMappingHandlerAdapter.java:879)\r\n	at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.handleInternal(RequestMappingHandlerAdapter.java:793)\r\n	at org.springframework.web.servlet.mvc.method.AbstractHandlerMethodAdapter.handle(AbstractHandlerMethodAdapter.java:87)\r\n	at org.springframework.web.servlet.DispatcherServlet.doDispatch(DispatcherServlet.java:1040)\r\n	at org.springframework.web.servlet.DispatcherServlet.doService(DispatcherServlet.java:943)\r\n	at org.springframework.web.servlet.FrameworkServlet.processRequest(FrameworkServlet.java:1006)\r\n	at org.springframework.web.servlet.FrameworkServlet.doGet(FrameworkServlet.java:898)\r\n	at javax.servlet.http.HttpServlet.service(HttpServlet.java:634)\r\n	at org.springframework.web.servlet.FrameworkServlet.service(FrameworkServlet.java:883)\r\n	at javax.servlet.http.HttpServlet.service(HttpServlet.java:741)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:231)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.apache.tomcat.websocket.server.WsFilter.doFilter(WsFilter.java:53)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:320)\r\n	at org.springframework.security.web.access.intercept.FilterSecurityInterceptor.invoke(FilterSecurityInterceptor.java:126)\r\n	at org.springframework.security.web.access.intercept.FilterSecurityInterceptor.doFilter(FilterSecurityInterceptor.java:90)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.access.ExceptionTranslationFilter.doFilter(ExceptionTranslationFilter.java:118)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.session.SessionManagementFilter.doFilter(SessionManagementFilter.java:137)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.authentication.AnonymousAuthenticationFilter.doFilter(AnonymousAuthenticationFilter.java:111)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.servletapi.SecurityContextHolderAwareRequestFilter.doFilter(SecurityContextHolderAwareRequestFilter.java:158)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.savedrequest.RequestCacheAwareFilter.doFilter(RequestCacheAwareFilter.java:63)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at top.naccl.config.JwtFilter.doFilter(JwtFilter.java:35)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.authentication.AbstractAuthenticationProcessingFilter.doFilter(AbstractAuthenticationProcessingFilter.java:200)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.authentication.logout.LogoutFilter.doFilter(LogoutFilter.java:116)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.web.filter.CorsFilter.doFilterInternal(CorsFilter.java:92)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.header.HeaderWriterFilter.doHeadersAfter(HeaderWriterFilter.java:92)\r\n	at org.springframework.security.web.header.HeaderWriterFilter.doFilterInternal(HeaderWriterFilter.java:77)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.context.SecurityContextPersistenceFilter.doFilter(SecurityContextPersistenceFilter.java:105)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.context.request.async.WebAsyncManagerIntegrationFilter.doFilterInternal(WebAsyncManagerIntegrationFilter.java:56)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:334)\r\n	at org.springframework.security.web.FilterChainProxy.doFilterInternal(FilterChainProxy.java:215)\r\n	at org.springframework.security.web.FilterChainProxy.doFilter(FilterChainProxy.java:178)\r\n	at org.springframework.web.filter.DelegatingFilterProxy.invokeDelegate(DelegatingFilterProxy.java:358)\r\n	at org.springframework.web.filter.DelegatingFilterProxy.doFilter(DelegatingFilterProxy.java:271)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.web.filter.RequestContextFilter.doFilterInternal(RequestContextFilter.java:100)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.web.filter.FormContentFilter.doFilterInternal(FormContentFilter.java:93)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.springframework.web.filter.CharacterEncodingFilter.doFilterInternal(CharacterEncodingFilter.java:201)\r\n	at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\r\n	at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:193)\r\n	at org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:166)\r\n	at org.apache.catalina.core.StandardWrapperValve.invoke(StandardWrapperValve.java:202)\r\n	at org.apache.catalina.core.StandardContextValve.invoke(StandardContextValve.java:96)\r\n	at org.apache.catalina.authenticator.AuthenticatorBase.invoke(AuthenticatorBase.java:541)\r\n	at org.apache.catalina.core.StandardHostValve.invoke(StandardHostValve.java:139)\r\n	at org.apache.catalina.valves.ErrorReportValve.invoke(ErrorReportValve.java:92)\r\n	at org.apache.catalina.core.StandardEngineValve.invoke(StandardEngineValve.java:74)\r\n	at org.apache.catalina.connector.CoyoteAdapter.service(CoyoteAdapter.java:343)\r\n	at org.apache.coyote.http11.Http11Processor.service(Http11Processor.java:373)\r\n	at org.apache.coyote.AbstractProcessorLight.process(AbstractProcessorLight.java:65)\r\n	at org.apache.coyote.AbstractProtocol$ConnectionHandler.process(AbstractProtocol.java:868)\r\n	at org.apache.tomcat.util.net.NioEndpoint$SocketProcessor.doRun(NioEndpoint.java:1590)\r\n	at org.apache.tomcat.util.net.SocketProcessorBase.run(SocketProcessorBase.java:49)\r\n	at java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1149)\r\n	at java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:624)\r\n	at org.apache.tomcat.util.threads.TaskThread$WrappingRunnable.run(TaskThread.java:61)\r\n	at java.lang.Thread.run(Thread.java:750)\r\n', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', '2023-09-16 01:31:58', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');

-- ----------------------------
-- Table structure for friend
-- ----------------------------
DROP TABLE IF EXISTS `friend`;
CREATE TABLE `friend`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `nickname` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '昵称',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '描述',
  `website` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '站点',
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '头像',
  `is_published` tinyint(1) NOT NULL COMMENT '公开或隐藏',
  `views` int NOT NULL COMMENT '点击次数',
  `create_time` datetime NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of friend
-- ----------------------------

-- ----------------------------
-- Table structure for login_log
-- ----------------------------
DROP TABLE IF EXISTS `login_log`;
CREATE TABLE `login_log`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '用户名称',
  `ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip',
  `ip_source` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip来源',
  `os` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作系统',
  `browser` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '浏览器',
  `status` bit(1) NULL DEFAULT NULL COMMENT '登录状态',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作描述',
  `create_time` datetime NOT NULL COMMENT '登录时间',
  `user_agent` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'user-agent用户代理',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 19 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of login_log
-- ----------------------------
INSERT INTO `login_log` VALUES (1, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-20 23:58:57', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (2, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:18', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (3, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:31', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (4, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:31', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (5, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:32', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (6, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:32', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (7, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:33', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (8, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:33', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (9, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:33', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (10, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:34', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (11, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:35', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (12, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:36', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (13, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:00:51', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (14, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:01:09', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (15, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:03:17', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (16, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'0', '用户名或密码错误', '2023-08-21 00:04:44', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (17, 'Admin', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', b'1', '登录成功', '2023-08-21 00:10:05', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `login_log` VALUES (18, 'admin', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', b'1', '登录成功', '2023-09-16 02:07:59', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `login_log` VALUES (19, 'admin', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', b'1', '登录成功', '2023-09-16 15:53:43', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');

-- ----------------------------
-- Table structure for moment
-- ----------------------------
DROP TABLE IF EXISTS `moment`;
CREATE TABLE `moment`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '动态内容',
  `create_time` datetime NOT NULL COMMENT '创建时间',
  `likes` int NULL DEFAULT NULL COMMENT '点赞数量',
  `is_published` bit(1) NOT NULL COMMENT '是否公开',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of moment
-- ----------------------------
INSERT INTO `moment` VALUES (1, '测试动态', '2024-03-03 00:12:18', 1, b'1');

-- ----------------------------
-- Table structure for operation_log
-- ----------------------------
DROP TABLE IF EXISTS `operation_log`;
CREATE TABLE `operation_log`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '操作者用户名',
  `uri` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求接口',
  `method` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求方式',
  `param` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '请求参数',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作描述',
  `ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip',
  `ip_source` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip来源',
  `os` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作系统',
  `browser` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '浏览器',
  `times` int NOT NULL COMMENT '请求耗时（毫秒）',
  `create_time` datetime NOT NULL COMMENT '操作时间',
  `user_agent` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'user-agent用户代理',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 10 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of operation_log
-- ----------------------------
INSERT INTO `operation_log` VALUES (1, 'Admin', '/admin/blog', 'POST', '{\"blog\":{\"id\":1,\"title\":\"123\",\"firstPicture\":\"123\",\"content\":\"测试\\n测试\\n\\n测试q\\n\\n## 的\\n\\n\",\"description\":\"::: hljs-center\\n\\n居中\\n\\n:::\\n123123\\n\\n测试\\n\\n测试\\n\\n测试\",\"published\":true,\"recommend\":false,\"appreciation\":false,\"commentEnabled\":false,\"top\":false,\"createTime\":1692547866601,\"updateTime\":1692547866601,\"views\":1231,\"words\":123,\"readTime\":1,\"password\":\"\",\"user\":{\"id\":1,\"username\":null,\"password\":null,\"nickname\":null,\"avatar\":null,\"email\":null,\"createTime\":null,\"updateTime\":null,\"role\":null},\"category\":{\"id\":1,\"name\":\"的\",\"blogs\":[]},\"tags\":[],\"cate\":\"的\",\"tagList\":[\"的\"]}}', '发布博客', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 14, '2023-08-21 00:11:07', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `operation_log` VALUES (2, 'Admin', '/admin/blog/1/visibility', 'PUT', '{\"id\":1,\"blogVisibility\":{\"appreciation\":false,\"recommend\":false,\"commentEnabled\":false,\"top\":false,\"published\":true,\"password\":\"123\"}}', '更新博客可见性状态', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 3, '2023-08-21 00:11:11', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `operation_log` VALUES (3, 'Admin', '/admin/blog/1/visibility', 'PUT', '{\"id\":1,\"blogVisibility\":{\"appreciation\":false,\"recommend\":false,\"commentEnabled\":false,\"top\":false,\"published\":true,\"password\":\"123\"}}', '更新博客可见性状态', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 02:08:13', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (4, 'Admin', '/admin/blog', 'DELETE', '{\"id\":1}', '删除博客', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 8, '2023-09-16 02:08:47', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (5, 'Admin', '/admin/blog', 'POST', '{\"blog\":{\"id\":2,\"title\":\"测试\",\"firstPicture\":\"cs\",\"content\":\"## 测试\",\"description\":\"测试\",\"published\":true,\"recommend\":false,\"appreciation\":false,\"commentEnabled\":false,\"top\":false,\"createTime\":1694801364396,\"updateTime\":1694801364396,\"views\":0,\"words\":10000,\"readTime\":50,\"password\":\"123\",\"user\":{\"id\":1,\"username\":null,\"password\":null,\"nickname\":null,\"avatar\":null,\"email\":null,\"createTime\":null,\"updateTime\":null,\"role\":null},\"category\":{\"id\":1,\"name\":\"的\",\"blogs\":[]},\"tags\":[],\"cate\":1,\"tagList\":[1]}}', '发布博客', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 7, '2023-09-16 02:09:24', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (6, 'Admin', '/admin/blog/2/visibility', 'PUT', '{\"id\":2,\"blogVisibility\":{\"appreciation\":false,\"recommend\":false,\"commentEnabled\":false,\"top\":false,\"published\":false,\"password\":\"\"}}', '更新博客可见性状态', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 3, '2023-09-16 02:12:38', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (7, 'Admin', '/admin/blog/2/visibility', 'PUT', '{\"id\":2,\"blogVisibility\":{\"appreciation\":true,\"recommend\":true,\"commentEnabled\":true,\"top\":true,\"published\":true,\"password\":\"\"}}', '更新博客可见性状态', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 3, '2023-09-16 02:12:56', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (8, 'Admin', '/admin/blog/recommend', 'PUT', '{\"id\":2,\"recommend\":false}', '更新博客推荐状态', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 15:53:48', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (9, 'Admin', '/admin/blog/top', 'PUT', '{\"id\":2,\"top\":false}', '更新博客置顶状态', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 15:53:48', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `operation_log` VALUES (10, 'Admin', '/admin/blog/2/visibility', 'PUT', '{\"id\":2,\"blogVisibility\":{\"appreciation\":false,\"recommend\":false,\"commentEnabled\":false,\"top\":false,\"published\":true,\"password\":\"123\"}}', '更新博客可见性状态', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 32, '2023-09-16 15:53:52', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');

-- ----------------------------
-- Table structure for schedule_job
-- ----------------------------
DROP TABLE IF EXISTS `schedule_job`;
CREATE TABLE `schedule_job`  (
  `job_id` bigint NOT NULL AUTO_INCREMENT COMMENT '任务id',
  `bean_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'spring bean名称',
  `method_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '方法名',
  `params` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '参数',
  `cron` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'cron表达式',
  `status` tinyint NULL DEFAULT NULL COMMENT '任务状态',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '备注',
  `create_time` datetime NULL DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`job_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of schedule_job
-- ----------------------------
INSERT INTO `schedule_job` VALUES (1, 'redisSyncScheduleTask', 'syncBlogViewsToDatabase', '', '0 0 1 * * ?', 1, '每天凌晨一点，从Redis将博客浏览量同步到数据库', '2020-11-17 23:45:42');
INSERT INTO `schedule_job` VALUES (2, 'visitorSyncScheduleTask', 'syncVisitInfoToDatabase', '', '0 0 0 * * ?', 1, '清空当天Redis访客标识，记录当天的PV和UV，更新当天所有访客的PV和最后访问时间，更新城市新增访客UV数', '2021-02-05 08:14:28');

-- ----------------------------
-- Table structure for schedule_job_log
-- ----------------------------
DROP TABLE IF EXISTS `schedule_job_log`;
CREATE TABLE `schedule_job_log`  (
  `log_id` bigint NOT NULL AUTO_INCREMENT COMMENT '任务日志id',
  `job_id` bigint NOT NULL COMMENT '任务id',
  `bean_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'spring bean名称',
  `method_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '方法名',
  `params` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '参数',
  `status` tinyint NOT NULL COMMENT '任务执行结果',
  `error` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL COMMENT '异常信息',
  `times` int NOT NULL COMMENT '耗时（单位：毫秒）',
  `create_time` datetime NULL DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`log_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of schedule_job_log
-- ----------------------------
INSERT INTO `schedule_job_log` VALUES (1, 2, 'visitorSyncScheduleTask', 'syncVisitInfoToDatabase', '', 1, NULL, 16, '2023-08-21 00:00:00');

-- ----------------------------
-- Table structure for site_setting
-- ----------------------------
DROP TABLE IF EXISTS `site_setting`;
CREATE TABLE `site_setting`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `name_en` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL,
  `name_zh` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL,
  `value` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL,
  `type` int NULL DEFAULT NULL COMMENT '1基础设置，2页脚徽标，3资料卡，4友链信息',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 33 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of site_setting
-- ----------------------------
INSERT INTO `site_setting` VALUES (1, 'blogName', '博客名称', 'Zero Blog', 1);
INSERT INTO `site_setting` VALUES (2, 'webTitleSuffix', '网页标题后缀', ' - Zero Blog', 1);
INSERT INTO `site_setting` VALUES (3, 'footerImgTitle', '页脚图片标题', '手机看本站', 1);
INSERT INTO `site_setting` VALUES (4, 'footerImgUrl', '页脚图片路径', '/img/qr.png', 1);
INSERT INTO `site_setting` VALUES (5, 'copyright', 'Copyright', '{\"title\":\"Copyright © 2019 - 2022\",\"siteName\":\"Zero Blog\"}', 1);
INSERT INTO `site_setting` VALUES (6, 'beian', 'ICP备案号', '', 1);
INSERT INTO `site_setting` VALUES (7, 'reward', '赞赏码', '/img/reward.jpg', 1);
INSERT INTO `site_setting` VALUES (8, 'commentAdminFlag', '博主评论标识', '咕咕', 1);
INSERT INTO `site_setting` VALUES (9, 'playlistServer', '播放器平台', 'netease', 1);
INSERT INTO `site_setting` VALUES (10, 'playlistId', '播放器歌单', '3071528549', 1);
INSERT INTO `site_setting` VALUES (11, 'avatar', '头像', '/img/avatar.jpg', 2);
INSERT INTO `site_setting` VALUES (12, 'name', '昵称', '路人', 2);
INSERT INTO `site_setting` VALUES (13, 'rollText', '滚动个签', '\"云鹤当归天，天不迎我妙木仙；\",\"游龙当归海，海不迎我自来也。\"', 2);
INSERT INTO `site_setting` VALUES (14, 'github', 'GitHub', 'https://github.com/Naccl', 2);
INSERT INTO `site_setting` VALUES (15, 'telegram', 'Telegram', 'https://t.me/NacclOfficial', 2);
INSERT INTO `site_setting` VALUES (16, 'qq', 'QQ', 'http://sighttp.qq.com/authd?IDKEY=', 2);
INSERT INTO `site_setting` VALUES (17, 'bilibili', 'bilibili', 'https://space.bilibili.com/', 2);
INSERT INTO `site_setting` VALUES (18, 'netease', '网易云音乐', 'https://music.163.com/#/user/home?id=', 2);
INSERT INTO `site_setting` VALUES (19, 'email', 'email', 'mailto:you@example.com', 2);
INSERT INTO `site_setting` VALUES (20, 'favorite', '自定义', '{\"title\":\"最喜欢的动漫 📺\",\"content\":\"异度侵入、春物语、NO GAME NO LIFE、实力至上主义的教室、辉夜大小姐、青春猪头少年不会梦到兔女郎学姐、路人女主、Re0、魔禁、超炮、俺妹、在下坂本、散华礼弥、OVERLORD、慎勇、人渣的本愿、白色相簿2、死亡笔记、DARLING in the FRANXX、鬼灭之刃\"}', 2);
INSERT INTO `site_setting` VALUES (21, 'favorite', '自定义', '{\"title\":\"最喜欢我的女孩子们 🤤\",\"content\":\"芙兰达、土间埋、食蜂操祈、佐天泪爷、樱岛麻衣、桐崎千棘、02、亚丝娜、高坂桐乃、五更琉璃、安乐冈花火、一色彩羽、英梨梨、珈百璃、时崎狂三、可儿那由多、和泉纱雾、早坂爱\"}', 2);
INSERT INTO `site_setting` VALUES (22, 'favorite', '自定义', '{\"title\":\"最喜欢玩的游戏 🎮\",\"content\":\"Stellaris、巫师、GTA、荒野大镖客、刺客信条、魔兽争霸、LOL、PUBG\"}', 2);
INSERT INTO `site_setting` VALUES (23, 'badge', '徽标', '{\"title\":\"本博客已开源于 GitHub\",\"url\":\"https://github.com/lurendie/zero_blog\",\"subject\":\"Zero Blog\",\"value\":\"Open Source\",\"color\":\"brightgreen\"}', 3);
INSERT INTO `site_setting` VALUES (24, 'badge', '徽标', '{\"title\":\"由 Actix强力驱动\",\"url\":\"https://actix.rs/\",\"subject\":\"Powered\",\"value\":\"Actix\",\"color\":\"blue\"}', 3);
INSERT INTO `site_setting` VALUES (25, 'badge', '徽标', '{\"title\":\"Vue.js 客户端渲染\",\"url\":\"https://cn.vuejs.org/\",\"subject\":\"SPA\",\"value\":\"Vue.js\",\"color\":\"brightgreen\"}', 3);
INSERT INTO `site_setting` VALUES (26, 'badge', '徽标', '{\"title\":\"UI 框架 Semantic-UI\",\"url\":\"https://semantic-ui.com/\",\"subject\":\"UI\",\"value\":\"Semantic-UI\",\"color\":\"semantic-ui\"}', 3);
INSERT INTO `site_setting` VALUES (27, 'badge', '徽标', '{\"title\":\"阿里云提供服务器及域名相关服务\",\"url\":\"https://www.aliyun.com/\",\"subject\":\"VPS & DNS\",\"value\":\"Aliyun\",\"color\":\"blueviolet\"}', 3);
INSERT INTO `site_setting` VALUES (28, 'badge', '徽标', '{\"title\":\"静态资源托管于 GitHub\",\"url\":\"https://github.com/\",\"subject\":\"OSS\",\"value\":\"GitHub\",\"color\":\"github\"}', 3);
INSERT INTO `site_setting` VALUES (29, 'badge', '徽标', '{\"title\":\"jsDelivr 加速静态资源\",\"url\":\"https://www.jsdelivr.com/\",\"subject\":\"CDN\",\"value\":\"jsDelivr\",\"color\":\"orange\"}', 3);
INSERT INTO `site_setting` VALUES (30, 'badge', '徽标', '{\"color\":\"lightgray\",\"subject\":\"CC\",\"title\":\"本站点采用 CC BY 4.0 国际许可协议进行许可\",\"url\":\"https://creativecommons.org/licenses/by/4.0/\",\"value\":\"BY 4.0\"}', 3);
INSERT INTO `site_setting` VALUES (31, 'friendContent', '友链页面信息', '随机排序，不分先后。欢迎交换友链~(￣▽￣)~*\n\n* 昵称：路人\n* 一句话：游龙当归海，海不迎我自来也。\n* 网址：[https://naccl.top](https://naccl.top)\n* 头像URL：[https://naccl.top/img/avatar.jpg](https://naccl.top/img/avatar.jpg)\n\n仅凭个人喜好添加友链，请在收到我的回复邮件后再于贵站添加本站链接。原则上已添加的友链不会删除，如果你发现自己被移除了，恕不另行通知，只需和我一样做就好。\n\n', 4);
INSERT INTO `site_setting` VALUES (32, 'friendCommentEnabled', '友链页面评论开关', '0', 4);

-- ----------------------------
-- Table structure for tag
-- ----------------------------
DROP TABLE IF EXISTS `tag`;
CREATE TABLE `tag`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `tag_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `color` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '标签颜色(可选)',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 5 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of tag
-- ----------------------------
INSERT INTO `tag` VALUES (1, 'Vue', '#7FFF00');
INSERT INTO `tag` VALUES (2, 'RUST', '#7FFF00');
INSERT INTO `tag` VALUES (3, 'Java', '#7FFF00');
INSERT INTO `tag` VALUES (4, '二次元', '#7FFF00');

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '用户名',
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '密码',
  `nickname` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '昵称',
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '头像地址',
  `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '邮箱',
  `create_time` datetime NOT NULL COMMENT '创建时间',
  `update_time` datetime NOT NULL COMMENT '更新时间',
  `role` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '角色访问权限',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of user
-- ----------------------------
INSERT INTO `user` VALUES (1, 'admin', '123456', 'admin', '/img/avatar.jpg', 'admin@naccl.top', '2020-09-21 16:47:18', '2020-09-21 16:47:22', 'ROLE_admin');

-- ----------------------------
-- Table structure for visit_log
-- ----------------------------
DROP TABLE IF EXISTS `visit_log`;
CREATE TABLE `visit_log`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '访客标识码',
  `uri` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求接口',
  `method` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求方式',
  `param` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '请求参数',
  `behavior` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '访问行为',
  `content` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '访问内容',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '备注',
  `ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip',
  `ip_source` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip来源',
  `os` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作系统',
  `browser` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '浏览器',
  `times` int NOT NULL COMMENT '请求耗时（毫秒）',
  `create_time` datetime NOT NULL COMMENT '访问时间',
  `user_agent` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'user-agent用户代理',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 50 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of visit_log
-- ----------------------------
INSERT INTO `visit_log` VALUES (1, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 7, '2023-08-20 23:56:54', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visit_log` VALUES (2, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 40, '2023-08-20 23:57:39', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visit_log` VALUES (3, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 1, '2023-08-20 23:57:43', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visit_log` VALUES (4, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 18, '2023-08-21 00:11:15', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visit_log` VALUES (5, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 5, '2023-08-21 00:11:23', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visit_log` VALUES (6, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', 2, '2023-08-21 00:11:28', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visit_log` VALUES (7, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 52, '2023-09-08 00:12:55', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (8, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-08 00:13:03', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (9, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 72, '2023-09-08 00:13:05', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (10, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 6, '2023-09-08 00:13:26', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (11, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-08 00:13:40', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (12, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 41, '2023-09-08 00:18:15', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (13, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 60, '2023-09-08 00:18:16', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.69');
INSERT INTO `visit_log` VALUES (14, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 18, '2023-09-16 01:32:02', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (15, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 65, '2023-09-16 01:32:04', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (16, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-16 01:32:10', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (17, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 2, '2023-09-16 01:32:16', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (18, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 2, '2023-09-16 01:32:23', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (19, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/tag', 'GET', '{\"tagName\":\"的\",\"pageNum\":1}', '查看标签', '的', '标签名称：的，第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 31, '2023-09-16 01:33:14', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (20, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 6, '2023-09-16 01:33:16', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (21, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/tag', 'GET', '{\"tagName\":\"的\",\"pageNum\":1}', '查看标签', '的', '标签名称：的，第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 9, '2023-09-16 01:33:29', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (22, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-16 01:33:35', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (23, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/about', 'GET', '{}', '访问页面', '关于我', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 12, '2023-09-16 01:39:40', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (24, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/friends', 'GET', '{}', '访问页面', '友链', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 14, '2023-09-16 01:39:42', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (25, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/moments', 'GET', '{\"pageNum\":1}', '访问页面', '动态', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 8, '2023-09-16 01:39:44', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (26, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/archives', 'GET', '{}', '访问页面', '归档', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-16 01:39:44', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (27, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/category', 'GET', '{\"categoryName\":\"的\",\"pageNum\":1}', '查看分类', '的', '分类名称：的，第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 6, '2023-09-16 01:39:47', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (28, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 2, '2023-09-16 01:39:48', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (29, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 6, '2023-09-16 01:39:52', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (30, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 3, '2023-09-16 01:40:35', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (31, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 01:40:37', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (32, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 2, '2023-09-16 01:40:41', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (33, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-16 01:40:45', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (34, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":1}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-16 02:06:52', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (35, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 8, '2023-09-16 02:10:50', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (36, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":2}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 3, '2023-09-16 02:10:52', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (37, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 2, '2023-09-16 02:11:06', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (38, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 02:11:31', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (39, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":2}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 02:11:32', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (40, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":2}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 02:11:36', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (41, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 2, '2023-09-16 02:11:51', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (42, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":2}', '查看博客', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 02:11:54', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (43, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 3, '2023-09-16 02:12:41', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (44, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 5, '2023-09-16 02:12:59', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (45, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '/blog', 'GET', '{\"id\":2}', '查看博客', '测试', '文章标题：测试', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 02:13:01', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (46, '0081b143-21ad-3a2d-af65-16ec2e8c340b', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 40, '2023-09-16 15:53:21', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (47, '0081b143-21ad-3a2d-af65-16ec2e8c340b', '/blog', 'GET', '{\"id\":2}', '查看博客', '测试', '文章标题：测试', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 40, '2023-09-16 15:53:25', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (48, '0081b143-21ad-3a2d-af65-16ec2e8c340b', '/blogs', 'GET', '{\"pageNum\":1}', '访问页面', '首页', '第1页', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 10, '2023-09-16 15:53:56', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (49, '0081b143-21ad-3a2d-af65-16ec2e8c340b', '/checkBlogPassword', 'POST', '{\"blogPassword\":{\"blogId\":2,\"password\":\"123\"}}', '校验博客密码', '', '', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 1, '2023-09-16 15:54:02', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');
INSERT INTO `visit_log` VALUES (50, '0081b143-21ad-3a2d-af65-16ec2e8c340b', '/blog', 'GET', '{\"id\":2}', '查看博客', '测试', '文章标题：测试', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', 4, '2023-09-16 15:54:02', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');

-- ----------------------------
-- Table structure for visit_record
-- ----------------------------
DROP TABLE IF EXISTS `visit_record`;
CREATE TABLE `visit_record`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `pv` int NOT NULL COMMENT '访问量',
  `uv` int NOT NULL COMMENT '独立用户',
  `date` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '日期\"02-23\"',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of visit_record
-- ----------------------------
INSERT INTO `visit_record` VALUES (1, 3, 1, '08-20');

-- ----------------------------
-- Table structure for visitor
-- ----------------------------
DROP TABLE IF EXISTS `visitor`;
CREATE TABLE `visitor`  (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '访客标识码',
  `ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip',
  `ip_source` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'ip来源',
  `os` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '操作系统',
  `browser` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '浏览器',
  `create_time` datetime NOT NULL COMMENT '首次访问时间',
  `last_time` datetime NOT NULL COMMENT '最后访问时间',
  `pv` int NULL DEFAULT NULL COMMENT '访问页数统计',
  `user_agent` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'user-agent用户代理',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `idx_uuid`(`uuid`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of visitor
-- ----------------------------
INSERT INTO `visitor` VALUES (1, 'cb959f51-e382-3fc8-ac9a-ff3aa7482ff7', '192.168.161.40', '内网IP|内网IP', 'Windows >=10', 'Edge 115', '2023-08-20 23:56:54', '2023-08-20 23:57:43', 3, 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.203');
INSERT INTO `visitor` VALUES (2, '0081b143-21ad-3a2d-af65-16ec2e8c340b', '192.168.10.109', '内网IP|内网IP', 'Windows >=10', 'Edge 116', '2023-09-16 15:53:21', '2023-09-16 15:53:21', 0, 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 Edg/116.0.1938.81');

SET FOREIGN_KEY_CHECKS = 1;
