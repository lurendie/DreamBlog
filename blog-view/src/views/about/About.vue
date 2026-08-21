<template>
	<div>
		<div class="ui top attached segment m-padded-lr-big">
			<h2 class="m-text-500" style="text-align: center">{{ about.title }}</h2>
			<meting-js server="netease" type="song" :id="about.musicId" theme="#25CCF7" v-if="about.musicId!=='' && playerReady"></meting-js>
		<div class="typo content m-margin-top-large" v-lazy-container="{selector: 'img'}" v-viewer v-safe-html="about.content"></div>
		</div>
		<!--评论-->
		<div class="ui bottom teal attached segment threaded comments">
			<CommentList :page="1" :blogId="null" v-if="about.commentEnabled==='true'"/>
			<h3 class="ui header" v-else>评论已关闭</h3>
		</div>
	</div>
</template>

<script>
	import {getAbout} from "@/api/about";
	import CommentList from "@/components/comment/CommentList.vue";
	import { directive as viewerDirective } from 'v-viewer'
	import 'viewerjs/dist/viewer.css'
	import { updateSeo } from '@/util/seo'
	import { loadMetingPlayer } from '@/util/loadExternalAsset'

	export default {
		name: "About",
		components: {CommentList},
		directives: {
			viewer: viewerDirective()
		},
		data() {
			return {
				about: {
					title: '',
					musicId: '',
					content: '',
					commentEnabled: 'false'
				},
				playerReady: false
			}
		},
		created() {
			this.getData()
		},
		methods: {
			getData() {
				getAbout().then(res => {
					if (res.code === 200) {
						this.about = res.data
						updateSeo({
							title: this.about.title || '关于我',
							description: this.$store.state.siteInfo?.siteDescription || '',
							keywords: this.$store.state.siteInfo?.siteKeywords || '',
							path: this.$route.fullPath,
						})
						if (this.about.musicId) {
							this.loadPlayer()
						}
					} else {
						this.msgError(res.msg)
					}
				}).catch(() => {
					this.msgError("请求失败")
				})
			},
			loadPlayer() {
				loadMetingPlayer().then(() => {
					this.playerReady = true
				}).catch(() => {
					this.msgError('音乐播放器加载失败')
				})
			}
		}
	}
</script>

<style>
	.content ul li {
		letter-spacing: 1px !important;
	}
</style>
