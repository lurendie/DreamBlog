<template>
	<div class="reader-page">
		<article class="reader-card m-box">
			<div class="reader-hero" v-if="blog.firstPicture" :style="{backgroundImage: 'url(' + blog.firstPicture + ')'}">
				<span v-if="blog.top" class="reader-pin">置顶</span>
			</div>
			<header class="reader-header">
				<router-link :to="`/category/${blog.category.name}`" class="reader-category" v-if="blog.category">
					{{ blog.category.name }}
				</router-link>
				<h1>{{ blog.title }}</h1>
				<div class="reader-meta">
					<span>{{ dateFormat(blog.createTime, 'YYYY-MM-DD') }}</span>
					<span>{{ blog.views }} 次浏览</span>
					<span>约 {{ blog.words }} 字</span>
					<span>{{ blog.readTime }} 分钟</span>
					<button type="button" @click.prevent="bigFontSize=!bigFontSize">字体</button>
					<button type="button" @click.prevent="changeFocusMode">专注</button>
				</div>
			</header>

			<div
				class="typo reader-content js-toc-content match-braces rainbow-braces"
				v-lazy-container="{selector: 'img'}"
				v-viewer
				:class="{'m-big-fontsize':bigFontSize}"
				v-safe-html="blog.content"
			></div>

			<div class="reader-reward">
				<el-popover placement="top" width="220" trigger="click" v-if="blog.appreciation">
					<div class="ui orange basic label" style="width: 100%">
						<div class="image">
							<div style="font-size: 12px;text-align: center;margin-bottom: 5px;">一毛是鼓励</div>
							<img :src="$store.state.siteInfo.reward" alt="" class="ui rounded bordered image" style="width: 100%">
							<div style="font-size: 12px;text-align: center;margin-top: 5px;">一块是真爱</div>
						</div>
					</div>
					<template #reference>
						<el-button round>赞赏</el-button>
					</template>
				</el-popover>
			</div>

			<div class="reader-tags" v-if="blog.tags && blog.tags.length">
				<router-link :to="`/tag/${tag.name}`" v-for="(tag,index) in blog.tags" :key="index">{{ tag.name }}</router-link>
			</div>
		</article>

		<!--博客信息-->
		<div class="ui attached positive message reader-info">
			<ul class="list">
				<li>作者：{{ $store.state.introduction.name }}
					<router-link to="/about">（联系作者）</router-link>
				</li>
				<li>发表时间：{{ dateFormat(blog.createTime, 'YYYY-MM-DD HH:mm') }}</li>
				<li>最后修改：{{ dateFormat(blog.updateTime, 'YYYY-MM-DD HH:mm') }}</li>
				<li>本站点采用<a href="https://creativecommons.org/licenses/by/4.0/" target="_blank"> 署名 4.0 国际 (CC BY 4.0) </a>创作共享协议。可自由转载、引用，并且允许商业性使用。但需署名作者且注明文章出处。</li>
			</ul>
		</div>
		<!--评论-->
		<div class="ui bottom teal attached segment threaded comments reader-comments">
			<CommentList :page="0" :blogId="blogId" v-if="blog.commentEnabled"/>
			<h3 class="ui header" v-else>评论已关闭</h3>
		</div>
	</div>
</template>

