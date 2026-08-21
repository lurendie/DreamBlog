<template>
	<!--评论分页-->
	<LightPagination
		class="pagination"
		:current-page="commentQuery.pageNum"
		:page-count="commentTotalPage"
		@current-change="handleCurrentChange"
	/>
</template>

<script>
	import {mapState} from 'vuex'
	import {SET_COMMENT_QUERY_PAGE_NUM, SET_PARENT_COMMENT_ID} from "@/store/mutations-types";
	import LightPagination from "@/components/common/LightPagination.vue";

	export default {
		name: "CommentPagination",
		components: {LightPagination},
		computed: {
			...mapState(['commentQuery', 'commentTotalPage'])
		},
		methods: {
			//监听页码改变的事件
			handleCurrentChange(newPage) {
				this.$store.commit(SET_COMMENT_QUERY_PAGE_NUM, newPage)
				this.$store.commit(SET_PARENT_COMMENT_ID, -1)
				this.$store.dispatch('getCommentList')
			},
		}
	}
</script>

<style scoped>
	.pagination {
		margin-top: 2em;
		text-align: center;
	}

	@media screen and (max-width: 767px) {
		.pagination {
			margin-top: 1.25rem;
		}
	}
</style>
