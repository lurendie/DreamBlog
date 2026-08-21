import {sanitizeRichHtml} from './sanitizeHtml'

function applyNativeLazyImages(el, binding) {
	const options = binding?.value && typeof binding.value === 'object' ? binding.value : {}
	const selector = options.selector || 'img'
	window.requestAnimationFrame(() => {
		el.querySelectorAll(selector).forEach((image, index) => {
			if (image.tagName.toLowerCase() !== 'img') {
				return
			}
			if (!image.hasAttribute('loading')) {
				image.setAttribute('loading', index === 0 && options.firstEager ? 'eager' : 'lazy')
			}
			if (!image.hasAttribute('decoding')) {
				image.setAttribute('decoding', 'async')
			}
		})
	})
}

const directives = {
	'safe-html': {
		mounted(el, binding) {
			el.innerHTML = sanitizeRichHtml(binding.value)
		},
		updated(el, binding) {
			if (binding.value !== binding.oldValue) {
				el.innerHTML = sanitizeRichHtml(binding.value)
			}
		}
	},

	'lazy-container': {
		mounted(el, binding) {
			applyNativeLazyImages(el, binding)
		},
		updated(el, binding) {
			applyNativeLazyImages(el, binding)
		}
	},

	/**
	 * 防抖 单位时间只触发最后一次
	 * 例：<el-button v-debounce="[reset,`click`,300]">刷新</el-button>
	 * 简写：<el-button v-debounce="[reset]">刷新</el-button>
	 */
	debounce: {
		mounted: function (el, binding) {
			let [fn, event = "click", time = 300] = binding.value
			let timer
			el.addEventListener(event, () => {
				timer && clearTimeout(timer)
				timer = setTimeout(() => fn(), time)
			})
		}
	},

	/**
	 * 节流 每单位时间可触发一次
	 * 例：<el-button v-throttle="[reset,`click`,300]">刷新</el-button>
	 * 传递参数：<el-button v-throttle="[()=>reset(param),`click`,300]">刷新</el-button>
	 */
	throttle: {
		mounted: function (el, binding) {
			let [fn, event = "click", time = 300] = binding.value
			let now, preTime
			el.addEventListener(event, () => {
				now = new Date()
				if (!preTime || now - preTime > time) {
					preTime = now
					fn()
				}
			})
		}
	}
}

export default {
	install(app) {
		Object.entries(directives).forEach(([name, directive]) => {
			app.directive(name, directive)
		})
	}
}
