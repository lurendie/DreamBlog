import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'node:path'

export default defineConfig({
	plugins: [vue()],
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
	server: {
		port: 8080,
	},
})
