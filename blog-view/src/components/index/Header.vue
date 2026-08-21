<template>
	<header ref="header">
		<div class="view">
			<img ref="imgbg1" :src="defaultSettings.bg1" class="hero-preload" alt="" aria-hidden="true" fetchpriority="high" decoding="async">
			<div class="bg1" :style="{backgroundImage:'url('+defaultSettings.bg1+')'}"></div>
			<div class="bg2" :style="{backgroundImage: layeredLoaded ? 'url('+defaultSettings.bg2+')' : 'none'}"></div>
			<div class="bg3" :style="{backgroundImage: layeredLoaded ? 'url('+defaultSettings.bg3+')' : 'none'}" v-show="layeredLoaded"></div>
		</div>
		<div class="text-malfunction" :data-word="defaultSettings.malfunctionText">
			{{ defaultSettings.malfunctionText }}
			<div class="line"></div>
		</div>
		<div class="wrapper">
			<i class="ali-iconfont icon-down" @click="scrollToMain"></i>
		</div>
		<div class="wave1"></div>
		<div class="wave2"></div>
	</header>
</template>

<script>
	import {mapState} from 'vuex'
	import defaultSettings from '@/settings'

	export default {
		name: "Header",
		data() {
			return {
				bg1Loaded: false,
				layeredLoaded: false,
				defaultSettings,
				hoverHandlers: [],
				cancelLayerLoad: null,
				isUnmounted: false,
			}
		},
		computed: {
			...mapState(['clientSize'])
		},
		watch: {
			'clientSize.clientHeight'() {
				this.setHeaderHeight()
			}
		},
		mounted() {
			/**
			 * 因为bg3.jpg比较小，通常会比bg1.jpg先加载，显示出来会有一瞬间bg1显示一半，bg3显示一半，为了解决这个问题，增加这个判断，让bg1加载完毕后再显示bg3
			 * HTML中使用img标签的原因：我个人想用div作为图片的载体，而只有img标签有图片加载完毕的onload回调，所以用一个display: none的img人柱力来加载图片
			 * 当img中的src加载完毕后，会把图片缓存到浏览器，后续在div中用background url的形式将直接从浏览器中取出图片，不会下载两次图片
			 */
			this.$refs.imgbg1.onload = this.handlePrimaryImageLoaded
			//图片已缓存时 load 事件不会再次触发，直接置为已加载
			if (this.$refs.imgbg1.complete) {
				this.handlePrimaryImageLoaded()
			}
			this.setHeaderHeight()
			let startingPoint
			const header = this.$refs.header
			const handleMouseEnter = (e) => {
				startingPoint = e.clientX
			}
			const handleMouseOut = () => {
				header.classList.remove('moving')
				header.style.setProperty('--percentage', 0.5)
			}
			const handleMouseMove = (e) => {
				let percentage = (e.clientX - startingPoint) / window.outerWidth + 0.5
				header.style.setProperty('--percentage', percentage)
				header.classList.add('moving')
			}
			header.addEventListener('mouseenter', handleMouseEnter)
			header.addEventListener('mouseout', handleMouseOut)
			header.addEventListener('mousemove', handleMouseMove)
			this.hoverHandlers = [
				['mouseenter', handleMouseEnter],
				['mouseout', handleMouseOut],
				['mousemove', handleMouseMove],
			]
		},
		beforeUnmount() {
			this.isUnmounted = true
			const header = this.$refs.header
			if (header) {
				this.hoverHandlers.forEach(([eventName, handler]) => {
					header.removeEventListener(eventName, handler)
				})
			}
			if (this.cancelLayerLoad) {
				this.cancelLayerLoad()
				this.cancelLayerLoad = null
			}
		},
		methods: {
			handlePrimaryImageLoaded() {
				if (this.bg1Loaded) {
					return
				}
				this.bg1Loaded = true
				this.scheduleLayeredImages()
			},
			scheduleLayeredImages() {
				const load = () => {
					Promise.all([
						this.preloadImage(defaultSettings.bg2),
						this.preloadImage(defaultSettings.bg3),
					]).then(() => {
						if (this.isUnmounted) {
							return
						}
						this.layeredLoaded = true
					}).catch(() => {
						if (this.isUnmounted) {
							return
						}
						this.layeredLoaded = false
					})
				}
				if (window.requestIdleCallback) {
					const idleHandle = window.requestIdleCallback(load, {timeout: 1800})
					this.cancelLayerLoad = () => window.cancelIdleCallback(idleHandle)
				} else {
					const timeoutHandle = setTimeout(load, 800)
					this.cancelLayerLoad = () => clearTimeout(timeoutHandle)
				}
			},
			preloadImage(src) {
				return new Promise((resolve, reject) => {
					const image = new Image()
					image.onload = resolve
					image.onerror = reject
					image.decoding = 'async'
					image.src = src
				})
			},
			//根据可视窗口高度，动态改变首图大小
			setHeaderHeight() {
				this.$refs.header.style.height = this.clientSize.clientHeight + 'px'
			},
			//平滑滚动至正文部分
			scrollToMain() {
				window.scrollTo({top: this.clientSize.clientHeight, behavior: 'smooth'})
			}
		},
	}
