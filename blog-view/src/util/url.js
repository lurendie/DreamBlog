/**
 * 校验并规范化"外部链接"URL
 *
 * 仅允许 http:/https: 协议，阻止 javascript:、data:、vbscript: 等危险 scheme 注入。
 * 实现方式：先用 new URL(value) 解析，再检查 scheme 白名单；解析失败或 scheme 不在白名单内一律返回 null。
 * 调用方应将返回值直接用于 :href 等属性绑定（null 时 Vue 不会渲染该属性），
 * 例如评论中的 website 字段：:href="safeExternalUrl(comment.website)"
 *
 * @param {*} value 待校验的 URL 字符串
 * @returns {string|null} 校验通过返回规范化后的 URL（绝对地址），否则返回 null
 */
export function safeExternalUrl(value) {
	if (typeof value !== 'string' || !value.trim()) {
		return null
	}
	try {
		const url = new URL(value.trim())
		return (url.protocol === 'http:' || url.protocol === 'https:') ? url.href : null
	} catch (_) {
		return null
	}
}