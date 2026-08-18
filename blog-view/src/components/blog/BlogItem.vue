<template>
	<div class="article-list">
		<article class="article-card m-box" :class="{'article-card--no-cover': !item.firstPicture}" v-for="item in blogList" :key="item.id">
			<div class="article-card__cover" v-if="item.firstPicture" :style="{ backgroundImage: 'url(' + item.firstPicture + ')' }">
				<span v-if="item.top" class="article-card__pin">置顶</span>
			</div>
			<div class="article-card__body">
				<div class="article-card__meta">
					<span>{{ dateFormat(item.createTime, 'YYYY-MM-DD')}}</span>
					<span>{{ item.views }} 次浏览</span>
					<span>{{ item.readTime }} 分钟</span>
				</div>
				<h2>
					<a href="javascript:;" @click.prevent="toBlog(item)">{{ item.title }}</a>
				</h2>
				<p class="article-card__summary">{{ getBlogPreview(item) }}</p>
				<div class="article-card__footer">
					<router-link v-if="item.category" :to="`/category/${item.category.name}`" class="article-card__category">
						{{ item.category.name }}
					</router-link>
					<div class="article-card__tags">
						<router-link :to="`/tag/${tag.name}`" v-for="(tag,index) in item.tags" :key="index">{{ tag.name }}</router-link>
					</div>
					<a href="javascript:;" @click.prevent="toBlog(item)" class="color-btn">阅读全文</a>
				</div>
			</div>
		</article>
	</div>
</template>

<script>
	import { createDescription } from '@/util/seo'

	export default {
		name: "BlogItem",
		props: {
			blogList: {
				type: Array,
				required: true
			}
		},
		methods: {
			getBlogPreview(blog) {
				if (!blog || typeof blog !== 'object') {
					return ''
				}
				return createDescription(blog.description || blog.content || '')
			},
			toBlog(blog) {
				this.$store.dispatch('goBlogPage', blog)
			}
		}
	}
</script>

<style scoped>
	.article-list {
		display: flex;
		flex-direction: column;
		gap: 22px;
	}

	.article-card {
		display: grid;
		grid-template-columns: minmax(220px, 34%) minmax(0, 1fr);
		overflow: hidden;
		background: rgba(255, 255, 255, 0.96);
	}

	.article-card__cover {
		position: relative;
		min-height: 270px;
		background-size: cover;
		background-position: center;
	}

	.article-card--no-cover {
		display: block;
	}

	.article-card__cover:after {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(180deg, transparent 35%, rgba(15, 23, 42, 0.32));
	}

	.article-card__pin {
		position: absolute;
		z-index: 1;
		left: 18px;
		top: 18px;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.92);
		color: #b91c1c;
		font-size: 12px;
		font-weight: 800;
		padding: 6px 10px;
	}

	.article-card__body {
		display: flex;
		min-width: 0;
		flex-direction: column;
		padding: 28px;
	}

	.article-card__meta {
		display: flex;
		flex-wrap: wrap;
		gap: 10px 16px;
		color: #64748b;
		font-size: 12px;
		font-weight: 650;
	}

	.article-card h2 {
		margin: 14px 0 12px;
		font-size: 27px;
		font-weight: 750;
		line-height: 1.28;
	}

	.article-card h2 a {
		color: #172033;
	}

	.article-card h2 a:hover {
		color: #0f766e;
	}

	.article-card__summary {
		display: -webkit-box;
		overflow: hidden;
		margin: 0;
		color: #475569;
		font-size: 15px;
		line-height: 1.75;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 3;
	}

	.article-card__footer {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 12px;
		margin-top: auto;
		padding-top: 22px;
	}

	.article-card__category,
	.article-card__tags a {
		border-radius: 999px;
		background: #eef7f6;
		color: #0f766e !important;
		font-size: 12px;
		font-weight: 700;
		padding: 7px 10px;
	}

	.article-card__tags {
		display: flex;
		flex: 1;
		flex-wrap: wrap;
		gap: 8px;
	}

@media (max-width: 768px) {
	.article-card {
		display: block;
	}

	.article-card__cover {
		min-height: 190px;
	}

	.article-card__body {
		padding: 20px;
	}

	.article-card h2 {
		font-size: 22px;
	}
}
</style>
