<template>
	<div
		ref="nav"
		class="ui fixed inverted stackable pointing menu"
		:class="{
			'transparent': $route.name==='home' && clientSize.clientWidth>768,
			'mobile-open': !mobileHide && clientSize.clientWidth <= 767
		}"
	>
		<div class="ui container">
			<router-link to="/" class="brand-link">
				<h3 class="ui header item m-blue brand-title">{{ blogName }}</h3>
			</router-link>
			<router-link to="/home" class="item" :class="{'m-mobile-hide': mobileHide,'active':$route.name==='home','m-mobile-menu-item': !mobileHide}">
				<i class="home icon"></i>首页
			</router-link>
			<template v-if="isMobile">
				<div
					class="item m-mobile-menu-item category-wrapper-mobile category-inline-trigger"
					:class="{'active': $route.name==='category', 'is-hidden': mobileHide}"
					@click.stop="toggleMobileCategories"
				>
					<span class="category-inline-label">
						<i class="idea icon"></i>分类
					</span>
					<i class="caret icon" :class="showMobileCategories ? 'up' : 'down'"></i>
				</div>
				<div v-if="!mobileHide && showMobileCategories" class="mobile-category-panel" @click.stop>
					<a
						v-for="(category,index) in categoryList"
						:key="index"
						class="mobile-category-item"
						@click.prevent="categoryRoute(category.name)"
					>{{ category.name }}</a>
					<div v-if="!categoryList.length" class="mobile-category-empty">暂无分类</div>
				</div>
			</template>
			<el-dropdown v-else class="category-wrapper-mobile" trigger="click" @command="categoryRoute">
				<span class="el-dropdown-link item" :class="{'m-mobile-hide': mobileHide,'active':$route.name==='category'}">
					<i class="idea icon"></i>分类<i class="caret down icon"></i>
				</span>
				<template #dropdown>
					<el-dropdown-menu>
						<el-dropdown-item :command="category.name" v-for="(category,index) in categoryList" :key="index">{{ category.name }}</el-dropdown-item>
					</el-dropdown-menu>
				</template>
			</el-dropdown>
			<router-link to="/archives" class="item" :class="{'m-mobile-hide': mobileHide,'active':$route.name==='archives','m-mobile-menu-item': !mobileHide}">
				<i class="clone icon"></i>归档
			</router-link>
			<router-link to="/moments" class="item" :class="{'m-mobile-hide': mobileHide,'active':$route.name==='moments','m-mobile-menu-item': !mobileHide}">
				<i class="comment alternate outline icon"></i>动态
			</router-link>
			<router-link to="/friends" class="item" :class="{'m-mobile-hide': mobileHide,'active':$route.name==='friends','m-mobile-menu-item': !mobileHide}">
				<i class="users icon"></i>友人帐
			</router-link>
			<router-link to="/about" class="item" :class="{'m-mobile-hide': mobileHide,'active':$route.name==='about','m-mobile-menu-item': !mobileHide}">
				<i class="info icon"></i>关于我
			</router-link>
			<el-autocomplete v-model="queryString" :fetch-suggestions="debounceQuery" placeholder="Search..."
			                 class="right item m-search" :class="{'m-mobile-hide': mobileHide}"
			                 popper-class="m-search-item" @select="handleSelect">
				<template #suffix>
					<i class="search icon el-input__icon"></i>
				</template>
				<template #default="{ item }">
					<div class="title">{{ item.title }}</div>
					<span class="content">{{ item.content }}</span>
				</template>
			</el-autocomplete>
			<button class="ui menu black icon button m-right-top m-mobile-show" @click="toggle">
				<i class="sidebar icon"></i>
			</button>
		</div>
	</div>
</template>

