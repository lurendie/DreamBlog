<template>
	<div>
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

</style>
