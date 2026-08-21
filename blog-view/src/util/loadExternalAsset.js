const loadedAssets = new Map()

function hasStyle(href) {
	return Array.from(document.styleSheets).some(styleSheet => styleSheet.href && new URL(styleSheet.href).pathname === href)
}

function hasScript(src) {
	return Array.from(document.scripts).some(script => script.getAttribute('src') === src)
}

export function loadStyle(href, attrs = {}) {
	if (loadedAssets.has(href)) {
		return loadedAssets.get(href)
	}

	const promise = new Promise((resolve, reject) => {
		if (hasStyle(href)) {
			resolve()
			return
		}

		const link = document.createElement('link')
		link.rel = 'stylesheet'
		link.href = href
		Object.entries(attrs).forEach(([name, value]) => {
			link.setAttribute(name, value === true ? '' : value)
		})
		link.onload = resolve
		link.onerror = error => {
			loadedAssets.delete(href)
			reject(error)
		}
		document.head.appendChild(link)
	})

	loadedAssets.set(href, promise)
	return promise
}

export function loadScript(src, attrs = {}) {
	if (loadedAssets.has(src)) {
		return loadedAssets.get(src)
	}

	const promise = new Promise((resolve, reject) => {
		if (hasScript(src)) {
			resolve()
			return
		}

		const script = document.createElement('script')
		script.src = src
		Object.entries(attrs).forEach(([name, value]) => {
			script.setAttribute(name, value === true ? '' : value)
		})
		script.onload = resolve
		script.onerror = error => {
			loadedAssets.delete(src)
			reject(error)
		}
		document.body.appendChild(script)
	})

	loadedAssets.set(src, promise)
	return promise
}

export async function highlightCodeBlocks(container = document) {
	const target = container || document
	if (!target.querySelector('code[class*="language-"], [class*="language-"] code, code[class*="lang-"], [class*="lang-"] code')) {
		return
	}

	await Promise.all([
		loadStyle('/lib/css/prism.css'),
		loadScript('/lib/js/prism.js', {'data-manual': true}),
	])

	if (window.Prism && typeof window.Prism.highlightAllUnder === 'function') {
		window.Prism.highlightAllUnder(target)
	}
}

export function loadTocbot() {
	return loadScript('/lib/js/tocbot.min.js', {
		integrity: 'sha384-T1Gf4Z/muD8k8yM2MyuI0A/ixPuqQQ3FE4rVdpUCex2cfLgeLRpZAuSdJ1VUTz7h',
		crossorigin: 'anonymous',
	})
}

export async function loadMetingPlayer() {
	window.meting_api = 'https://api.naccl.top/meting/api?server=:server&type=:type&id=:id&auth=:auth&r=:r'
	await loadStyle('/lib/css/APlayer.min.css', {
		integrity: 'sha384-tLMkTWh2pfXNWGFlUS0w1TFtRG5xZ9lPWFOooj+vDDLIL+xBGQU/voDBY5XE2lVh',
		crossorigin: 'anonymous',
	})
	await loadScript('/lib/js/APlayer.min.js', {
		integrity: 'sha384-gdGYZwHnfJM54evoZhpO0s6ZF5BQiybkiyW7VXr+h5UfruuRL/aORyw+5+HZoU6e',
		crossorigin: 'anonymous',
	})
	await loadScript('/lib/js/Meting.min.js')
}