<script>
	import {getSearchBlogList} from "@/api/blog";
	import {mapState} from 'vuex'

	export default {
		name: "Nav",
		props: {
			blogName: {
				type: String,
				required: true
			},
			categoryList: {
				type: Array,
				required: true
			},
		},
		data() {
			return {
				mobileHide: true,
				showMobileCategories: false,
				queryString: '',
				queryResult: [],
				timer: null,
				handleScroll: null,
				handleDocumentClick: null,
			}
		},
		computed: {
			...mapState(['clientSize']),
			isMobile() {
				return this.clientSize.clientWidth <= 767
			}
		},
		watch: {
			//路由改变时，收起导航栏
			'$route.path'() {
				this.mobileHide = true
				this.showMobileCategories = false
			}
		},
		mounted() {
			//监听页面滚动位置，改变导航栏的显示
			this.handleScroll = () => {
				//首页且不是移动端
				if (!this.$refs.nav) {
					return
				}
				if (this.$route.name === 'home' && this.clientSize.clientWidth > 768) {
					if (window.scrollY > this.clientSize.clientHeight / 2) {
						this.$refs.nav.classList.remove('transparent')
					} else {
						this.$refs.nav.classList.add('transparent')
					}
				}
			}
			window.addEventListener('scroll', this.handleScroll)
			//监听点击事件，收起导航菜单
			this.handleDocumentClick = (e) => {
				const nav = this.$refs.nav
				if (!nav) {
					return
				}
				//遍历冒泡
				let flag = nav.contains(e.target)
				//如果导航栏是打开状态，且点击的元素不是Nav的子元素，则收起菜单
				if (!this.mobileHide && !flag) {
					this.mobileHide = true
					this.showMobileCategories = false
				}
			}
			document.addEventListener('click', this.handleDocumentClick)
		},
		beforeUnmount() {
			if (this.timer) {
				clearTimeout(this.timer)
				this.timer = null
			}
			if (this.handleScroll) {
				window.removeEventListener('scroll', this.handleScroll)
			}
			if (this.handleDocumentClick) {
				document.removeEventListener('click', this.handleDocumentClick)
			}
		},
		methods: {
			toggle() {
				this.mobileHide = !this.mobileHide
				if (this.mobileHide) {
					this.showMobileCategories = false
				}
			},
			toggleMobileCategories() {
				if (this.isMobile) {
					this.showMobileCategories = !this.showMobileCategories
				}
			},
			categoryRoute(name) {
				this.showMobileCategories = false
				this.mobileHide = true
				this.$router.push(`/category/${name}`)
			},
			debounceQuery(queryString, callback) {
				this.timer && clearTimeout(this.timer)
				this.timer = setTimeout(() => this.querySearchAsync(queryString, callback), 1000)
			},
			querySearchAsync(queryString, callback) {
				if (queryString == null
						|| queryString.trim() === ''
						|| queryString.indexOf('%') !== -1
						|| queryString.indexOf('_') !== -1
						|| queryString.indexOf('[') !== -1
						|| queryString.indexOf('#') !== -1
						|| queryString.indexOf('*') !== -1
						|| queryString.trim().length > 20) {
					return
				}
				getSearchBlogList(queryString).then(res => {
					if (res.code === 200) {
						this.queryResult = res.data
						if (this.queryResult.length === 0) {
							this.queryResult.push({title: '无相关搜索结果'})
						}
						callback(this.queryResult)
					}
				}).catch(() => {
					this.msgError("请求失败")
				})
			},
			handleSelect(item) {
				if (item.id) {
					//复用统一的隐私判断：密码保护文章未验证时弹密码框，而不是直接跳转
					this.$store.dispatch('goBlogPage', item)
				}
			}
		}
	}
</script>

