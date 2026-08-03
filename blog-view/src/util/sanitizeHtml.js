const allowedTags = new Set([
	'a', 'abbr', 'article', 'aside', 'b', 'blockquote', 'br', 'caption', 'code', 'del', 'details', 'div',
	'em', 'figcaption', 'figure', 'footer', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'header', 'hr', 'i',
	'img', 'ins', 'kbd', 'li', 'main', 'mark', 'menu', 'nav', 'ol', 'p', 'pre', 'q', 's', 'section',
	'small', 'span', 'strong', 'sub', 'sup', 'table', 'tbody', 'td', 'tfoot', 'th', 'thead', 'tr',
	'u', 'ul'
])

const globalAllowedAttributes = new Set(['class', 'id', 'title'])
const allowedAttributesByTag = {
	a: new Set(['href', 'name', 'target', 'rel']),
	img: new Set(['src', 'alt', 'title', 'width', 'height', 'loading']),
	code: new Set(['class']),
	pre: new Set(['class'])
}

const allowedSchemes = new Set(['http:', 'https:', 'mailto:', 'tel:'])
const allowedImageSchemes = new Set(['http:', 'https:', 'data:'])

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
			return allowedImageSchemes.has(url.protocol)
		}
		return allowedSchemes.has(url.protocol)
	} catch (_) {
		return false
	}
}

function sanitizeNode(node) {
	const children = Array.from(node.children)
	children.forEach(child => {
		const tagName = child.tagName.toLowerCase()
		if (!allowedTags.has(tagName)) {
			child.replaceWith(...Array.from(child.childNodes))
			return
		}

		Array.from(child.attributes).forEach(attribute => {
			const attrName = attribute.name.toLowerCase()
			const allowedByTag = allowedAttributesByTag[tagName]
			const isAllowedAttribute = globalAllowedAttributes.has(attrName) || (allowedByTag && allowedByTag.has(attrName))
			const isEventAttribute = attrName.startsWith('on')

			if (isEventAttribute || !isAllowedAttribute || !isSafeUrl(tagName, attrName, attribute.value)) {
				child.removeAttribute(attribute.name)
			}
		})

		if (tagName === 'a') {
			child.setAttribute('rel', 'external nofollow noopener noreferrer')
		}

		sanitizeNode(child)
	})
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
