import path from 'path'
import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import {createSvgIconsPlugin} from 'vite-plugin-svg-icons'
import settings from './src/settings.js'

function resolve(dir) {
	return path.resolve(__dirname, dir)
}

export default defineConfig({
	plugins: [
		vue(),
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
	build: {
		outDir: 'dist',
		assetsDir: 'static',
		sourcemap: false,
		rollupOptions: {
			output: {
				manualChunks: {
					vue: ['vue', 'vue-router', 'vuex'],
					elementPlus: ['element-plus'],
					editor: ['md-editor-v3'],
				},
			},
		},
	},
	define: {
		__APP_TITLE__: JSON.stringify(settings.title),
	},
})
