import path from 'node:path'
import {fileURLToPath} from 'node:url'
import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import {ElementPlusResolver} from 'unplugin-vue-components/resolvers'
import {createSvgIconsPlugin} from 'vite-plugin-svg-icons'
import settings from './src/settings.js'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

function resolve(dir) {
	return path.resolve(__dirname, dir)
}

export default defineConfig({
	plugins: [
		vue(),
		AutoImport({
			dts: false,
			resolvers: [ElementPlusResolver({importStyle: 'css'})],
		}),
		Components({
			dts: false,
			resolvers: [ElementPlusResolver({importStyle: 'css'})],
		}),
		createSvgIconsPlugin({
			iconDirs: [resolve('src/icons/svg')],
			symbolId: 'icon-[name]',
		}),
	],
	resolve: {
		alias: {
			'@': resolve('src'),
			'@/layout': resolve('src/layout/index.vue'),
			'@/components/SvgIcon': resolve('src/components/SvgIcon/index.vue'),
			'@/components/Breadcrumb': resolve('src/components/Breadcrumb/index.vue'),
			'@/components/Hamburger': resolve('src/components/Hamburger/index.vue'),
		},
		extensions: ['.mjs', '.js', '.mts', '.ts', '.jsx', '.tsx', '.json', '.vue'],
	},
	server: {
		host: '0.0.0.0',
		port: Number(process.env.port || process.env.npm_config_port || 8079),
		open: true,
	},
	css: {
		preprocessorOptions: {
			scss: {
				api: 'modern',
			},
		},
	},
	build: {
		outDir: 'dist',
		assetsDir: 'static',
		sourcemap: false,
		rollupOptions: {
			output: {
				manualChunks: {
					vue: ['vue', 'vue-router', 'vuex'],
				},
			},
		},
	},
	define: {
		__APP_TITLE__: JSON.stringify(settings.title),
	},
})
