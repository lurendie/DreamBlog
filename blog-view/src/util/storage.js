/**
 * localStorage 相关的安全工具函数
 */

/**
 * 安全的 JSON.parse：解析失败或输入为空时返回 fallback
 * localStorage 中可能被篡改/损坏/写入超大内容，直接 JSON.parse 会抛异常导致页面报错，
 * 所有 JSON.parse(localStorage.getItem(...)) 都应改为使用本函数
 */
export function safeParse(value, fallback = null) {
	if (value === null || value === undefined) {
		return fallback
	}
	try {
		return JSON.parse(value)
	} catch (_) {
		return fallback
	}
}

export function safeParseArray(value, fallback = []) {
	const parsed = safeParse(value, fallback)
	return Array.isArray(parsed) ? parsed : fallback
}

//私密文章密码验证通过的"布尔标记"（与真实 token 共用一个键 blog{id}，用于区分两种情况）
export const BLOG_PASSWORD_VERIFIED_MARKER = 'verified'

/**
 * 保存私密文章密码验证结果
 * - token 为字符串时才把 token 本身写入 localStorage
 * - 否则（例如后端返回整个文章对象）只写入"已验证"布尔标记，防止把大对象塞进 localStorage
 */
export function setBlogVerified(blogId, token) {
	const key = `blog${blogId}`
	if (typeof token === 'string' && token) {
		window.localStorage.setItem(key, token)
	} else {
		window.localStorage.setItem(key, BLOG_PASSWORD_VERIFIED_MARKER)
	}
}

/**
 * 读取私密文章 token（用于 Authorization 请求头）
 * 仅当存储值是真实 token 字符串（而非"已验证"布尔标记）时才返回，否则返回空字符串
 */
export function getBlogToken(blogId) {
	const value = window.localStorage.getItem(`blog${blogId}`)
	return value && value !== BLOG_PASSWORD_VERIFIED_MARKER ? value : ''
}

/** 私密文章是否已通过密码验证（无论是否持有 token 字符串） */
export function isBlogVerified(blogId) {
	return !!window.localStorage.getItem(`blog${blogId}`)
}
