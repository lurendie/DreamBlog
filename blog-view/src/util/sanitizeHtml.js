const allowedTags = new Set([
	'a', 'abbr', 'article', 'aside', 'b', 'blockquote', 'br', 'caption', 'code', 'del', 'details', 'div',
	'em', 'figcaption', 'figure', 'footer', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'header', 'hr', 'i',
	'img', 'ins', 'kbd', 'li', 'main', 'mark', 'menu', 'meting-js', 'nav', 'ol', 'p', 'pre', 'q', 's', 'section',
	'small', 'span', 'strong', 'sub', 'sup', 'table', 'tbody', 'td', 'tfoot', 'th', 'thead', 'tr',
	'u', 'ul'
])

const globalAllowedAttributes = new Set(['class', 'id', 'title'])
const allowedAttributesByTag = {
	a: new Set(['href', 'name', 'target', 'rel']),
	img: new Set(['src', 'alt', 'title', 'width', 'height', 'loading']),
	code: new Set(['class']),
	pre: new Set(['class']),
	'meting-js': new Set(['server', 'type', 'id', 'theme', 'autoplay', 'volume', 'mutex', 'listmaxheight', 'preload', 'loop', 'mini', 'fixed', 'order', 'storage'])
}

const allowedSchemes = new Set(['http:', 'https:', 'mailto:', 'tel:'])
const allowedImageSchemes = new Set(['http:', 'https:', 'data:'])

//一旦出现即整体移除（不做属性清洗、不解包其内容）
//svg/math 中的内容（如 <svg><a href="javascript:">）即便解包也会被浏览器视为危险上下文，一律移除
const removedTags = new Set(['script', 'style', 'iframe', 'object', 'embed', 'svg', 'math'])

function isSafeUrl(tagName, attrName, value) {
	if (!value) {
		return false
	}
	if (attrName !== 'href' && attrName !== 'src') {
		return true
	}
	try {
		const url = new URL(value, window.location.origin)
		if (tagName === 'img') {
			//img 额外允许 data:（内联图片），但不允许 javascript:/vbscript:
			return allowedImageSchemes.has(url.protocol)
		}
		return allowedSchemes.has(url.protocol)
	} catch (_) {
		return false
	}
}

/**
 * 深度清洗单个节点：
 * 1. 对于元素节点，先递归清洗其子节点，再处理自身的属性与去留
 * 2. 危险标签（script/style/iframe/object/embed/svg/math）直接移除，不解包
 * 3. 白名单外标签：先把其 childNodes 全部递归清洗，再用清洗后的子节点替换自身（解包）
 * 4. 文本节点原样保留（其内容由后续 escapeHtml/textContent 处理，这里不改动）
 */
function sanitizeNode(node) {
	//文本节点或注释节点无需递归，直接保留
	if (node.nodeType === 3 || node.nodeType === 8) {
		return
	}
	if (node.nodeType !== 1) {
		//其它非元素/文本节点（如文档片段在递归时不会出现），不处理
		return
	}

	const tagName = node.tagName.toLowerCase()

	//危险标签：整体移除，绝不解包
	if (removedTags.has(tagName)) {
		node.remove()
		return
	}

	//根容器（body 等）不参与白名单判断，只递归清洗子节点，不解包自身
	if (node === node.ownerDocument?.body) {
		Array.from(node.childNodes).forEach(child => sanitizeNode(child))
		return
	}

	//白名单外的标签：先把其所有子节点深度清洗，再解包（用清洗后的子节点替换自身）
	//注意：解包前必须递归清洗每个子节点，否则"扶正"的节点（如 <svg> 里的 <a href="javascript:">）会绕过属性/事件清洗
	if (!allowedTags.has(tagName)) {
		//先深度清洗直接子节点（含元素与文本）
		Array.from(node.childNodes).forEach(child => sanitizeNode(child))
		//解开白名单外标签，只剩经过清洗的子节点
		node.replaceWith(...Array.from(node.childNodes))
		return
	}

	//白名单内标签：
	//1) 深度清洗所有子节点（先递归，再决定层内去留）
	Array.from(node.childNodes).forEach(child => sanitizeNode(child))

	//2) 属性清洗：on* 事件属性、style 属性一律删除；其余按白名单 + scheme 校验
	Array.from(node.attributes).forEach(attribute => {
		const attrName = attribute.name.toLowerCase()
		const allowedByTag = allowedAttributesByTag[tagName]
		const isAllowedAttribute = globalAllowedAttributes.has(attrName) || (allowedByTag && allowedByTag.has(attrName))
		const isEventAttribute = attrName.startsWith('on')
		const isStyleAttribute = attrName === 'style'

		if (isEventAttribute || isStyleAttribute || !isAllowedAttribute || !isSafeUrl(tagName, attrName, attribute.value)) {
			node.removeAttribute(attribute.name)
		}
	})

	if (tagName === 'a') {
		node.setAttribute('rel', 'external nofollow noopener noreferrer')
	}
}

function parseHtml(value) {
	const parser = new DOMParser()
	return parser.parseFromString(value || '', 'text/html')
}

export function sanitizeRichHtml(value) {
	const document = parseHtml(value)
	sanitizeNode(document.body)
	return document.body.innerHTML
}

export function escapeHtml(value) {
	const div = document.createElement('div')
	div.textContent = value || ''
	return div.innerHTML
}
