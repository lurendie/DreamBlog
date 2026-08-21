<template>
	<div class="site">
		<!--顶部导航-->
		<Nav :blogName="siteInfo.blogName" :categoryList="categoryList"/>
		<!--首页大图 只在首页且pc端时显示-->
		<div class="m-mobile-hide">
			<Header v-if="$route.name==='home' && !isMobile"/>
		</div>

		<div class="main">
			<div class="m-padded-tb-big">
				<div class="ui container">
					<div class="ui stackable grid">
						<!--左侧-->
						<div v-if="!isMobile" class="three wide column m-mobile-hide">
							<Introduction :class="{'m-display-none':focusMode}"/>
						</div>
						<!--中间-->
						<div class="ten wide column">
							<router-view v-slot="{ Component }">
								<keep-alive include="Home">
									<component :is="Component"/>
								</keep-alive>
							</router-view>
						</div>
						<!--右侧-->
						<div v-if="!isMobile" class="three wide column m-mobile-hide">
							<RandomBlog :randomBlogList="randomBlogList" :class="{'m-display-none':focusMode}"/>
							<Tags :tagList="tagList" :class="{'m-display-none':focusMode}"/>
							<!--只在文章页面显示目录-->
							<Tocbot v-if="$route.name==='blog'"/>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!--私密文章密码对话框-->
		<BlogPasswordDialog v-if="blogPasswordDialogVisible"/>

		<!--APlayer-->
		<div v-if="!isMobile" class="m-mobile-hide">
			<button
				v-if="canUsePlaylist && !showPlaylist"
				type="button"
				class="music-loader"
				:title="musicLoading ? '播放器加载中' : '加载音乐播放器'"
				:aria-label="musicLoading ? '播放器加载中' : '加载音乐播放器'"
				:disabled="musicLoading"
				@click="enablePlaylist"
			>
				<i :class="musicLoading ? 'spinner loading icon' : 'music icon'"></i>
			</button>
			<meting-js
				v-if="canUsePlaylist && showPlaylist"
				:server="siteInfo.playlistServer"
				:id="siteInfo.playlistId"
				type="playlist"
				fixed="true"
				theme="#25CCF7"
			></meting-js>
		</div>
		<!--回到顶部-->
		<button type="button" class="backtop-button" title="回到顶部" aria-label="回到顶部" @click="scrollToTop">
			<i class="angle up icon"></i>
		</button>
		<!--底部footer-->
		<Footer :siteInfo="siteInfo" :badges="badges" :newBlogList="newBlogList" :hitokoto="hitokoto"/>
	</div>
</template>