<style>
	.ui.fixed.menu .container {
		width: min(1400px, calc(100vw - 24px)) !important;
		margin-left: auto !important;
		margin-right: auto !important;
	}

	.ui.fixed.menu .brand-link {
		display: flex;
		align-items: stretch;
		color: inherit;
		text-decoration: none;
	}

	.ui.fixed.menu .brand-title {
		min-width: 180px;
		justify-content: center;
	}

	.ui.fixed.menu .brand-title:before,
	.ui.fixed.menu .brand-title:after {
		content: none !important;
		display: none !important;
	}

	.ui.fixed.menu {
		background: rgba(255, 255, 255, 0.9) !important;
		border: 0 !important;
		backdrop-filter: blur(16px);
		box-shadow: 0 12px 34px rgba(15, 23, 42, 0.08) !important;
		transition: background .3s ease-out, box-shadow .3s ease-out, min-height .25s ease;
	}

	.ui.inverted.pointing.menu.transparent {
		background: transparent !important;
		box-shadow: none !important;
	}

	.ui.inverted.pointing.menu.transparent .item,
	.ui.inverted.pointing.menu.transparent .item > i,
	.ui.inverted.pointing.menu.transparent .el-dropdown-link,
	.ui.inverted.pointing.menu.transparent .ui.header.item {
		color: #fff !important;
		text-shadow: 0 8px 24px rgba(0, 0, 0, 0.45);
	}

	.ui.inverted.pointing.menu.transparent .active.item:after {
		background: transparent !important;
		transition: .3s ease-out;
	}

	.ui.inverted.pointing.menu.transparent .active.item:hover:after {
		background: transparent !important;
	}

	.ui.inverted.pointing.menu .item,
	.ui.inverted.pointing.menu .item > i,
	.ui.inverted.pointing.menu .el-dropdown-link {
		color: #334155 !important;
	}

	.ui.inverted.pointing.menu .item:hover,
	.ui.inverted.pointing.menu .el-dropdown-link:hover {
		background: rgba(20, 184, 166, 0.08) !important;
	}

	.ui.inverted.pointing.menu .active.item {
		background: rgba(20, 184, 166, 0.12) !important;
	}

	.ui.inverted.pointing.menu .active.item:after {
		background: #14b8a6 !important;
	}

	.el-dropdown-link {
		outline-style: none !important;
		outline-color: unset !important;
		height: 100%;
		cursor: pointer;
	}

	.el-dropdown-menu {
		margin: 7px 0 0 0 !important;
		padding: 0 !important;
		border: 0 !important;
		background: #ffffff !important;
		box-shadow: 0 16px 40px rgba(63, 122, 186, 0.18) !important;
		border-radius: 12px !important;
	}

	.el-dropdown-menu__item {
		padding: 0 15px !important;
		color: #337ecc !important;
		font-size: 16px !important;
		line-height: 42px !important;
	}

	.el-dropdown-menu__item:hover {
		background: rgba(64, 158, 255, .10) !important;
	}

	.el-popper .popper__arrow::after {
		content: none !important;
	}

	.popper__arrow {
		display: none !important;
	}

	.m-search {
		display: flex !important;
		align-items: center;
		min-width: 220px;
		max-width: 260px;
		margin-left: auto !important;
		padding: 0 !important;
		background: transparent !important;
		box-shadow: none !important;
		border: 0 !important;
	}

	.m-search .el-input {
		display: flex;
		align-items: center;
	}

	.m-search .el-input__wrapper {
		padding: 0 12px !important;
		background: transparent !important;
		box-shadow: none !important;
		border-radius: 0 !important;
	}

	.m-search .el-input__inner {
		height: 38px !important;
		color: #4b5563 !important;
		background-color: transparent !important;
	}

	.m-search .el-input__inner::placeholder {
		color: #9ca3af !important;
	}

	.m-search .el-input__suffix,
	.m-search .el-input__suffix-inner {
		display: flex;
		align-items: center;
	}

	.m-search i,
	.m-search .el-input__icon {
		color: #6b7280 !important;
	}

	.m-search-item {
		min-width: 350px !important;
	}

	.m-search-item li {
		line-height: normal !important;
		padding: 8px 10px !important;
	}

	.m-search-item li .title {
		text-overflow: ellipsis;
		overflow: hidden;
		color: rgba(0, 0, 0, 0.87);
	}

	.m-search-item li .content {
		text-overflow: ellipsis;
		font-size: 12px;
		color: rgba(0, 0, 0, .70);
	}

	@media screen and (max-width: 767px) {
		.ui.fixed.menu {
			min-height: 56px;
			padding: 0 !important;
			backdrop-filter: blur(10px);
			overflow: visible !important;
			z-index: 1001 !important;
		}

		.ui.fixed.menu .container {
			width: calc(100vw - 16px) !important;
			display: flex !important;
			flex-wrap: wrap;
			align-items: center;
			align-content: flex-start;
			padding: 8px 0 0;
			overflow: visible !important;
		}

		.ui.fixed.menu.mobile-open {
			height: auto !important;
			padding-bottom: 8px !important;
			box-shadow: 0 18px 44px rgba(31, 41, 55, 0.14) !important;
		}

		.ui.fixed.menu .item,
		.ui.fixed.menu .el-dropdown-link {
			font-size: 15px !important;
		}

		.ui.fixed.menu .ui.header.item {
			min-width: 0;
			justify-content: flex-start;
			padding-left: 0.75rem !important;
			padding-right: 0.75rem !important;
			font-size: 22px !important;
		}

		.ui.fixed.menu .m-mobile-hide {
			display: none !important;
		}

		.ui.fixed.menu .m-mobile-hide.item,
		.ui.fixed.menu .m-mobile-hide.el-dropdown-link,
		.ui.fixed.menu .category-wrapper-mobile,
		.ui.fixed.menu .m-search.m-mobile-hide {
			width: 100%;
		}

		.ui.fixed.menu .item.m-mobile-hide,
		.ui.fixed.menu .el-dropdown-link.m-mobile-hide {
			display: none !important;
		}

		.ui.fixed.menu .item:not(.m-mobile-hide),
		.ui.fixed.menu .el-dropdown-link:not(.m-mobile-hide) {
			padding-top: 0.9rem !important;
			padding-bottom: 0.9rem !important;
		}

		.ui.fixed.menu .item.m-mobile-menu-item,
		.ui.fixed.menu .category-wrapper-mobile {
			display: flex !important;
			align-items: center;
			justify-content: space-between;
			width: 100%;
			margin: 0;
			border-top: 1px solid rgba(15, 23, 42, 0.06);
			background: rgba(255, 255, 255, 0.98);
			animation: mobileMenuFadeIn .18s ease-out;
		}

		.ui.fixed.menu .category-wrapper-mobile.is-hidden {
			display: none !important;
		}

		.ui.fixed.menu .category-wrapper-mobile .el-dropdown-link {
			width: 100%;
		}

		.ui.fixed.menu .category-inline-trigger {
			cursor: pointer;
		}

		.ui.fixed.menu .category-inline-label {
			display: inline-flex;
			align-items: center;
		}

		.mobile-category-panel {
			display: block;
			width: 100%;
			padding: 0.35rem 0.25rem 0.75rem;
			background: rgba(247, 250, 255, 0.98);
			border-top: 1px solid rgba(15, 23, 42, 0.05);
			animation: mobileMenuFadeIn .18s ease-out;
			position: relative;
			z-index: 2;
		}

		.mobile-category-item {
			display: block;
			padding: 0.78rem 0.9rem;
			margin: 0.3rem 0;
			border-radius: 12px;
			color: #0f766e !important;
			font-size: 16px;
			font-weight: 500;
			background: #ffffff;
			box-shadow: inset 0 0 0 1px rgba(20, 184, 166, 0.08);
		}

		.mobile-category-empty {
			padding: 0.85rem 0.95rem;
			border-radius: 12px;
			color: #6b7280;
			font-size: 14px;
			background: rgba(255, 255, 255, 0.92);
			box-shadow: inset 0 0 0 1px rgba(148, 163, 184, 0.12);
		}

		.ui.fixed.menu .item.m-mobile-menu-item i,
		.ui.fixed.menu .category-wrapper-mobile i {
			margin-right: 8px !important;
		}

		.m-search {
			min-width: 0;
			max-width: 100%;
			width: 100%;
			margin-left: 0 !important;
			padding: 0.35rem 0 0.75rem !important;
			order: 20;
			animation: mobileMenuFadeIn .22s ease-out;
		}

		.m-search .el-input__wrapper {
			padding: 0 10px !important;
			border-radius: 10px !important;
			background: rgba(255, 255, 255, 0.9) !important;
			box-shadow: inset 0 0 0 1px rgba(20, 184, 166, 0.18) !important;
		}

		.m-search .el-input__inner {
			height: 40px !important;
		}

		.m-search-item {
			min-width: 0 !important;
			width: calc(100vw - 32px) !important;
			max-width: calc(100vw - 32px) !important;
		}

		.m-right-top {
			top: 6px;
			right: 4px;
			display: inline-flex !important;
			width: 42px !important;
			height: 42px !important;
			align-items: center;
			justify-content: center;
			padding: 0 !important;
			border: 0 !important;
			border-radius: 12px !important;
			background: rgba(20, 184, 166, 0.14) !important;
			color: #1f2937 !important;
			box-shadow: none !important;
			cursor: pointer;
		}

		.m-right-top:hover,
		.m-right-top:active {
			background: rgba(20, 184, 166, 0.22) !important;
		}

		@keyframes mobileMenuFadeIn {
			from {
				opacity: 0;
				transform: translateY(-6px);
			}
			to {
				opacity: 1;
				transform: translateY(0);
			}
		}
	}
</style>
