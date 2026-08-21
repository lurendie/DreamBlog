import {
	SAVE_COMMENT_RESULT,
	SET_PARENT_COMMENT_ID,
	RESET_COMMENT_FORM,
	SET_BLOG_PASSWORD_DIALOG_VISIBLE,
	SET_BLOG_PASSWORD_FORM
} from "./mutations-types";

import {getCommentListByQuery, submitComment} from "@/api/comment";
import { ElMessage } from 'element-plus/es/components/message/index.mjs'
import { ElNotification } from 'element-plus/es/components/notification/index.mjs'
import router from "../router";
import tvMapper from '@/plugins/tvMapper.json'
import aruMapper from '@/plugins/aruMapper.json'
import paopaoMapper from '@/plugins/paopaoMapper.json'
import { escapeHtml } from '@/util/sanitizeHtml'
import {getBlogToken, isBlogVerified} from '@/util/storage'

//评论列表请求序号：路由快速切换时丢弃过期响应，避免旧页面评论覆盖新页面
let commentRequestSeq = 0

export default {
	getCommentList({commit, rootState}) {
		const seq = ++commentRequestSeq
		//密码保护的文章，需要发送密码验证通过后保存在localStorage的Token
		const blogToken = getBlogToken(rootState.commentQuery.blogId)
		//博主身份由 httpOnly Cookie 自动携带，无需再发 JWT 头

		function replaceEmoji(comment, emoji) {
			comment.content = comment.content.replace(new RegExp(emoji.reg, 'g'), `<img src="${emoji.src}">`)
		}

		function convertEmoji(comment) {
			tvMapper.forEach(emoji => {
				replaceEmoji(comment, emoji)
			})
			aruMapper.forEach(emoji => {
				replaceEmoji(comment, emoji)
			})
			paopaoMapper.forEach(emoji => {
				replaceEmoji(comment, emoji)
			})
		}

		getCommentListByQuery(blogToken, rootState.commentQuery).then(res => {
			//过期响应（已切换到其它页面）直接丢弃
			if (seq !== commentRequestSeq) {
				return
			}
			if (res.code === 200) {
				res.data.comments.list.forEach(comment => {
					//转义评论中的html
					comment.content = escapeHtml(comment.content)
					//查找评论中是否有表情
					if (comment.content.indexOf('@[') != -1) {
						convertEmoji(comment)
					}
					comment.replyComments.forEach(comment => {
						//转义评论中的html
						comment.content = escapeHtml(comment.content)
						//查找评论中是否有表情
						if (comment.content.indexOf('@[') != -1) {
							convertEmoji(comment)
						}
					})
				})
				commit(SAVE_COMMENT_RESULT, res.data)
			}
		}).catch(() => {
			if (seq === commentRequestSeq) {
				ElMessage.error("请求失败")
			}
		})
	},
	submitCommentForm({rootState, dispatch, commit}, token) {
		let form = {...rootState.commentForm}
		form.page = rootState.commentQuery.page
		form.blogId = rootState.commentQuery.blogId
		form.parentCommentId = rootState.parentCommentId
		submitComment(token, form).then(res => {
			if (res.code === 200) {
				ElNotification({
					title: res.msg,
					type: 'success'
				})
				commit(SET_PARENT_COMMENT_ID, -1)
				commit(RESET_COMMENT_FORM)
				dispatch('getCommentList')
			} else {
				ElNotification({
					title: '评论失败',
					message: res.msg,
					type: 'error'
				})
			}
		}).catch(() => {
			ElNotification({
				title: '评论失败',
				message: '异常错误',
				type: 'error'
			})
		})
	},
	goBlogPage({commit}, blog) {
		if (blog.privacy) {
			//isAdmin 仅登录态标记（httpOnly Cookie 中的会话由后端校验）
			const isAdmin = window.localStorage.getItem('isAdmin')
			const blogVerified = isBlogVerified(blog.id)
			//密码保护文章：博主登录态或已验证标记都可以进入，后端会校验会话有效性
			if (isAdmin || blogVerified) {
				return router.push(`/blog/${blog.id}`)
			}
			commit(SET_BLOG_PASSWORD_FORM, {blogId: blog.id, password: ''})
			commit(SET_BLOG_PASSWORD_DIALOG_VISIBLE, true)
		} else {
			router.push(`/blog/${blog.id}`)
		}
	},
}
