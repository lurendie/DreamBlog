<template>
	<div class="site">
		<!--顶部导航-->
		<Nav :blogName="siteInfo.blogName" :categoryList="categoryList"/>
		<!--首页大图 只在首页且pc端时显示-->
		<div class="m-mobile-hide">
			<Header v-if="$route.name==='home'"/>
		</div>

		<div class="main">
			<div class="m-padded-tb-big">
				<div class="ui container">
					<div class="ui stackable grid">
						<!--左侧-->
						<div class="three wide column m-mobile-hide">
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
						<div class="three wide column m-mobile-hide">
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
		<BlogPasswordDialog/>

		<!--APlayer-->
		<div class="m-mobile-hide">
			<meting-js :server="siteInfo.playlistServer" :id="siteInfo.playlistId" type="playlist" fixed="true" theme="#25CCF7" v-if="siteInfo.playlistServer && siteInfo.playlistId"></meting-js>
		</div>
		<!--回到顶部-->
		<el-backtop style="box-shadow: none;background: none;z-index: 9999;">
			<img src="/img/paper-plane.png" style="width: 40px;height: 40px;">
		</el-backtop>
		<!--底部footer-->
		<Footer :siteInfo="siteInfo" :badges="badges" :newBlogList="newBlogList" :hitokoto="hitokoto"/>
	</div>
</template>

<script>
	import {getHitokoto, getSite} from '@/api/index'
	import Nav from "@/components/index/Nav.vue";
	import Header from "@/components/index/Header.vue";
	import Footer from "@/components/index/Footer.vue";
	import Introduction from "@/components/sidebar/Introduction.vue";
	import Tags from "@/components/sidebar/Tags.vue";
	import RandomBlog from "@/components/sidebar/RandomBlog.vue";
	import Tocbot from "@/components/sidebar/Tocbot.vue";
	import BlogPasswordDialog from "@/components/index/BlogPasswordDialog.vue";
	import {mapState} from 'vuex'
	import {SAVE_CLIENT_SIZE, SAVE_INTRODUCTION, SAVE_SITE_INFO, RESTORE_COMMENT_FORM} from "@/store/mutations-types";
	import { updateSeo } from '@/util/seo'

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
			}
		},
		computed: {
			...mapState(['focusMode'])
		},
		watch: {
			//路由改变时，页面滚动至顶部
			'$route.path'() {
				this.scrollToTop()
			}
		},
		created() {
			this.getSite()
			this.getHitokoto()
			//从localStorage恢复之前的评论信息
			this.$store.commit(RESTORE_COMMENT_FORM)
		},
		mounted() {
			//保存可视窗口大小
			this.saveClientSize()
			this.resizeHandler = this.saveClientSize
			window.addEventListener('resize', this.resizeHandler)
		},
		beforeUnmount() {
			if (this.resizeHandler) {
				window.removeEventListener('resize', this.resizeHandler)
			}
		},
		methods: {
			normalizeList(list) {
				return Array.isArray(list) ? list.filter(item => item && typeof item === 'object') : []
			},
			saveClientSize() {
				this.$store.commit(SAVE_CLIENT_SIZE, {clientHeight: document.body.clientHeight, clientWidth: document.body.clientWidth})
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
		margin-top: 40px;
		flex: 1;
	}

	.main .ui.container {
		width: min(1400px, calc(100vw - 24px)) !important;
		margin-left: auto !important;
		margin-right: auto !important;
	}

	.ui.grid .three.column {
		padding: 0;
	}

	.ui.grid .ten.column {
		padding-top: 0;
	}

	.m-display-none {
		display: none !important;
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
	}
</style>