</script>

<style scoped>
	header {
		--percentage: 0.5;
		position: relative;
		overflow: hidden;
		user-select: none;
	}

	header:after {
		content: '';
		position: absolute;
		inset: 0;
		z-index: 30;
		background:
			linear-gradient(180deg, rgba(15, 23, 42, 0.42), rgba(15, 23, 42, 0.16) 38%, rgba(15, 23, 42, 0.48)),
			radial-gradient(circle at 50% 40%, rgba(20, 184, 166, 0.18), transparent 28rem);
		pointer-events: none;
	}

	.view {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		display: flex;
		justify-content: center;
		transform: translatex(calc(var(--percentage) * 100px));
	}

	.view div {
		background-position: center center;
		background-size: cover;
		position: absolute;
		width: 110%;
		height: 100%;
	}

	.view .bg1 {
		z-index: 10;
		opacity: calc(1 - (var(--percentage) - 0.5) / 0.5);
	}

	.view .bg2 {
		z-index: 20;
		opacity: calc(1 - (var(--percentage) - 0.25) / 0.25);
	}

	.view .bg3 {
		left: -10%;
	}

	header .view,
	header .bg1,
	header .bg2 {
		transition: .2s all ease-in;
	}

	header.moving .view,
	header.moving .bg1,
	header.moving .bg2 {
		transition: none;
	}

	.text-malfunction {
		position: absolute;
		z-index: 60;
		padding: 0 4px;
		top: 42%;
		left: 50%;
		max-width: min(920px, calc(100vw - 40px));
		transform: translate(-50%, -50%);
		font-size: clamp(42px, 8vw, 92px);
		font-family: Georgia, "Times New Roman", "Noto Serif SC", serif;
		font-weight: 700;
		line-height: 1.05;
		color: #fff;
		text-align: center;
		text-shadow: 0 22px 60px rgba(0, 0, 0, 0.45);
	}

	.hero-preload {
		display: none;
	}

	.line {
		display: none;
	}

	.text-malfunction:before, .text-malfunction:after {
		content: none;
	}

	.text-malfunction:before {
		left: 0;
		color: red;
		text-shadow: 1px 0 0 red;
		z-index: 30;
		animation: malfunctionAni 0.95s infinite;
	}

	.text-malfunction:after {
		left: -1px;
		color: cyan;
		text-shadow: -1px 0 0 cyan;
		z-index: 40;
		mix-blend-mode: lighten;
		animation: malfunctionAni 1.1s infinite 0.2s;
	}

	@keyframes lineMove {
		9% {
			top: 38px;
		}
		14% {
			top: 8px;
		}
		18% {
			top: 42px;
		}
		22% {
			top: 1px;
		}
		32% {
			top: 32px;
		}
		34% {
			top: 12px;
		}
		40% {
			top: 26px;
		}
		43% {
			top: 7px;
		}
		99% {
			top: 30px;
		}
	}

	@keyframes malfunctionAni {
		10% {
			top: -0.4px;
			left: -1.1px;
		}
		20% {
			top: 0.4px;
			left: -0.2px;
		}
		30% {
			left: .5px;
		}
		40% {
			top: -0.3px;
			left: -0.7px;
		}
		50% {
			left: 0.2px;
		}
		60% {
			top: 1.8px;
			left: -1.2px;
		}
		70% {
			top: -1px;
			left: 0.1px;
		}
		80% {
			top: -0.4px;
			left: -0.9px;
		}
		90% {
			left: 1.2px;
		}
		100% {
			left: -1.2px;
		}
	}

	.wrapper {
		position: absolute;
		width: 100px;
		bottom: 110px;
		left: 0;
		right: 0;
		margin: auto;
		font-size: 26px;
		z-index: 70;
		color: #fff;
		text-align: center;
	}

	.wrapper i {
		font-size: 60px;
		opacity: 0.5;
		cursor: pointer;
		position: absolute;
		top: 55px;
		left: 20px;
		animation: opener .5s ease-in-out alternate infinite;
		transition: opacity .2s ease-in-out, transform .5s ease-in-out .2s;
	}

	.wrapper i:hover {
		opacity: 1;
	}

	@keyframes opener {
		100% {
			top: 65px
		}
	}

	.wave1, .wave2 {
		position: absolute;
		bottom: 0;
		transition-duration: .4s, .4s;
		z-index: 80;
	}

	.wave1 {
		background: url('/img/header/wave1.png') repeat-x;
		height: 75px;
		width: 100%;
	}

	.wave2 {
		background: url('/img/header/wave2.png') repeat-x;
		height: 90px;
		width: calc(100% + 100px);
		left: -100px;
	}
</style>
