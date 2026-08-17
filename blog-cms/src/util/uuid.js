/**
 * 生成 UUID（v4）。
 * 优先使用 crypto.randomUUID（更安全，避免可预测的 Math.random），
 * 环境不支持时回退到基于 crypto.getRandomValues 的实现，
 * 最后才回退到 Math.random 的实现（现代浏览器基本不会走到）。
 */
function fallbackRandomUUID() {
	//crypto.getRandomValues 方案，确保回退实现也是密码学安全的
	if (typeof crypto !== 'undefined' && typeof crypto.getRandomValues === 'function') {
		const bytes = new Uint8Array(16)
		crypto.getRandomValues(bytes)
		bytes[6] = (bytes[6] & 0x0f) | 0x40 // version 4
		bytes[8] = (bytes[8] & 0x3f) | 0x80 // variant 10
		const hex = Array.from(bytes, b => b.toString(16).padStart(2, '0')).join('')
		return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`
	}
	//最终兜底（极少数不支持 getRandomValues 的环境）
	return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
		let r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8)
		return v.toString(16)
	})
}

export function randomUUID() {
	if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
		return crypto.randomUUID()
	}
	return fallbackRandomUUID()
}