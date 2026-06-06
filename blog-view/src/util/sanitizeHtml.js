import sanitizeHtml from 'sanitize-html'

const richHtmlOptions = {
	allowedTags: [
		...sanitizeHtml.defaults.allowedTags,
		'img',
		'h1',
		'h2',
		'h3',
		'h4',
		'h5',
		'h6',
		'pre',
		'code',
		'span',
		'div',
		'table',
		'thead',
		'tbody',
		'tr',
		'th',
		'td'
	],
	allowedAttributes: {
		'*': ['class', 'id', 'title'],
		a: ['href', 'name', 'target', 'rel'],
		img: ['src', 'alt', 'title', 'width', 'height', 'loading'],
		code: ['class'],
		pre: ['class']
	},
	allowedSchemes: ['http', 'https', 'mailto', 'tel'],
	allowedSchemesByTag: {
		img: ['http', 'https', 'data']
	},
	transformTags: {
		a: sanitizeHtml.simpleTransform('a', {
			rel: 'external nofollow noopener noreferrer'
		}, true)
	}
}

export function sanitizeRichHtml(value) {
	return sanitizeHtml(value || '', richHtmlOptions)
}
