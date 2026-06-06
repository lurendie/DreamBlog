import store from '@/store'
import getPageTitle from '@/util/get-page-title'

const DEFAULT_DESCRIPTION = 'DreamBlog 是一个基于 Vue 与 Rust 的个人博客站点，包含文章、归档、分类、标签、动态、友链与关于页面。'
const DEFAULT_KEYWORDS = 'DreamBlog,博客,技术博客,Rust,Vue,前端,后端'

function ensureMeta(attr, key, content) {
	let node = document.head.querySelector(`meta[${attr}="${key}"]`)
	if (!node) {
		node = document.createElement('meta')
		node.setAttribute(attr, key)
		document.head.appendChild(node)
	}
	node.setAttribute('content', content)
}

function ensureLink(rel, href) {
	let node = document.head.querySelector(`link[rel="${rel}"]`)
	if (!node) {
		node = document.createElement('link')
		node.setAttribute('rel', rel)
		document.head.appendChild(node)
	}
	node.setAttribute('href', href)
}

function stripHtml(value = '') {
	return value
		.replace(/<[^>]*>/g, ' ')
		.replace(/\s+/g, ' ')
		.trim()
}

function truncate(value, maxLength = 160) {
	if (!value) {
		return ''
	}
	if (value.length <= maxLength) {
		return value
	}
	return `${value.slice(0, maxLength - 1).trim()}…`
}

function getOrigin() {
	return process.env.VUE_APP_SITE_URL || window.location.origin || ''
}

function toAbsoluteUrl(path = window.location.pathname) {
	try {
		return new URL(path, getOrigin()).toString()
	} catch (_) {
		return `${getOrigin()}${path}`
	}
}

function getSiteTitleSuffix() {
	return store.state.siteInfo?.webTitleSuffix || ''
}

function getSiteName() {
	const blogName = store.state.siteInfo?.blogName
	if (blogName) {
		return blogName
	}
	const suffix = getSiteTitleSuffix()
	return suffix ? suffix.replace(/^[\s\-|_]+/, '') : 'DreamBlog'
}

export function createDescription(value, fallback = DEFAULT_DESCRIPTION) {
	const cleaned = truncate(stripHtml(value))
	return cleaned || fallback
}

export function updateSeo({
	title,
	description = DEFAULT_DESCRIPTION,
	keywords = DEFAULT_KEYWORDS,
	path = window.location.pathname,
	image,
	type = 'website',
	noindex = false,
	author,
	publishedTime,
	modifiedTime,
} = {}) {
	document.title = title ? getPageTitle(title) : getPageTitle(getSiteName())

	const absoluteUrl = toAbsoluteUrl(path)
	const resolvedTitle = document.title
	const resolvedDescription = createDescription(description)
	const robots = noindex ? 'noindex,nofollow' : 'index,follow'

	ensureMeta('name', 'description', resolvedDescription)
	ensureMeta('name', 'keywords', keywords)
	ensureMeta('name', 'robots', robots)
	ensureMeta('property', 'og:type', type)
	ensureMeta('property', 'og:title', resolvedTitle)
	ensureMeta('property', 'og:description', resolvedDescription)
	ensureMeta('property', 'og:url', absoluteUrl)
	ensureMeta('property', 'og:site_name', getSiteName())
	ensureMeta('property', 'og:locale', 'zh_CN')
	ensureMeta('name', 'twitter:card', image ? 'summary_large_image' : 'summary')
	ensureMeta('name', 'twitter:title', resolvedTitle)
	ensureMeta('name', 'twitter:description', resolvedDescription)

	if (image) {
		const absoluteImage = toAbsoluteUrl(image)
		ensureMeta('property', 'og:image', absoluteImage)
		ensureMeta('name', 'twitter:image', absoluteImage)
	} else {
		const ogImage = document.head.querySelector('meta[property="og:image"]')
		const twitterImage = document.head.querySelector('meta[name="twitter:image"]')
		if (ogImage) {
			ogImage.remove()
		}
		if (twitterImage) {
			twitterImage.remove()
		}
	}

	if (author) {
		ensureMeta('name', 'author', author)
	}
	if (publishedTime) {
		ensureMeta('property', 'article:published_time', publishedTime)
	}
	if (modifiedTime) {
		ensureMeta('property', 'article:modified_time', modifiedTime)
	}

	ensureLink('canonical', absoluteUrl)
}
