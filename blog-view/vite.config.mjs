import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

function getPackageName(id) {
	const normalized = id.split('\\').join('/')
	const parts = normalized.split('/node_modules/')[1]?.split('/') || []
	if (!parts.length) {
		return ''
	}
	return parts[0].startsWith('@') ? `${parts[0]}/${parts[1] || ''}` : parts[0]
}

export default defineConfig({
	plugins: [
		vue({
			template: {
				compilerOptions: {
					isCustomElement: tag => tag === 'meting-js',
				},
			},
		}),
		Components({
			dts: false,
			resolvers: [ElementPlusResolver({ importStyle: 'css' })],
		}),
	],
	resolve: {
		alias: {
			'@': path.resolve(__dirname, 'src'),
			assets: path.resolve(__dirname, 'src/assets'),
			common: path.resolve(__dirname, 'src/common'),
			components: path.resolve(__dirname, 'src/components'),
			api: path.resolve(__dirname, 'src/api'),
			views: path.resolve(__dirname, 'src/views'),
			plugins: path.resolve(__dirname, 'src/plugins'),
		},
	},
	build: {
		rollupOptions: {
			output: {
				manualChunks(id) {
					if (!id.includes('node_modules')) {
						return
					}
					const packageName = getPackageName(id)

					if (['vue', 'vue-router', 'vuex'].includes(packageName)) {
						return 'vue-vendor'
					}
					if (packageName === 'element-plus') {
						return
					}
					if (packageName === '@element-plus/icons-vue') {
						return 'element-plus-icons'
					}
					if (
						[
							'@floating-ui/dom',
							'@floating-ui/core',
							'@floating-ui/utils',
							'@popperjs/core',
							'@sxzz/popperjs-es',
						].includes(packageName)
					) {
						return 'floating-vendor'
					}
					if (
						[
							'@ctrl/tinycolor',
							'async-validator',
							'dayjs',
							'lodash',
							'lodash-es',
							'lodash-unified',
							'memoize-one',
							'normalize-wheel-es',
							'@vueuse/core',
							'@vueuse/shared',
						].includes(packageName)
					) {
						return 'element-plus-utils'
					}
					if (['viewerjs', 'v-viewer', 'vue3-lazyload'].includes(packageName)) {
						return 'media-vendor'
					}
					if (['semantic-ui-css'].includes(packageName)) {
						return 'semantic-ui'
					}
					return 'vendor'
				},
			},
		},
	},
	server: {
		port: 8080,
	},
})
