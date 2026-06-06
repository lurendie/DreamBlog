const fs = require('fs')
const path = require('path')

const projectRoot = path.resolve(__dirname, '..')
const publicDir = path.join(projectRoot, 'public')
const robotsPath = path.join(publicDir, 'robots.txt')

function loadEnvFile(filePath) {
	if (!fs.existsSync(filePath)) {
		return
	}
	const content = fs.readFileSync(filePath, 'utf8')
	for (const line of content.split(/\r?\n/)) {
		const trimmed = line.trim()
		if (!trimmed || trimmed.startsWith('#')) {
			continue
		}
		const separatorIndex = trimmed.indexOf('=')
		if (separatorIndex === -1) {
			continue
		}
		const key = trimmed.slice(0, separatorIndex).trim()
		const rawValue = trimmed.slice(separatorIndex + 1).trim()
		const value = rawValue.replace(/^['"]|['"]$/g, '')
		if (!process.env[key]) {
			process.env[key] = value
		}
	}
}

function loadEnv() {
	loadEnvFile(path.join(projectRoot, '.env'))
	loadEnvFile(path.join(projectRoot, '.env.local'))
	loadEnvFile(path.join(projectRoot, '.env.production'))
	loadEnvFile(path.join(projectRoot, '.env.production.local'))
}

function normalizeUrl(value, fallback) {
	return (value || fallback).replace(/\/+$/, '')
}

function main() {
	loadEnv()

	const siteUrl = normalizeUrl(process.env.VUE_APP_SITE_URL, 'http://localhost:8080')
	const apiBaseUrl = normalizeUrl(process.env.VUE_APP_API_BASE_URL, 'http://localhost:8090/blog')
	const sitemapUrl = `${apiBaseUrl}/sitemap.xml`

	const content =
		`User-agent: *\n` +
		`Allow: /\n` +
		`Disallow: ${siteUrl}/login\n` +
		`Sitemap: ${sitemapUrl}\n`

	fs.writeFileSync(robotsPath, content, 'utf8')
	console.log(`robots.txt written to ${robotsPath}`)
}

main()