<script>
	import {getHitokoto, getSite} from '@/api/index'
	import { defineAsyncComponent } from 'vue'
	import Nav from "@/components/index/Nav.vue";
	import Footer from "@/components/index/Footer.vue";
	import {mapState} from 'vuex'
	import {SAVE_CLIENT_SIZE, SAVE_INTRODUCTION, SAVE_SITE_INFO, RESTORE_COMMENT_FORM} from "@/store/mutations-types";
	import { updateSeo } from '@/util/seo'
	import { loadMetingPlayer } from '@/util/loadExternalAsset'

	const Header = defineAsyncComponent(() => import("@/components/index/Header.vue"))
	const Introduction = defineAsyncComponent(() => import("@/components/sidebar/Introduction.vue"))
	const Tags = defineAsyncComponent(() => import("@/components/sidebar/Tags.vue"))
	const RandomBlog = defineAsyncComponent(() => import("@/components/sidebar/RandomBlog.vue"))
	const Tocbot = defineAsyncComponent(() => import("@/components/sidebar/Tocbot.vue"))
	const BlogPasswordDialog = defineAsyncComponent(() => import("@/components/index/BlogPasswordDialog.vue"))

	export default {
		name: "Index",
		components: {Header, BlogPasswordDialog, Tocbot, RandomBlog, Tags, Nav, Footer, Introduction},
		data() {
			return {
				siteInfo: {
					blogName: '',
					webTitleSuffix: '',
					playlistServer: '',
					playlistId: ''
				},
				categoryList: [],
				tagList: [],
				randomBlogList: [],
				badges: [],
				newBlogList: [],
				hitokoto: {},
				resizeHandler: null,
				resizeFrame: null,
				musicLoading: false,
				showPlaylist: false,
			}
		},
		computed: {
			...mapState(['focusMode', 'clientSize', 'blogPasswordDialogVisible']),
			isMobile() {
				return this.clientSize.clientWidth <= 767
			},
			canUsePlaylist() {
				return Boolean(this.siteInfo.playlistServer && this.siteInfo.playlistId)
			}
		},
		watch: {
			//路由改变时，页面滚动至顶部
			'$route.path'() {
				this.scrollToTop()
			}
		},
		created() {
			this.saveClientSize()
			this.getSite()
			this.getHitokoto()
			//从localStorage恢复之前的评论信息
			this.$store.commit(RESTORE_COMMENT_FORM)
		},
		mounted() {
			this.resizeHandler = this.scheduleSaveClientSize
			window.addEventListener('resize', this.resizeHandler)
		},
		beforeUnmount() {
			if (this.resizeHandler) {
				window.removeEventListener('resize', this.resizeHandler)
			}
			if (this.resizeFrame) {
				window.cancelAnimationFrame(this.resizeFrame)
				this.resizeFrame = null
			}
		},
		methods: {
			normalizeList(list) {
				return Array.isArray(list) ? list.filter(item => item && typeof item === 'object') : []
			},
			saveClientSize() {
				this.$store.commit(SAVE_CLIENT_SIZE, {clientHeight: window.innerHeight, clientWidth: window.innerWidth})
			},
			scheduleSaveClientSize() {
				if (this.resizeFrame) {
					return
				}
				this.resizeFrame = window.requestAnimationFrame(() => {
					this.resizeFrame = null
					this.saveClientSize()
				})
			},
			getSite() {
				getSite().then(res => {
					if (res.code === 200) {
						this.siteInfo = res.data.siteInfo && typeof res.data.siteInfo === 'object' ? res.data.siteInfo : {}
						this.badges = this.normalizeList(res.data.badges)
						this.newBlogList = this.normalizeList(res.data.newBlogList)
						this.categoryList = this.normalizeList(res.data.categoryList)
						this.tagList = this.normalizeList(res.data.tagList)
						this.randomBlogList = this.normalizeList(res.data.randomBlogList)
						this.$store.commit(SAVE_SITE_INFO, this.siteInfo)
						this.$store.commit(SAVE_INTRODUCTION, res.data.introduction)
						if (this.$route.name !== 'blog') {
							updateSeo({
								title: this.$route.meta.title,
								description: this.siteInfo.siteDescription || '',
								keywords: this.siteInfo.siteKeywords || '',
								path: this.$route.fullPath,
							})
						}
					}
				})
			},
			//获取一言
			getHitokoto() {
				getHitokoto().then(res => {
					this.hitokoto = res
				})
			},
			enablePlaylist() {
				if (this.musicLoading || this.showPlaylist) {
					return
				}
				this.musicLoading = true
				loadMetingPlayer().then(() => {
					this.showPlaylist = true
				}).catch(() => {
					this.msgError('音乐播放器加载失败')
				}).finally(() => {
					this.musicLoading = false
				})
			}
		}
	}
</script>

<style scoped>
	.site {
		display: flex;
		min-height: 100vh; /* 没有元素时，也把页面撑开至100% */
		flex-direction: column;
	}

	.main {
		margin-top: 34px;
		flex: 1;
	}

	.main .ui.container {
		width: min(1440px, calc(100vw - 28px)) !important;
		margin-left: auto !important;
		margin-right: auto !important;
	}

	.ui.grid .three.column {
		padding-top: 0 !important;
	}

	.ui.grid .ten.column {
		padding-top: 0;
	}

	.main :deep(.ui.stackable.grid) {
		align-items: flex-start;
	}

	.m-display-none {
		display: none !important;
	}

	.music-loader {
		position: fixed;
		left: 0;
		bottom: 0;
		z-index: 9999;
		display: inline-flex;
		width: 66px;
		height: 66px;
		align-items: center;
		justify-content: center;
		border: 0;
		border-radius: 0 8px 0 0;
		background: rgba(255, 255, 255, 0.92);
		color: #25CCF7;
		box-shadow: 0 8px 26px rgba(15, 23, 42, 0.14);
		cursor: pointer;
	}

	.music-loader:disabled {
		cursor: default;
	}

	.music-loader i.icon {
		width: auto;
		height: auto;
		margin: 0;
		font-size: 24px;
	}

	.backtop-button {
		position: fixed;
		right: 28px;
		bottom: 30px;
		z-index: 9999;
		display: inline-flex;
		width: 44px;
		height: 44px;
		align-items: center;
		justify-content: center;
		border: 0;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.92);
		color: #0f766e;
		box-shadow: 0 12px 30px rgba(15, 23, 42, 0.16);
		cursor: pointer;
	}

	.backtop-button i.icon {
		width: auto;
		height: auto;
		margin: 0;
		font-size: 20px;
	}

	@media screen and (max-width: 767px) {
		.main {
			margin-top: 56px;
		}

		.m-padded-tb-big {
			padding-top: 0.75rem !important;
			padding-bottom: 2rem !important;
		}

		.main .ui.container {
			width: calc(100vw - 16px) !important;
		}

		.backtop-button {
			right: 16px;
			bottom: 18px;
		}
	}
</style>
