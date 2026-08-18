<template>
	<div class="blog-feed">
		<div class="feed-heading">
			<span>Latest Notes</span>
			<h1>最近文章</h1>
		</div>
		<!--content-->
		<BlogItem :blogList="blogList"/>
		<!--分页-->
		<BlogPagination :getBlogList="getBlogList" :totalPage="totalPage"/>
	</div>
</template>

<script>
	import BlogItem from "./BlogItem.vue";
	import BlogPagination from "./BlogPagination.vue";

	export default {
		name: "BlogList",
		components: {BlogPagination, BlogItem},
		props: {
			getBlogList: {
				type: Function,
				required: true
			},
			blogList: {
				type: Array,
				required: true
			},
			totalPage: {
				type: Number,
				required: true
			}
		}
	}
</script>

<style scoped>
	.blog-feed {
		display: flex;
		flex-direction: column;
		gap: 18px;
	}

	.feed-heading {
		padding: 4px 2px 0;
	}

	.feed-heading span {
		color: #0f766e;
		font-size: 12px;
		font-weight: 800;
	}

	.feed-heading h1 {
		margin: 6px 0 0;
		color: #172033;
		font-size: 28px;
		font-weight: 750;
		line-height: 1.2;
	}
</style>