<script>
	import {getBlogById} from "@/api/blog";
	import CommentList from "@/components/comment/CommentList.vue";
	import {mapState} from "vuex";
	import {SET_FOCUS_MODE, SET_IS_BLOG_RENDER_COMPLETE} from '@/store/mutations-types';
	import { createDescription, updateSeo } from '@/util/seo'
	import {getBlogToken} from '@/util/storage'

	export default {
		name: "Blog",
		components: {CommentList},
		data() {
			return {
				blog: {},
				bigFontSize: false,
			}
		},
		computed: {
			blogId() {
				return parseInt(this.$route.params.id)
			},
			...mapState(['siteInfo', 'focusMode'])
		},
		beforeRouteEnter(to, from, next) {
			//路由到博客文章页面之前，应将文章的渲染完成状态置为 false
			next(vm => {
				// 当 beforeRouteEnter 钩子执行前，组件实例尚未创建
				// vm 就是当前组件的实例，可以在 next 方法中把 vm 当做 this用
				vm.$store.commit(SET_IS_BLOG_RENDER_COMPLETE, false)
			})
		},
		beforeRouteLeave(to, from, next) {
			this.$store.commit(SET_FOCUS_MODE, false)
			// 从文章页面路由到其它页面时，销毁当前组件的同时，要销毁tocbot实例
			// 否则tocbot一直在监听页面滚动事件，而文章页面的锚点已经不存在了，会报"Uncaught TypeError: Cannot read property 'className' of null"
			if (window.tocbot && typeof window.tocbot.destroy === 'function') {
				window.tocbot.destroy()
			}
			next()
		},
		beforeRouteUpdate(to, from, next) {
			// 一般有两种情况会触发这个钩子
			// ①当前文章页面跳转到其它文章页面
			// ②点击目录跳转锚点时，路由hash值会改变，导致当前页面会重新加载，这种情况是不希望出现的
			// 在路由 beforeRouteUpdate 中判断路径是否改变
			// 如果跳转到其它页面，to.path!==from.path 就放行 next()
			// 如果是跳转锚点，path不会改变，hash会改变，to.path===from.path, to.hash!==from.path 不放行路由跳转，就能让锚点正常跳转
			if (to.path !== from.path) {
				this.$store.commit(SET_FOCUS_MODE, false)
				//在当前组件内路由到其它博客文章时，要重新获取文章
				this.getBlog(to.params.id)
				//只要路由路径有改变，且停留在当前Blog组件内，就把文章的渲染完成状态置为 false
				this.$store.commit(SET_IS_BLOG_RENDER_COMPLETE, false)
				next()
			} else {
				next()
			}
		},
		created() {
			this.getBlog()
		},
		methods: {
			getBlog(id = this.blogId) {
				//密码保护的文章，需要发送密码验证通过后保存在localStorage的Token
				//getBlogToken 仅在存储值是真实 token 字符串时才返回；若只存了"已验证"标记则返回 ''
				const blogToken = getBlogToken(id)
				//博主身份由 httpOnly Cookie 自动携带，这里仅发送密码解锁 token
				getBlogById(blogToken, id).then(res => {
					if (res.code === 200) {
						this.blog = res.data
						updateSeo({
							title: this.blog.title,
							description: createDescription(this.blog.description || ''),
							keywords: this.createArticleKeywords(this.blog.tags),
							path: `/blog/${id}`,
							image: this.blog.firstPicture,
							type: 'article',
							author: this.$store.state.introduction.name,
							publishedTime: this.blog.createTime,
							modifiedTime: this.blog.updateTime,
						})
							//富文本渲染完毕后，渲染代码块样式
						this.$nextTick(() => {
							Prism.highlightAll()
							//将文章渲染完成状态置为 true
							this.$store.commit(SET_IS_BLOG_RENDER_COMPLETE, true)
						})
					} else {
						this.msgError(res.msg)
					}
				}).catch(() => {
					this.msgError("请求失败")
				})
			},
			changeFocusMode() {
				this.$store.commit(SET_FOCUS_MODE, !this.focusMode)
			},
			createArticleKeywords(tags = []) {
				if (!Array.isArray(tags)) {
					return ''
				}
				return tags.map(tag => tag.name).filter(Boolean).join(',')
			}
		}
	}
</script>

<style scoped>
	.el-divider {
		margin: 1rem 0 !important;
	}

	.reader-page {
		max-width: 920px;
		margin: 0 auto;
	}

	.reader-card {
		overflow: hidden;
		background: rgba(255, 255, 255, 0.98);
	}

	.reader-hero {
		position: relative;
		min-height: 360px;
		background-size: cover;
		background-position: center;
	}

	.reader-hero:after {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(180deg, transparent 45%, rgba(15, 23, 42, 0.30));
	}

	.reader-pin {
		position: absolute;
		z-index: 1;
		top: 22px;
		left: 22px;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.92);
		color: #b91c1c;
		font-size: 12px;
		font-weight: 800;
		padding: 7px 12px;
	}

	.reader-header {
		padding: 34px 44px 18px;
		text-align: left;
	}

	.reader-category {
		display: inline-flex;
		border-radius: 999px;
		background: #eef7f6;
		color: #0f766e !important;
		font-size: 12px;
		font-weight: 800;
		padding: 7px 11px;
	}

	.reader-header h1 {
		margin: 16px 0 14px;
		color: #172033;
		font-size: clamp(30px, 5vw, 48px);
		font-weight: 760;
		line-height: 1.18;
	}

	.reader-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 10px 16px;
		color: #64748b;
		font-size: 13px;
	}

	.reader-meta button {
		border: 0;
		border-radius: 999px;
		background: #f1f5f9;
		color: #334155;
		font-size: 12px;
		font-weight: 700;
		padding: 3px 10px;
		cursor: pointer;
	}

	.reader-content {
		padding: 12px 44px 26px !important;
		color: #1f2937;
		font-size: 16px;
		line-height: 1.9;
	}

	.reader-content :deep(img) {
		border-radius: 8px;
	}

	.reader-reward {
		display: flex;
		justify-content: center;
		padding: 0 44px 22px;
	}

	.reader-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		border-top: 1px solid rgba(148, 163, 184, 0.18);
		padding: 22px 44px 30px;
	}

	.reader-tags a {
		border-radius: 999px;
		background: #f1f5f9;
		color: #475569 !important;
		font-size: 12px;
		font-weight: 700;
		padding: 7px 10px;
	}

	.reader-info,
	.reader-comments {
		margin-top: 16px !important;
		border-radius: 10px !important;
		box-shadow: 0 18px 45px rgba(15, 23, 42, 0.08) !important;
	}

	@media screen and (max-width: 767px) {
		.reader-page {
			max-width: none;
		}

		.reader-hero {
			min-height: 220px;
		}

		.reader-header,
		.reader-content,
		.reader-reward,
		.reader-tags {
			padding-left: 20px !important;
			padding-right: 20px !important;
		}

		.ui.attached.positive.message,
		.ui.bottom.teal.attached.segment.threaded.comments {
			padding-left: 1rem !important;
			padding-right: 1rem !important;
			font-size: 14px;
		}
	}

	h1::before, h2::before, h3::before, h4::before, h5::before, h6::before {
		display: block;
		content: " ";
		height: 55px;
		margin-top: -55px;
		visibility: hidden;
	}
</style>
